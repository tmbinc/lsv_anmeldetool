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
  } from "../../../../api/api";
  import { page } from "$app/state";
  import {
    Button,
    Group,
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";

  let org: Org | null = $state(null);
  let event: Event | null = $state(null);
  let teams: Team[] = $state([]);
  let org_id = page.params.org;
  let event_id = page.params.event;
  let groups = $state([]);
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
      groups = resp_groups.data;
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
      updateTeam({ ...event_to_update, event: event_to_update.id });
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
          <TableBodyCell><input bind:value={team.name} /></TableBodyCell>
          <TableBodyCell>
            <Select class="mt-2" items={groups} bind:value={team.group_id} />
          </TableBodyCell>
          <TableBodyCell><input bind:value={team.contact_name} /></TableBodyCell
          >
          <TableBodyCell
            ><input bind:value={team.contact_phone} /></TableBodyCell
          >
        </TableBodyRow>
      {/each}
      <TableBodyRow>
        <TableBodyCell>
          <Button
            on:click={() => new_team()}
            class="px-3 py-2 text-xs font-medium text-center text-white bg-green-700 rounded-lg hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
            >Neu...</Button
          >
        </TableBodyCell>
        <TableBodyCell></TableBodyCell>
        <TableBodyCell></TableBodyCell>
        <TableBodyCell></TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
</main>
