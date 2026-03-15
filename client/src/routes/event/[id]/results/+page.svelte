<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    getResults,
    type Group,
    type ResultEntry,
  } from "../../../../api/api";
  import { page } from "$app/state";
  import { browser } from "$app/environment";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  const event_id = page.params.id || "";
  let groups: Group[] = $state([]);
  let results: ResultEntry[] = $state([]);
  let evtSource: EventSource | null = null;

  // Org selection — shared localStorage key with the pairings page so the
  // selection persists when navigating between the two views.
  let org_selected = $state((browser && localStorage.getItem("org_selected")) ?? "*");
  let orgs: string[] = $state([]);

  function saveOrg() {
    if (browser) localStorage.setItem("org_selected", org_selected);
  }

  onMount(async () => {
    const results_request = getResults({ event: event_id });

    results_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        results = resp.data.results.sort((a, b) => (a.rank ?? 0) - (b.rank ?? 0));
        groups = resp.data.groups;
        event_name = resp.data.event_name;

        // Collect unique orgs (preserve alphabetic order)
        const seen = new Set<string>();
        for (const r of results) {
          if (r.team_org) seen.add(r.team_org);
        }
        orgs = [...seen].sort((a, b) => a.localeCompare(b));

        loading = false;
      } else if (resp) {
        failed_load = true;
        loading = false;
      }
    });

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.kind == "results") results_request.reload();
    };
  });

  onDestroy(() => evtSource?.close());

  function roundForGroup(group: Group): number | null {
    return results.find((r) => r.group == group.id)?.round ?? null;
  }

  function fmt(v: number | null | undefined): string {
    if (v == null) return "–";
    return Number.isInteger(v) ? String(v) : v.toFixed(1);
  }

  const visibleGroups = $derived(groups.filter((g) => !g.replacement));
</script>

<div class="min-h-screen bg-gray-50">

  <!-- Sticky header with org selector + group jump pills -->
  <header class="sticky top-0 z-10 border-b border-gray-200 bg-white shadow-sm">
    <!-- Row 1: title -->
    <div class="px-4 pt-3 pb-2">
      <h1 class="text-lg font-bold text-gray-900">{event_name || "Ergebnisse"}</h1>
    </div>

    <!-- Row 2: org selector -->
    {#if orgs.length > 0}
      <div class="px-4 pb-2">
        <select
          bind:value={org_selected}
          onchange={saveOrg}
          class="w-full rounded-lg border border-gray-200 bg-white px-2 py-1.5 text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-400"
        >
          <option value="*">Alle Schulen</option>
          {#each orgs as org}
            <option value={org}>{org}</option>
          {/each}
        </select>
      </div>
    {/if}

    <!-- Row 2: group jump pills (only when more than one group) -->
    {#if visibleGroups.length > 1}
      <div class="flex flex-wrap gap-2 px-4 pb-2.5">
        {#each visibleGroups as group (group.id)}
          {@const merged = groups.filter((g) => g.replacement === group.id)}
          <button
            onclick={() => document.getElementById(`group-${group.id}`)?.scrollIntoView({ behavior: "smooth", block: "start" })}
            class="flex shrink-0 items-center gap-1.5 whitespace-nowrap rounded-full border border-gray-200 bg-white px-3 py-1 text-xs font-semibold text-gray-700 shadow-sm transition-all hover:bg-gray-50 active:bg-gray-100"
          >
            <span class="h-2 w-2 shrink-0 rounded-full" style="background-color: {group.color};"></span>
            {group.name}{#if merged.length > 0}<span class="ml-0.5 font-normal text-gray-400">({merged.map((g) => g.name).join(", ")})</span>{/if}
          </button>
        {/each}
      </div>
    {/if}
  </header>

  <main class="mx-auto max-w-2xl px-4 py-6 space-y-5">

    {#if loading}
      <div class="flex items-center justify-center py-24 text-sm text-gray-400">Lade…</div>

    {:else if failed_load}
      <div class="rounded-xl border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-600">
        Ergebnisse konnten nicht geladen werden.
      </div>

    {:else if visibleGroups.length === 0}
      <div class="flex items-center justify-center py-24 text-sm text-gray-400">
        Noch keine Ergebnisse verfügbar.
      </div>

    {:else}
      {#each visibleGroups as group (group.id)}
        {@const groupResults = results.filter((r) => r.group === group.id)}
        {@const round = roundForGroup(group)}

        <div id="group-{group.id}" class="scroll-mt-44 overflow-hidden rounded-xl border border-gray-200 bg-white shadow-sm">

          <!-- Group header -->
          <div
            class="flex items-baseline justify-between px-4 py-3"
            style="background-color: {group.color};"
          >
            <span class="font-bold text-gray-900">{group.name}</span>
            {#if round}
              <span class="text-sm font-medium text-gray-800 opacity-70">Stand: Runde {round}</span>
            {:else}
              <span class="text-sm text-gray-800 opacity-40">Keine Ergebnisse</span>
            {/if}
          </div>

          {#if groupResults.length === 0}
            <p class="px-4 py-4 text-sm text-gray-400">Noch keine Ergebnisse.</p>
          {:else}

            <!-- Column labels -->
            <div
              class="grid border-b border-gray-100 bg-gray-50 px-2 py-1.5 text-xs font-semibold uppercase tracking-wide text-gray-400"
              style="grid-template-columns: 2.25rem 1fr 3.5rem 3.5rem 3.5rem;"
            >
              <div class="text-center">#</div>
              <div class="px-1">Mannschaft</div>
              <div class="text-right">Man.</div>
              <div class="text-right">Brt.</div>
              <div class="text-right">Buchh</div>
            </div>

            {#each groupResults as result, i}
              {@const highlighted = org_selected !== "*" && result.team_org === org_selected}
              {@const dimmed     = org_selected !== "*" && result.team_org !== org_selected}
              <div
                class="grid items-center border-b border-gray-50 px-2 py-2 last:border-0
                  {highlighted ? 'bg-amber-50' : dimmed ? '' : i % 2 === 1 ? 'bg-gray-50/60' : ''}"
                style="grid-template-columns: 2.25rem 1fr 3.5rem 3.5rem 3.5rem; border-left: 3px solid {group.color};"
              >
                <!-- Rank -->
                <div class="text-center tabular-nums
                  {highlighted ? 'text-base font-black text-gray-700' : 'text-sm font-black text-gray-400'}
                  {dimmed ? 'opacity-40' : ''}">
                  {result.rank ?? "–"}
                </div>

                <!-- Team + org -->
                <div class="min-w-0 px-1 {dimmed ? 'opacity-40' : ''}">
                  <div class="truncate leading-tight
                    {highlighted ? 'font-bold text-gray-900' : 'font-semibold text-gray-900'}">
                    {result.team ?? "–"}
                  </div>
                  {#if result.team_org}
                    <div class="truncate text-xs leading-tight
                      {highlighted ? 'font-medium text-amber-700' : 'text-gray-400'}">
                      {result.team_org}
                    </div>
                  {/if}
                </div>

                <!-- Points -->
                <div class="text-right tabular-nums text-sm font-semibold
                  {highlighted ? 'text-gray-800' : 'text-gray-700'} {dimmed ? 'opacity-40' : ''}">
                  {fmt(result.points_team)}
                </div>
                <div class="text-right tabular-nums text-sm font-medium
                  {highlighted ? 'text-gray-700' : 'text-gray-600'} {dimmed ? 'opacity-40' : ''}">
                  {fmt(result.points_player)}
                </div>
                <div class="text-right tabular-nums text-sm
                  {highlighted ? 'text-gray-500' : 'text-gray-400'} {dimmed ? 'opacity-40' : ''}">
                  {fmt(result.tie)}
                </div>
              </div>
            {/each}

          {/if}
        </div>
      {/each}
    {/if}

  </main>
</div>
