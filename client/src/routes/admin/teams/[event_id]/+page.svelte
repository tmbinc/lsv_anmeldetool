<script lang="ts">
  import { onMount } from "svelte";
  import {
    getEventOrgs,
    getGroupsForEvent,
    getTeamsForOrgEvent,
    type EventOrg,
    type Group,
    type Org,
  } from "../../../../api/api";

  import { page } from "$app/state";
  import {
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";
  import FetchErrors from "../../../FetchErrors.svelte";

  let event_id = page.params.event_id;
  let fetch_errors: FetchErrors;

  let groups = $state(new Map<string, GroupTeamStats>());
  let all_ready = $state(0);
  let all_total = $state(0);
  const event_orgs: EventOrg[] = $state([]);

  type GroupTeamStats = {
    group: Group;
    total: number;
    ready: number;
  };

  onMount(async () => {
    const resp_groups = await getGroupsForEvent({
      event: page.params.event_id,
    }).result;

    let new_groups = new Map<string, GroupTeamStats>();
    if (resp_groups.ok) {
      new_groups = new Map(
        resp_groups.data.map((group) => [
          group.id,
          {
            group: group,
            total: 0,
            ready: 0,
          },
        ])
      );
    }

    const resp_event_orgs = await getEventOrgs({ event: event_id }).result;

    if (resp_event_orgs.ok) {
      for (const event_org of resp_event_orgs.data) {
        let org_teams = await getTeamsForOrgEvent({
          org: event_org.org.id,
          event: event_id,
        }).result;

        if (org_teams.ok) {
          for (const team of org_teams.data) {
            const ready = team.present && event_org.state == "Verified";
            if (team.group_id) {
              let group = new_groups.get(team.group_id);
              if (group) {
                group.ready += ready ? 1 : 0;
                group.total += 1;
              }
            }
          }
        } else {
          fetch_errors.check(org_teams);
        }
      }
    } else {
      fetch_errors.check(resp_event_orgs);
    }
    groups = new_groups;
    all_total = groups
      .values()
      .map((g) => g.total)
      .reduce((sum, number) => sum + number, 0);
    all_ready = groups
      .values()
      .map((g) => g.ready)
      .reduce((sum, number) => sum + number, 0);
  });
</script>

<div class="sm:m-6">
  <FetchErrors bind:this={fetch_errors} />

  <div class="text-7xl">Groups</div>
  <div>
    <Table shadow hoverable>
      <TableBody>
        {#each [...groups.entries()] as [k, group]}
          <TableBodyRow
            class={group.total == group.ready
              ? "bg-green-300 hover:bg-green-200"
              : "bg-red-300 hover:bg-red-200"}
          >
            <TableBodyCell>
              <Button
                class="w-full"
                href="/admin/teams/{page.params.event_id}/{group.group.id}"
              >
                {group.group.name}
              </Button>
            </TableBodyCell>
            <TableBodyCell>
              <div class="text-center">
                {group.ready} of
                {group.total} Teams ready.
              </div>
            </TableBodyCell>
          </TableBodyRow>
        {/each}
        <TableBodyRow
          ><TableBodyCell>
            <Button href="/admin/teams/{page.params.event_id}/all">all</Button>
          </TableBodyCell><TableBodyCell>
            <div class="text-center">
              Total: {all_total}
              of {all_ready} Teams ready.
            </div>
          </TableBodyCell></TableBodyRow
        >
      </TableBody>
    </Table>
  </div>
</div>
