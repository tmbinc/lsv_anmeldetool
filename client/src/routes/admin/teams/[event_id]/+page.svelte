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
  import { Button } from "flowbite-svelte";
  import FetchErrors from "../../../FetchErrors.svelte";

  let event_id = page.params.event_id;
  let fetch_errors: FetchErrors;

  let groups = $state(new Map<string, GroupTeamStats>());
  const event_orgs: EventOrg[] = $state([]);
  $inspect(groups);

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
                console.log(team.group_id, ready, group);
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
  });
</script>

<div class="m-6">
  <FetchErrors bind:this={fetch_errors} />

  <h1 class="text-7xl">Groups</h1>
  {#each [...groups.entries()] as [k, group]}
    <div class="mb-5">
      <Button href="/admin/teams/{page.params.event_id}/{group.group.id}">
        {group.group.name}
      </Button>
      {group.ready} of
      {group.total} Teams ready.
    </div>
  {/each}
</div>
