<script lang="ts">
  import { onMount } from "svelte";
  import {
    getEvent,
    getOrg,
    type Org,
    type Event,
    getTeamsForOrgEvent,
    type Team,
    createTeam,
    getGroupsForEvent,
    getTeam,
    updateTeam,
    deleteTeam,
    setEventOrgState,
    getOrgEventStatus,
    type EventOrgState,
    updateOrg,
    authOrgLogin,
    getTeamCountForEvent,
  } from "../../../../../api/api";
  import { page } from "$app/state";
  import {
    Alert,
    Button,
    ButtonGroup,
    Card,
    Checkbox,
    FloatingLabelInput,
    Group,
    Helper,
    Input,
    Label,
    Modal,
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    type SelectOptionType,
  } from "flowbite-svelte";
  import { ExclamationCircleOutline } from "flowbite-svelte-icons";
  import FetchErrors from "../../../../FetchErrors.svelte";
  import Loading from "../../../../Loading.svelte";
  import { beforeNavigate } from "$app/navigation";
  import type { BeforeNavigate } from "@sveltejs/kit";
  import LoadError from "../../../../LoadError.svelte";
  import Questionnaire from "./Questionnaire.svelte";

  let org: Org | null = $state(null);
  let reg_event: Event | null = $state(null);
  let teams: Team[] = $state([]);
  let org_id = page.params.org || "";
  let event_id = page.params.event || "";
  let login_token = page.params.token || "";
  let groups: SelectOptionType<string>[] = $state([]);
  let teams_changed: string[] = $state([]);
  let team_to_delete: string | null = $state(null);
  let team_delete_modal = $state(false);
  let fetch_error: FetchErrors;
  let login_error = $state(false);
  let loading = $state(true);
  let submit_modal = $state(false);
  let org_changed = $state(false);
  let org_event_state: EventOrgState = $state("NotEnlisted");
  let finalized = $state(false);
  let questionnaire_changes = $state(false);
  let data_missing_modal = $state(false);
  let team_count = $state(0);

  const state_description = {
    NotEnlisted: "Not Enlisted",
    Registered: "Registriert",
    Invited: "Neu",
    Updated: "In Bearbeitung",
    Submitted: "Finalisiert, warte auf Bestätigung des Ausrichters",
    Verified: "Bestätigt! Viel Spaß am Turniertag!",
  };

  const genera: SelectOptionType<string>[] = [
    { name: "Der", value: "m" },
    { name: "Die", value: "f" },
    { name: "Das", value: "n" },
  ];

  onMount(async () => {
    let all_good = true;

    if (login_token) {
      let resp = await authOrgLogin({ org: org_id, token: login_token }).result;
      if (resp.ok) {
      } else {
        all_good = false;
        login_error = true;
      }
    }

    const request = getOrg({ org: org_id });
    const resp = await request.result;
    if (resp.ok) {
      org = resp.data;
      org.contact_email = org.contact_email || "";
      org.contact_name = org.contact_name || "";
      org.contact_phone = org.contact_phone || "";
    } else {
      fetch_error.check(resp);
      all_good = false;
    }
    const event_request = getEvent({ event: event_id });
    const event_resp = await event_request.result;
    if (event_resp.ok) {
      reg_event = event_resp.data;
    } else {
      fetch_error.check(event_resp);
      all_good = false;
    }

    const teams_request = getTeamsForOrgEvent({ org: org_id, event: event_id });
    const teams_resp = await teams_request.result;
    if (teams_resp.ok) {
      teams = teams_resp.data;
      // To fix null not being handeable in <FloatingInput>
      teams.forEach((x) => {
        x.contact_name = x.contact_name || "";
        x.contact_phone = x.contact_phone || "";
      });
    } else {
      fetch_error.check(teams_resp);
      all_good = false;
    }

    const resp_groups = await getGroupsForEvent({ event: event_id }).result;
    if (resp_groups.ok) {
      groups = resp_groups.data.map((group) => ({
        name: group.name,
        value: group.id,
      }));
    } else {
      fetch_error.check(resp_groups);
      all_good = false;
    }

    getOrgEventStatus({ event: event_id, org: org_id }).resp.subscribe(
      (resp) => {
        if (resp?.ok) {
          org_event_state = resp.data;
          finalized =
            org_event_state == "Submitted" || org_event_state == "Verified";

          // On first login, set team status to "updated"
          if (org_event_state == "Invited" && login_token) {
            unsubmit_teams();
          }
        }
      },
    );

    getTeamCountForEvent({ event: event_id }).resp.subscribe((resp) => {
      if (resp?.ok) {
        team_count = resp.data;
      }
    });
    loading = !all_good;
  });

  async function new_team() {
    let res = await createTeam({
      event: event_id,
      org: org_id,
      name: "Team " + (teams.length + 1).toString(),
    }).result;
    if (res.ok) {
      res.data.contact_name = "";
      res.data.contact_phone = "";
      teams.push(res.data);
    }
  }

  function update_team(id: string) {
    let team_to_update = teams.find((team) => team.id == id);
    if (team_to_update) {
      let validated_team =
        team_to_update.contact_name?.trim() &&
        team_to_update.contact_phone?.trim();

      if (!validated_team) {
        data_missing_modal = true;
      } else {
        updateTeam(team_to_update);
        teams_changed = teams_changed.filter((item) => item != id);
      }
    }
  }

  async function revert_team(id: string) {
    let i = teams.findIndex((team) => team.id == id);
    if (i != -1) {
      let resp = await getTeam({ team: id }).result;
      if (resp.ok) {
        teams[i] = resp.data;
        teams_changed = teams_changed.filter((item) => item != id);
      }
    }
  }

  function change_team(id: string) {
    if (!teams_changed.includes(id)) {
      teams_changed.push(id);
    }
  }

  async function delete_team(id: string | null) {
    if (id) {
      const resp = await deleteTeam({ team: id }).result;
      if (resp.ok) {
        teams_changed = teams_changed.filter((item) => item != id);
        teams = teams.filter((team) => team.id != id);
      }
    }
  }

  async function submit_teams() {
    org_event_state = "Submitted";
    setEventOrgState({ event: event_id, org: org_id, state: org_event_state });
    finalized = true;
  }

  async function unsubmit_teams() {
    org_event_state = "Updated";
    setEventOrgState({ event: event_id, org: org_id, state: org_event_state });
    finalized = false;
  }

  async function update_org() {
    let validate_ok =
      org?.contact_email?.trim() &&
      org?.contact_name?.trim() &&
      org?.contact_phone?.trim();

    if (!validate_ok) {
      data_missing_modal = true;
    } else {
      if (org_changed && org) {
        const resp = await updateOrg({ ...org, org: org.id }).result;
        if (resp.ok) {
          org_changed = false;
        }
      }
    }
  }

  beforeNavigate(({ cancel }) => {
    if (teams_changed.length != 0 || org_changed || questionnaire_changes) {
      if (!confirm("Achtung, ungespeicherte Daten. Wirklich schließen?")) {
        cancel();
      }
    }
  });
</script>

<main class="px-2 md:px-10 pt-6 flex flex-col gap-4">
  <FetchErrors admin={false} bind:this={fetch_error} />
  {#if login_error}
    <LoadError
      text="Ungültiger Link! Bitte folgen Sie dem Link aus der Registrierungsbestätigung."
    />
  {:else if loading}
    <Loading />
  {:else}
    <h1 class="text-3xl font-bold underline">
      Willkommen, {org?.name}!
    </h1>

    <p class="my-4 text-xl text-gray-500">
      &gt;&gt; Allgemeine Turnierinformation
    </p>

    <div class="w-full gap-4">
      {@html reg_event?.description}
    </div>

    <p class="my-4 text-xl text-gray-500">&gt;&gt; Allgemeines</p>

    {#if org}
      <div>
        <div>
          <Label for="org_name"
            >Name der Schule, wie er auf den Urkunden gedruckt werden soll:</Label
          >
          <Select
            class="inline w-32"
            items={genera}
            onchange={() => (org_changed = true)}
            bind:value={org.genus}
          />
          <Input
            class="inline w-8/12"
            id="org_name"
            type="text"
            on:input={() => (org_changed = true)}
            bind:value={org.name}
            placeholder="Schulname"
            required
          />
        </div>

        <div>
          <Label for="org_add_name"
            >Ort: (Für uns zur Unterscheidung; wird nicht für die Urkunde
            verwendet.)</Label
          >

          <Input
            class="w-100"
            id="org_add_name"
            type="text"
            on:input={() => (org_changed = true)}
            bind:value={org.name_additional}
            placeholder="Schulort"
            required
          />
        </div>

        <div>
          Kontaktperson für Organisatorisches (muss nicht am Turniertag anwesend
          sein):
        </div>
        <div
          id="exampleWrapper"
          class="grid mt-5 gap-6 items-end w-full md:grid-cols-3"
        >
          {#if org.contact_name !== null}
            <div>
              <FloatingLabelInput
                style="outlined"
                type="text"
                on:input={() => (org_changed = true)}
                bind:value={org.contact_name}>Name</FloatingLabelInput
              >
              <Helper color="red">
                <span class="font-medium">
                  &nbsp; {#if !org.contact_name?.trim()}Bitte einen Kontaktnamen
                    angeben!{/if}
                </span>
              </Helper>
            </div>
          {/if}

          {#if org.contact_email !== null}
            <div>
              <FloatingLabelInput
                style="outlined"
                type="email"
                on:input={() => (org_changed = true)}
                bind:value={org.contact_email}>Email</FloatingLabelInput
              >
              <Helper color="red">
                <span class="font-medium">
                  &nbsp; {#if !org.contact_email?.trim()}Bitte eine
                    Email-Adresse angeben!{/if}
                </span>
              </Helper>
            </div>
          {/if}

          {#if org.contact_phone !== null}
            <div>
              <FloatingLabelInput
                style="outlined"
                type="tel"
                on:input={() => (org_changed = true)}
                on:change={() => {
                  if (org?.contact_phone == "110") {
                    // This is a hack to allow an admin user to do changes. Non-admin
                    // users still can't change events when user changes are disabled.
                    finalized = false;
                  }
                }}
                bind:value={org.contact_phone}>Telefon</FloatingLabelInput
              >
              <Helper color="red">
                <span class="font-medium">
                  &nbsp; {#if !org.contact_phone?.trim()}Bitte eine
                    Telefonnummer angeben!{/if}
                </span>
              </Helper>
            </div>
          {/if}
        </div>
      </div>

      <Button color="green" disabled={!org_changed} on:click={update_org}
        >{#if org_changed}Änderungen speichern{:else}Daten gespeichert.{/if}</Button
      >
    {/if}

    <Questionnaire
      {event_id}
      {org_id}
      bind:active_changes={questionnaire_changes}
      allow_user_changes={reg_event?.allow_user_changes}
    />

    <p class="my-4 text-xl text-gray-500">&gt;&gt; Mannschaften</p>
    {#if reg_event?.allow_user_changes}
      <p class="text-xl font-medium">
        Bitte melden Sie die Mannschaften für das Turnier {reg_event?.name}.
      </p>
    {:else}
      <Alert
        >Es können keine weiteren Änderungen an den Mannschaften mehr
        vorgenommen werden.
      </Alert>
    {/if}
    <p>
      Es sind aktuell {team_count} Mannschaften gemeldet.
      {#if reg_event?.public_reg_until}Meldeschluss ist {new Date(
          reg_event?.public_reg_until || 0,
        ).toLocaleString()}.{/if}
    </p>

    <Card class="p-4 sm:p-6 md:p-8">
      <h5
        class="mb-2 text-2xl font-semibold tracking-tight text-gray-900 dark:text-white"
      >
        Team-Namen?
      </h5>
      <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">
        Bei den Team-Namen dürfen die Mannschaften gerne kreativ sein: Auf der
        Urkunde und während des Turniers ist der Schulname ebenfalls
        ersichtlich, und muss daher nicht explizit im Mannschaftsnamen enthalten
        sein.
      </p>
      <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">
        Ein einfaches "Mannschaft 3" oder "Klasse 5a" ist natürlich auch in
        Ordnung.
      </p>
      <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">
        Beispiel-Urkundentext:
      </p>
      <div class="text-white-600 bg-slate-300 items-center">
        Die Mannschaft <br />
        <p class="font-bold">
          {#if teams.length == 0}Die Drei Könige{:else}
            {teams[0].name}{/if}
        </p>
        <p>
          {"" + { f: "der", m: "des", n: "des" }[org?.genus || ""]}
        </p>
        <p class="font-bold">
          {org?.name}
        </p>

        <p>
          erreichte im Turnier der Altersklassen xyz mit xx von yy Punkten den
          zz. Platz.
        </p>
      </div>
    </Card>

    <p class="text-lg font-medium">
      Bei "Teamleitung" bitte eine Person eintragen, die am Turniertag vor Ort
      und telefonisch erreichbar ist!
    </p>

    <Table class="table-fixed" shadow>
      <TableHead class="flex-col lg:flex-row lg:block hidden">
        <TableHeadCell>Team-Name</TableHeadCell>
        <TableHeadCell>Altersgruppe</TableHeadCell>
        <TableHeadCell>Teamleitung (Name)</TableHeadCell>
        <TableHeadCell>Bearbeiten</TableHeadCell>
      </TableHead>
      <TableBody>
        {#if teams.length == 0}
          <TableBodyRow class="flex flex-col lg:flex-row">
            <TableBodyCell>Bitte mindestens ein Team hinzufügen!</TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell></TableBodyCell>
          </TableBodyRow>
        {/if}
        {#each teams as team}
          <TableBodyRow class="flex flex-col lg:flex-row">
            <TableBodyCell
              ><Label class="block mb-3 lg:hidden">Team-Name</Label>
              <FloatingLabelInput
                style="outlined"
                type="text"
                disabled={finalized}
                oninput={() => change_team(team.id)}
                bind:value={team.name}>Team-Name</FloatingLabelInput
              >
            </TableBodyCell>
            <TableBodyCell>
              <Label class="block mb-3 lg:hidden">Altersgruppe</Label>
              <Select
                class="mt-2"
                items={groups}
                disabled={finalized}
                oninput={() => change_team(team.id)}
                bind:value={team.group_id}
              />
            </TableBodyCell>
            <TableBodyCell
              ><Label class="block mb-3 lg:hidden">Teamleitung</Label>

              {#if team.contact_name !== null}
                <FloatingLabelInput
                  style="outlined"
                  type="text"
                  disabled={finalized}
                  oninput={() => change_team(team.id)}
                  bind:value={team.contact_name}>Name</FloatingLabelInput
                >
                <Helper color="red">
                  <span class="font-medium">
                    &nbsp; {#if !team.contact_name?.trim()}Bitte eine
                      Kontaktperson angeben!{/if}
                  </span>
                </Helper>
              {/if}
              {#if team.contact_phone !== null}
                <FloatingLabelInput
                  style="outlined"
                  type="text"
                  disabled={finalized}
                  oninput={() => change_team(team.id)}
                  bind:value={team.contact_phone}>Telefon</FloatingLabelInput
                >
                <Helper color="red">
                  <span class="font-medium">
                    &nbsp; {#if !team.contact_phone?.trim()}Bitte eine
                      Telefonnummer angeben!{/if}
                  </span>
                </Helper>
              {/if}
            </TableBodyCell>
            <TableBodyCell>
              {#if org_event_state != "Verified"}
                <ButtonGroup>
                  <Button
                    on:click={() => update_team(team.id)}
                    color="green"
                    disabled={!teams_changed.includes(team.id)}
                    >{#if teams_changed.includes(team.id)}Speichern{:else}Gespeichert.{/if}</Button
                  >
                  <Button
                    color="yellow"
                    disabled={!teams_changed.includes(team.id)}
                    on:click={() => revert_team(team.id)}>Rückgängig</Button
                  >
                  <Button
                    color="red"
                    disabled={finalized || !reg_event?.allow_user_changes}
                    on:click={() => {
                      team_to_delete = team.id;
                      team_delete_modal = true;
                    }}>Team löschen</Button
                  >
                </ButtonGroup>
              {:else}
                {#if reg_event?.allow_set_present}
                  <Checkbox
                    checked={team.presence_state == "present"}
                    oninput={() => {
                      if (team.presence_state == "present") {
                        team.presence_state = "";
                      } else {
                        team.presence_state = "present";
                      }
                      change_team(team.id);
                    }}>Anwesenheit erklärt</Checkbox
                  >
                {/if}
                <Button
                  on:click={() => update_team(team.id)}
                  color="green"
                  disabled={!teams_changed.includes(team.id)}>Speichern</Button
                >
              {/if}
            </TableBodyCell>
          </TableBodyRow>
        {/each}
        <TableBodyRow class="flex flex-col lg:flex-row ">
          <TableBodyCell>
            <Button
              disabled={finalized}
              on:click={() => new_team()}
              color="green">Team hinzufügen...</Button
            >
          </TableBodyCell>
          <TableBodyCell
            >Status: {state_description[org_event_state]}</TableBodyCell
          >
          <TableBodyCell></TableBodyCell>
          <TableBodyCell>
            {#if org_event_state == "Invited" || org_event_state == "Registered" || org_event_state == "Updated"}
              <Button
                color="red"
                on:click={() => (submit_modal = true)}
                disabled={teams.length == 0 ||
                  teams_changed.length != 0 ||
                  questionnaire_changes}>Anmeldung finalisieren</Button
              >
            {/if}
            {#if (org_event_state == "Submitted" || org_event_state == "Verified") && reg_event?.allow_user_changes}
              <Button color="red" on:click={() => (submit_modal = true)}
                >Anmeldung bearbeiten</Button
              >
            {/if}
          </TableBodyCell>
        </TableBodyRow>
      </TableBody>
    </Table>

    <Modal bind:open={team_delete_modal} size="xs" autoclose>
      <div class="text-center">
        <ExclamationCircleOutline
          class="mx-auto text-gray-400 w-12 h-12 dark:text-gray-200"
        />
        <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
          Soll dieses Team wirklich gelöscht werden?
        </h3>
        <Button
          color="red"
          class="me-2"
          on:click={() => delete_team(team_to_delete)}>Ja</Button
        >
        <Button color="alternative">Nein (Abbruch)</Button>
      </div>
    </Modal>

    <Modal bind:open={data_missing_modal} size="xs" autoclose>
      <div class="text-center">
        <ExclamationCircleOutline
          class="mx-auto text-red-400 w-12 h-12 dark:text-gray-200"
        />
        <h3 class="mb-5 text-lg font-normal text-red-500 dark:text-gray-400">
          Bitte die fehlenden Daten eingeben!
        </h3>
        <Button color="alternative">Ok</Button>
      </div>
    </Modal>

    <Modal bind:open={submit_modal} size="xs" autoclose>
      <div class="text-center">
        <ExclamationCircleOutline
          class="mx-auto text-gray-400 w-12 h-12 dark:text-gray-200"
        />
        {#if org_event_state == "Invited" || org_event_state == "Registered" || org_event_state == "Updated"}
          <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
            Anmeldungen absenden?
          </h3>
          <Button color="red" class="me-2" on:click={() => submit_teams()}
            >Ja</Button
          >
        {/if}
        {#if org_event_state == "Verified" || org_event_state == "Submitted"}
          <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
            Anmeldung wieder bearbeiten?
          </h3>
          <Button color="red" class="me-2" on:click={() => unsubmit_teams()}
            >Ja</Button
          >
        {/if}
        <Button color="alternative">Nein (Abbruch)</Button>
      </div>
    </Modal>
  {/if}
</main>
