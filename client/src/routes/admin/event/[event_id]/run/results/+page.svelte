<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { getGroupsForEvent, type Group } from "../../../../../../api/api";
  import FetchErrors from "../../../../../FetchErrors.svelte";

  let groups: Group[] = $state([]);
  let fetch_errors: FetchErrors;
  const event_id = page.params.event_id || "";

  onMount(async () => {
    const resp = await getGroupsForEvent({ event: event_id }).result;
    if (resp.ok) {
      groups = resp.data;
    } else {
      fetch_errors.check(resp);
    }
  });
</script>

<FetchErrors bind:this={fetch_errors} />

<div class="mx-auto max-w-3xl px-6 py-10">
  <h1 class="mb-6 text-2xl font-bold text-gray-900">Ergebnisse melden</h1>
  <div class="grid gap-3 sm:grid-cols-2">
    {#each groups as group}
      <a
        href="/admin/event/{event_id}/{group.id}/"
        class="flex items-center gap-4 rounded-xl border border-transparent p-4 shadow-sm transition-all hover:brightness-95"
        style="background-color: {group.color};"
      >
        <span class="font-medium text-gray-900">{group.name}</span>
      </a>
    {/each}
  </div>
</div>
