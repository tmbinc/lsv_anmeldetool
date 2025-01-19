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
  } from "../../../../api/api";
  import { page } from "$app/state";
  import {
    Button,
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
          <TableBodyCell>{team.name}</TableBodyCell>
          <TableBodyCell>U10</TableBodyCell>
          <TableBodyCell>Teamleiter</TableBodyCell>
          <TableBodyCell>(phone)</TableBodyCell>
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
        <TableBodyCell>a</TableBodyCell>
        <TableBodyCell>b</TableBodyCell>
        <TableBodyCell>xx</TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
</main>
