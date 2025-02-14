<script lang="ts">
  import { onMount, type Component } from "svelte";
  import {
    Button,
    Checkbox,
    TableBody,
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
  let groups: Group[] = $state([]);
  let event_orgs: EventOrg[] = $state([]);
  let group_names = $state(new Map<string, string>());
  let download_a: Element;

  type OrgTeam = {
    org: Org;
    team: Team;
    org_state: EventOrgState;
  };

  let teams: OrgTeam[] = $state([]);

  type SwissChessTeam = {
    number: string;
    teamname: string;
    federation: string;
    rank: string;
    state: string;
  };

  onMount(async () => {
    if (group_id != "all") {
      const resp_group = await getGroup({
        group: group_id,
      }).result;

      if (resp_group.ok) {
        group = resp_group.data;
      } else {
        fetch_errors.check(resp_group);
      }
    } else {
      const resp_groups = await getGroupsForEvent({
        event: event_id,
      }).result;

      if (resp_groups.ok) {
        groups = resp_groups.data;
      } else {
        fetch_errors.check(resp_groups);
      }
      group_names = new Map(groups.map((group) => [group.id, group.name]));
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
            .filter((team) => group_id == "all" || team.group_id == group_id)
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

  async function download_swisschess() {
    const swiss_teams: SwissChessTeam[] = teams.map((n, i) => ({
      number: (i + 1).toString(),
      teamname: n.team.name,
      federation: n.org.slug,
      rank: "0",
      state: "",
    }));
    let data = { teams: swiss_teams };
    const blob = new Blob([JSON.stringify(data)], {
      type: "text/plain;charset=utf-8",
    });

    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = (group?.name || "WK") + ".json";
    link.style.display = "none";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }
</script>

<div class="m-6 md:mx-10">
  <div class="text-4xl items-center">Teams</div>
  <div>
    {#if group_id != "all"}
      <div class="inline text-4xl">Gruppe</div>
      <div class="inline text-4xl">{group?.name}</div>
    {:else}
      <div class="inline text-4xl">All Groups</div>
    {/if}
  </div>
  <FetchErrors bind:this={fetch_errors} />
  <div class="m-5">
    <Checkbox bind:checked={filter_ready}>Show only ready</Checkbox>
  </div>
  <div class="table-div-class">
    <Table hoverable={true} shadow class="text-sm">
      <TableHead class="sm:table-header-group hidden">
        <TableHeadCell class="td-class">#</TableHeadCell>
        <TableHeadCell class="td-class">Name</TableHeadCell>
        <TableHeadCell class="td-class">Schule</TableHeadCell>
        <TableHeadCell class="td-class">State</TableHeadCell>
        {#if group_id == "all"}
          <TableHeadCell class="td-class">Group</TableHeadCell>
        {/if}
        <TableHeadCell class="td-class">Present</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each teams.filter((t) => !filter_ready || (t.team.present && t.org_state == "Verified")) as team, index}
          <TableBodyRow
            class={"tr-class " +
              (team.org_state == "Verified" && team.team.present
                ? "bg-green-300 hover:bg-green-200"
                : "bg-red-400 hover:bg-red-300")}
          >
            <TableBodyCell class="td-class">{index + 1}</TableBodyCell>
            <TableBodyCell class="td-class">
              <span class="font-bold">
                {team.team.name.substring(0, 32)}</span
              ><span class="text-red-900">
                {team.team.name.substring(32)}</span
              ></TableBodyCell
            >
            <TableBodyCell class="td-class">
              <a href="/org/{team.org.id}/{event_id}">{team.org.name}</a>
              <sup class={team.org.slug.length > 3 ? "text-red-600" : ""}
                >{team.org.slug}</sup
              >
            </TableBodyCell>
            <TableBodyCell class="td-class">
              <a href="/admin/event/{event_id}">{team.org_state}</a>
            </TableBodyCell>

            {#if group_id == "all"}
              <TableBodyCell class="td-class">
                {group_names.get(team.team.group_id || "") || "??"}
              </TableBodyCell>
            {/if}
            <TableBodyCell class="td-class">
              <Checkbox
                disabled={team.org_state != "Verified"}
                bind:checked={team.team.present}
                on:change={() => set_team_present(team)}
              ></Checkbox>
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </div>
  <Button class="m-10" on:click={download_swisschess}
    >Download SwissChess Mannschaftsliste...</Button
  >
</div>

<style lang="postcss">
  :global(.td-class) {
    @apply px-4 py-3;
  }
  :global(.tr-class) {
    @apply flex flex-col mb-4 sm:table-row;
  }
  :global(.table-div-class) {
    @apply flex sm:justify-normal justify-center ml-4 sm:ml-0;
  }
</style>
