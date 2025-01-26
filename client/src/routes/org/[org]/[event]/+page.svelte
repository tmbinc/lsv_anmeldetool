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
  } from "../../../../api/api";
  import { page } from "$app/state";
  import {
    Button,
    Group,
    Input,
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    type SelectOptionType,
  } from "flowbite-svelte";

  let org: Org | null = $state(null);
  let event: Event | null = $state(null);
  let teams: Team[] = $state([]);
  let org_id = page.params.org;
  let event_id = page.params.event;
  let groups: SelectOptionType<string>[] = $state([]);
  let teams_changed: string[] = $state([]);

  onMount(async () => {
    const request = getOrg({ org: org_id });
    const resp = await request.result;
    if (resp.ok) {
      org = resp.data;
    }
    const event_request = getEvent({ event: event_id });
    const event_resp = await event_request.result;
    if (event_resp.ok) {
      event = event_resp.data;
    }

    const teams_request = getTeamsForOrgEvent({ org: org_id, event: event_id });
    const teams_resp = await teams_request.result;
    if (teams_resp.ok) {
      teams = teams_resp.data;
    }

    const resp_groups = await getGroupsForEvent({ event: event_id }).result;
    if (resp_groups.ok) {
      groups = resp_groups.data.map((group) => ({
        name: group.name,
        value: group.id,
      }));
    }
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

  async function delete_team(id: string) {
    const resp = await deleteTeam({ team: id }).result;
    if (resp.ok) {
      teams_changed = teams_changed.filter((item) => item != id);
      teams = teams.filter((team) => team.id != id);
    }
  }
</script>

<main>
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
      <TableHeadCell>Email</TableHeadCell>
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
            ><Input bind:value={team.contact_phone} /></TableBodyCell
          >
          <TableBodyCell>
            <Button
              on:click={() => update_team(team.id)}
              class="px-3 py-2 text-xs font-medium text-center text-white bg-green-700 rounded-lg hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
              disabled={!teams_changed.includes(team.id)}>Speichern</Button
            >
            <Button
              class="px-3 py-2 text-xs font-medium text-center text-white bg-red-700 rounded-lg hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800"
              disabled={!teams_changed.includes(team.id)}
              on:click={() => revert_team(team.id)}>Rückgängig</Button
            >
            <Button
              class="px-3 py-2 text-xs font-medium text-center text-white bg-red-700 rounded-lg hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800"
              on:click={() => delete_team(team.id)}>Team löschen</Button
            >
          </TableBodyCell>
        </TableBodyRow>
      {/each}
      <TableBodyRow>
        <TableBodyCell>
          <Button
            on:click={() => new_team()}
            class="px-3 py-2 text-s font-medium text-center text-white bg-green-700 rounded-lg hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
            >Team hinzufügen...</Button
          >
        </TableBodyCell>
        <TableBodyCell>Status: Anmeldung in Bearbeitung</TableBodyCell>
        <TableBodyCell></TableBodyCell>
        <TableBodyCell>
          <Button
            class="px-3 py-2 text-s font-medium text-center text-white bg-orange-700 rounded-lg hover:bg-orange-800 focus:ring-4 focus:outline-none focus:ring-orange-300 dark:bg-orange-600 dark:hover:bg-orange-700 dark:focus:ring-orange-800"
            >Anmeldung finalisieren</Button
          >
        </TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
</main>
