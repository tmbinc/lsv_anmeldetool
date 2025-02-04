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
  } from "../../../../../api/api";
  import { page } from "$app/state";
  import {
    Button,
    ButtonGroup,
    Checkbox,
    Group,
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
  let org_id = page.params.org;
  let event_id = page.params.event;
  let login_token = page.params.token;
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
        }
      }
    );

    loading = !all_good;
  });

  async function new_team() {
    let res = await createTeam({
      event: event_id,
      org: org_id,
      name: "Team " + (teams.length + 1).toString(),
    }).result;
    if (res.ok) {
      teams.push(res.data);
    }
  }

  function update_team(id: string) {
    let team_to_update = teams.find((team) => team.id == id);
    if (team_to_update) {
      updateTeam(team_to_update);
      teams_changed = teams_changed.filter((item) => item != id);
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
    if (org_changed && org) {
      const resp = await updateOrg({ ...org, org: org.id }).result;
      if (resp.ok) {
        org_changed = false;
      }
    }
  }

  beforeNavigate(({ cancel }) => {
    if (teams_changed.length != 0 || org_changed) {
      if (!confirm("Achtung, ungespeicherte Daten. Wirklich schließen?")) {
        cancel();
      }
    }
  });
</script>

<main>
  <FetchErrors admin={false} bind:this={fetch_error} />
  {#if login_error}
    <LoadError
      text="Ungültiger Link! Bitte folgen Sie dem Link aus der Registrierungsbestätigung."
    />
  {:else if loading}
    <Loading />
  {:else}
    <h1 class="text-3xl font-bold underline">
      Wilkommen, {org?.name}!
    </h1>

    {#if org}
      <div>
        <div>
          <Label for="org_name"
            >Name der Schule, wie er auf den Urkunden gedruckt werden soll -
            bitte auch den Artikel korrekt einstellen!</Label
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
          <Label for="org_contact_name"
            >Kontaktperson für Organisatorisches (muss nicht am Turniertag
            anwesend sein):</Label
          >
          <Input
            class="w-100"
            id="org_contact_name"
            type="text"
            on:input={() => (org_changed = true)}
            bind:value={org.contact_name}
            placeholder="Name"
            required
          />
        </div>
        <div>
          <Label for="org_contact_email">Email-Adresse:</Label>

          <Input
            class="w-100"
            id="org_contact_email"
            type="text"
            on:input={() => (org_changed = true)}
            bind:value={org.contact_email}
            placeholder="Email-Addresse"
            required
          />
        </div>
        <div>
          <Label for="org_contact_phone">Telefon:</Label>

          <Input
            class="w-100"
            id="org_contact_phone"
            type="text"
            on:input={() => (org_changed = true)}
            bind:value={org.contact_phone}
            placeholder="Telefon"
            required
          />
        </div>
      </div>

      <Button color="green" disabled={!org_changed} on:click={update_org}
        >{#if org_changed}Änderungen speichern{:else}Daten gespeichert.{/if}</Button
      >
    {/if}

    <Questionnaire {event_id} {org_id} />

    <p class="text-lg font-medium">
      Bitte melden Sie die Mannschaften für das Turnier {reg_event?.name}.
    </p>

    <Table>
      <TableHead class="flex flex-col md:flex-row  mb-4">
        <TableHeadCell>Team-Name</TableHeadCell>
        <TableHeadCell>Altersgruppe</TableHeadCell>
        <TableHeadCell>Ansprechpartner (Name)</TableHeadCell>
        <TableHeadCell>Telefon</TableHeadCell>
        <TableHeadCell>Bearbeiten</TableHeadCell>
      </TableHead>
      <TableBody>
        {#if teams.length == 0}
          <TableBodyRow class="flex flex-col md:flex-row  mb-4">
            <TableBodyCell>Bitte mindestens ein Team hinzufügen!</TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell></TableBodyCell>
          </TableBodyRow>
        {/if}
        {#each teams as team}
          <TableBodyRow class="flex flex-col md:flex-row  mb-4">
            <TableBodyCell
              ><Input
                disabled={finalized}
                bind:value={team.name}
                oninput={() => change_team(team.id)}
              /></TableBodyCell
            >
            <TableBodyCell>
              <Select
                class="mt-2"
                items={groups}
                disabled={finalized}
                oninput={() => change_team(team.id)}
                bind:value={team.group_id}
              />
            </TableBodyCell>
            <TableBodyCell
              ><Input
                bind:value={team.contact_name}
                oninput={() => change_team(team.id)}
              /></TableBodyCell
            >
            <TableBodyCell
              ><Input
                bind:value={team.contact_phone}
                oninput={() => change_team(team.id)}
              /></TableBodyCell
            >
            <TableBodyCell>
              {#if org_event_state != "Verified"}
                <ButtonGroup>
                  <Button
                    on:click={() => update_team(team.id)}
                    color="green"
                    disabled={!teams_changed.includes(team.id)}
                    >{#if teams_changed.includes(team.id)}Speichern?{:else}Gespeichert.{/if}</Button
                  >
                  <Button
                    color="yellow"
                    disabled={!teams_changed.includes(team.id)}
                    on:click={() => revert_team(team.id)}>Rückgängig</Button
                  >
                  <Button
                    color="red"
                    disabled={finalized}
                    on:click={() => {
                      team_to_delete = team.id;
                      team_delete_modal = true;
                    }}>Team löschen</Button
                  >
                </ButtonGroup>
              {:else}
                <Checkbox
                  bind:checked={team.present}
                  oninput={() => change_team(team.id)}
                  >Anwesenheit erklärt</Checkbox
                >
                <Button
                  on:click={() => update_team(team.id)}
                  color="green"
                  disabled={!teams_changed.includes(team.id)}>Speichern</Button
                >
              {/if}
            </TableBodyCell>
          </TableBodyRow>
        {/each}
        <TableBodyRow class="flex flex-col md:flex-row  mb-4">
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
          <TableBodyCell></TableBodyCell>
          <TableBodyCell>
            {#if org_event_state == "Invited" || org_event_state == "Registered" || org_event_state == "Updated"}
              <Button
                color="red"
                on:click={() => (submit_modal = true)}
                disabled={teams.length == 0 || teams_changed.length != 0}
                >Anmeldung finalisieren</Button
              >
            {/if}
            {#if org_event_state == "Submitted" || org_event_state == "Verified"}
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
          class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
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

    <Modal bind:open={submit_modal} size="xs" autoclose>
      <div class="text-center">
        <ExclamationCircleOutline
          class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
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
