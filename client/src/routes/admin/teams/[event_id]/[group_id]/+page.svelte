<script lang="ts">
  import { onMount } from "svelte";
  import {
    Button,
    Checkbox,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import {
    getEventOrgs,
    getGroup,
    getGroupsForEvent,
    getTeamsForOrgEvent,
    setTeamPresent,
    type EventOrg,
    type EventOrgState,
    type Group,
    type Org,
    type Team,
  } from "../../../../../api/api";

  import { page } from "$app/state";
  import { Table } from "flowbite-svelte";
  import FetchErrors from "../../../../FetchErrors.svelte";

  let event_id = page.params.event_id;
  let group_id = page.params.group_id;
  let fetch_errors: FetchErrors;
  let filter_ready = $state(false);

  let group: Group | undefined = $state(undefined);
  let event_orgs: EventOrg[] = $state([]);

  type OrgTeam = {
    org: Org;
    team: Team;
    org_state: EventOrgState;
  };
  let teams: OrgTeam[] = $state([]);

  onMount(async () => {
    const resp_groups = await getGroup({
      group: group_id,
    }).result;

    if (resp_groups.ok) {
      group = resp_groups.data;
    } else {
      fetch_errors.check(resp_groups);
    }

    const resp_event_orgs = await getEventOrgs({ event: event_id }).result;

    if (resp_event_orgs.ok) {
      const team_list: OrgTeam[] = [];

      for (const event_org of resp_event_orgs.data) {
        let org_teams = await getTeamsForOrgEvent({
          org: event_org.org.id,
          event: event_id,
        }).result;

        if (org_teams.ok) {
          const new_teams = org_teams.data
            .filter((team) => team.group_id == page.params.group_id)
            .map((team) => ({
              org: event_org.org,
              team: team,
              org_state: event_org.state,
            }));
          team_list.push(...new_teams);
        } else {
          fetch_errors.check(org_teams);
        }
      }

      team_list.sort(
        (a, b) =>
          a.org.name.localeCompare(b.org.name) ||
          a.team.name.localeCompare(b.team.name)
      );
      console.log(team_list);
      teams = team_list;
    } else {
      fetch_errors.check(resp_event_orgs);
    }
  });

  async function set_team_present(team: OrgTeam) {
    let res = await setTeamPresent({
      team_id: team.team.id,
      ready: team.team.present,
    }).result;
    if (res.ok) {
    } else {
      fetch_errors.check(res);
      alert("failed to set team readiness");
    }
  }
</script>

<div class="mb-6 md:mx-10">
  <h1 class="text-4xl">Teams</h1>
  <div>
    <div class="inline text-4xl">Gruppe</div>
    <div class="inline text-4xl">{group?.name}</div>
  </div>
  <FetchErrors bind:this={fetch_errors} />
  <div class="m-5">
    <Checkbox bind:checked={filter_ready}>Show only ready</Checkbox>
  </div>
  <Table hoverable={true} shadow>
    <TableHead>
      <TableHeadCell>#</TableHeadCell>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>Schule</TableHeadCell>
      <TableHeadCell class="collapse md:visible">State</TableHeadCell>
      <TableHeadCell class="collapse md:visible">Present</TableHeadCell>
    </TableHead>
    {#each teams.filter((t) => !filter_ready || (t.team.present && t.org_state == "Verified")) as team, index}
      <TableBodyRow
        class={team.org_state == "Verified" && team.team.present
          ? "bg-green-300 hover:bg-green-200"
          : "bg-red-400 hover:bg-red-300"}
      >
        <TableBodyCell>{index + 1}</TableBodyCell>
        <TableBodyCell>
          {team.team.name}</TableBodyCell
        >
        <TableBodyCell>
          <a href="/org/{team.org.id}/{event_id}">{team.org.name}</a>
        </TableBodyCell>
        <TableBodyCell class="collapse md:visible">
          <a href="/admin/event/{event_id}">{team.org_state}</a>
        </TableBodyCell>
        <TableBodyCell class="collapse md:visible">
          <Checkbox
            disabled={team.org_state != "Verified"}
            bind:checked={team.team.present}
            on:change={() => set_team_present(team)}
          ></Checkbox>
        </TableBodyCell>
      </TableBodyRow>
    {/each}
  </Table>
  <Button class="m-10" disabled={!filter_ready}
    >SwissChess Mannschaftsliste...</Button
  >
</div>
