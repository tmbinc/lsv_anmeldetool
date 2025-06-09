<script lang="ts">
  let props = $props();
  import { page } from "$app/state";

  import { onMount } from "svelte";
  import { getGroupsForEvent, type Group } from "../../../../../api/api";
  import FetchErrors from "../../../../FetchErrors.svelte";
  import { Button } from "flowbite-svelte";

  let groups: Group[] = $state([]);
  let fetch_errors: FetchErrors;
  const event_id = page.params.event_id;

  onMount(async () => {
    const resp_groups = await getGroupsForEvent({ event: event_id }).result;
    if (resp_groups.ok) {
      groups = resp_groups.data;
    } else {
      fetch_errors.check(resp_groups);
    }
  });
</script>

<FetchErrors bind:this={fetch_errors} />
