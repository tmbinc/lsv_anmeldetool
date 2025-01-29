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
  } from "../../../../api/api";
  import { page } from "$app/state";
  import {
    Button,
    ButtonGroup,
    Group,
    Input,
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
  import FetchErrors from "../../../FetchErrors.svelte";
  import Loading from "../../../Loading.svelte";

  let org: Org | null = $state(null);
  let event: Event | null = $state(null);
  let teams: Team[] = $state([]);
  let org_id = page.params.org;
  let event_id = page.params.event;
  let groups: SelectOptionType<string>[] = $state([]);
  let teams_changed: string[] = $state([]);
  let team_to_delete: string | null = $state(null);
  let team_delete_modal = $state(false);
  let fetch_error: FetchErrors;
  let loading = $state(true);
  let submit_modal = $state(false);

  onMount(async () => {
    const request = getOrg({ org: org_id });
    const resp = await request.result;
    let all_good = true;
    if (resp.ok) {
      org = resp.data;
    } else {
      fetch_error.check(resp);
      all_good = false;
    }
    const event_request = getEvent({ event: event_id });
    const event_resp = await event_request.result;
    if (event_resp.ok) {
      event = event_resp.data;
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
    setEventOrgState({ event: event_id, org: org_id, state: "Submitted" });
  }
</script>

<main>
  <FetchErrors admin={false} bind:this={fetch_error} />
  {#if loading}
    <Loading />
  {:else}
    <h1 class="text-3xl font-bold underline">
      Wilkommen, {org?.name}!
    </h1>

    <p class="text-lg font-medium">
      Bitte melden Sie die Mannschaften für das Turnier {event?.name}.
    </p>
    <Table>
      <TableHead>
        <TableHeadCell>Team-Name</TableHeadCell>
        <TableHeadCell>Altersgruppe</TableHeadCell>
        <TableHeadCell>Ansprechpartner (Name)</TableHeadCell>
        <TableHeadCell>Telefon</TableHeadCell>
        <TableHeadCell>Bearbeiten</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each teams as team}
          <TableBodyRow>
            <TableBodyCell
              ><Input
                bind:value={team.name}
                oninput={() => change_team(team.id)}
              /></TableBodyCell
            >
            <TableBodyCell>
              <Select
                class="mt-2"
                items={groups}
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
              <ButtonGroup>
                <Button
                  on:click={() => update_team(team.id)}
                  color="green"
                  disabled={!teams_changed.includes(team.id)}>Speichern</Button
                >
                <Button
                  color="yellow"
                  disabled={!teams_changed.includes(team.id)}
                  on:click={() => revert_team(team.id)}>Rückgängig</Button
                >
                <Button
                  color="red"
                  on:click={() => {
                    team_to_delete = team.id;
                    team_delete_modal = true;
                  }}>Team löschen</Button
                >
              </ButtonGroup>
            </TableBodyCell>
          </TableBodyRow>
        {/each}
        <TableBodyRow>
          <TableBodyCell>
            <Button on:click={() => new_team()} color="green"
              >Team hinzufügen...</Button
            >
          </TableBodyCell>
          <TableBodyCell>Status: Anmeldung in Bearbeitung</TableBodyCell>
          <TableBodyCell></TableBodyCell>
          <TableBodyCell></TableBodyCell>
          <TableBodyCell>
            <Button color="red" on:click={() => (submit_modal = true)}
              >Anmeldung finalisieren</Button
            >
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
        <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
          Anmeldungen absenden?
        </h3>
        <Button color="red" class="me-2" on:click={() => submit_teams()}
          >Ja</Button
        >
        <Button color="alternative">Nein (Abbruch)</Button>
      </div>
    </Modal>
  {/if}
</main>
