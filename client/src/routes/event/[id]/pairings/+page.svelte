<script lang="ts">
  import { page } from "$app/state";
  import { onDestroy, onMount } from "svelte";
  import { getPairings, type Group, type PairingEntry, type Room } from "../../../../api/api";
  import { browser } from "$app/environment";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  let pairings: PairingEntry[] = $state([]);
  let groups: Group[] = $state([]);
  let rooms: Room[] = $state([]);
  let orgs: string[] = $state([]);

  let group_selected = $state(browser ? (localStorage.getItem("group_selected") ?? "*") : "*");
  let org_selected = $state(browser ? (localStorage.getItem("org_selected") ?? "*") : "*");

  const event_id = page.params.id || "";
  let evtSource: EventSource | null = null;

  function saveFilters() {
    if (browser) {
      localStorage.setItem("org_selected", org_selected);
      localStorage.setItem("group_selected", group_selected);
    }
  }

  onMount(async () => {
    const pairing_request = getPairings({ event: event_id });

    pairing_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        pairings = resp.data.pairings;
        groups = resp.data.groups;
        event_name = resp.data.event_name;
        rooms = resp.data.rooms;
        pairings.sort((a, b) => a.table - b.table);

        const seen = new Set<string>();
        for (const p of pairings) {
          if (p.team_home_org) seen.add(p.team_home_org);
          if (p.team_guest_org) seen.add(p.team_guest_org);
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
      if (data.kind == "pairing") pairing_request.reload();
    };
  });

  onDestroy(() => evtSource?.close());

  function findRoom(group_id: string, table: number): string {
    for (const room of rooms) {
      if (room.group_id == group_id && room.table_num_low <= table && room.table_num_high >= table)
        return room.room;
    }
    return "";
  }

  function roundForGroup(group: Group): number | undefined {
    return pairings.find((p) => p.group == group.id)?.round;
  }

  function fmt(v: number | null | undefined): string {
    if (v == null) return "";
    return Number.isInteger(v) ? String(v) : v.toFixed(1);
  }

  const nonReplacementGroups = $derived(groups.filter((g) => !g.replacement));

  const visibleGroups = $derived(
    nonReplacementGroups.filter((g) => group_selected === "*" || group_selected === g.id)
  );
</script>

<div class="min-h-screen bg-gray-50">

  <!-- Sticky header -->
  <header class="sticky top-0 z-30 border-b border-gray-200 bg-white shadow-sm">

    <!-- Nav row -->
    <div class="flex items-center gap-3 px-4 pt-2 pb-1 text-xs text-gray-400">
      <a href="/event/{event_id}/timetable" class="hover:text-gray-600 transition-colors">← Zeitplan</a>
      <span class="opacity-40">|</span>
      <a href="/event/{event_id}/results" class="hover:text-gray-600 transition-colors">Ergebnisse →</a>
    </div>

    <!-- Title -->
    <div class="px-4 pt-1 pb-2">
      <h1 class="text-lg font-bold text-gray-900">{event_name || "Paarungen"}</h1>
    </div>

    <!-- Org selector -->
    {#if orgs.length > 0}
      <div class="px-4 pb-2">
        <select
          bind:value={org_selected}
          onchange={saveFilters}
          class="w-full rounded-lg border border-gray-200 bg-white px-2 py-1.5 text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-400"
        >
          <option value="*">Alle Schulen</option>
          {#each orgs as org}
            <option value={org}>{org}</option>
          {/each}
        </select>
      </div>
    {/if}

    <!-- Group jump / filter pills -->
    {#if nonReplacementGroups.length > 1}
      <div class="flex flex-wrap gap-2 px-4 pb-2.5">
        <button
          onclick={() => { group_selected = "*"; saveFilters(); }}
          class="flex shrink-0 items-center whitespace-nowrap rounded-full border px-3 py-1 text-xs font-semibold transition-all
            {group_selected === '*' ? 'border-blue-400 bg-blue-50 text-blue-700' : 'border-gray-200 bg-white text-gray-500 hover:bg-gray-50'}"
        >Alle</button>
        {#each nonReplacementGroups as group (group.id)}
          <button
            onclick={() => {
              group_selected = group.id;
              saveFilters();
              document.getElementById(`group-${group.id}`)?.scrollIntoView({ behavior: "smooth", block: "start" });
            }}
            class="flex shrink-0 items-center gap-1.5 whitespace-nowrap rounded-full border px-3 py-1 text-xs font-semibold transition-all
              {group_selected === group.id ? 'border-blue-400 bg-blue-50 text-blue-700' : 'border-gray-200 bg-white text-gray-500 hover:bg-gray-50'}"
          >
            <span class="h-2 w-2 shrink-0 rounded-full" style="background-color: {group.color};"></span>
            {group.name}
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
        Paarungen konnten nicht geladen werden.
      </div>

    {:else if visibleGroups.length === 0}
      <div class="flex items-center justify-center py-24 text-sm text-gray-400">
        Noch keine Paarungen verfügbar.
      </div>

    {:else}
      {#each visibleGroups as group (group.id)}
        {@const groupPairings = pairings.filter((p) =>
          p.group === group.id &&
          (org_selected === "*" || org_selected === p.team_home_org || org_selected === p.team_guest_org)
        )}
        {@const round = roundForGroup(group)}

        <div id="group-{group.id}" class="scroll-mt-44 overflow-hidden rounded-xl border border-gray-200 bg-white shadow-sm">

          <!-- Group header -->
          <div class="flex items-center justify-between px-4 py-3" style="background-color: color-mix(in srgb, {group.color} 30%, white); border-left: 4px solid {group.color};">
            <span class="font-bold text-gray-900">{group.name}</span>
            {#if round}
              <span class="rounded-full px-3 py-0.5 text-sm font-bold text-gray-900" style="background-color: color-mix(in srgb, {group.color} 50%, white);">Runde {round}</span>
            {/if}
          </div>

          {#if groupPairings.length === 0}
            <p class="px-4 py-4 text-sm text-gray-400">Keine Paarungen.</p>
          {:else}
            <div class="flex flex-col gap-1.5 p-2">
            {#each groupPairings as pairing, i}
              {@const hl_home  = org_selected !== "*" && pairing.team_home_org === org_selected}
              {@const hl_guest = org_selected !== "*" && pairing.team_guest_org === org_selected}
              {@const room = findRoom(group.id, pairing.table)}

              <div
                class="overflow-hidden rounded-lg border border-gray-200 bg-white shadow-sm"
                style="border-left: 4px solid {group.color};"
              >
                <div class="flex items-stretch">

                  <!-- Table number + room -->
                  <div
                    class="flex w-14 shrink-0 flex-col items-center justify-center py-2 text-gray-900"
                    style="background-color: color-mix(in srgb, {group.color} 25%, white);"
                  >
                    <span class="text-[10px] font-semibold uppercase leading-none tracking-wide opacity-60">Tisch</span>
                    <span class="text-2xl font-black tabular-nums leading-tight">{pairing.table}</span>
                    {#if room}
                      <span class="mt-0.5 text-center text-[10px] leading-tight opacity-60">{room}</span>
                    {/if}
                  </div>

                  <!-- Teams: stacked on mobile, side-by-side on sm+ -->
                  <div class="flex min-w-0 flex-1 flex-col sm:flex-row sm:items-stretch">

                    <!-- Home team (left-aligned) -->
                    <div class="flex min-w-0 flex-1 flex-col justify-center px-2 pt-2 pb-1 sm:py-2 {hl_home ? 'bg-amber-50' : ''}">
                      <div class="truncate font-semibold leading-tight {hl_home ? 'text-gray-900' : 'text-gray-800'}">
                        {pairing.team_home ?? "–"}
                      </div>
                      <div class="mt-0.5 flex flex-wrap items-center gap-x-1.5 gap-y-0.5">
                        {#if pairing.team_home_org}
                          <span class="text-xs leading-tight {hl_home ? 'text-amber-700' : 'text-gray-400'}">{pairing.team_home_org}</span>
                        {/if}
                        {#if pairing.points_home != null}
                          <span class="tabular-nums text-xs text-gray-500">({fmt(pairing.points_home)})</span>
                        {/if}
                        {#if pairing.team_guest !== null}
                          <span class="flex items-center gap-0.5">
                            <tt class="inline-flex h-4 w-4 items-center justify-center bg-black text-[10px] font-bold leading-none text-white">1</tt>
                            <tt class="inline-flex h-4 w-4 items-center justify-center border border-gray-400 bg-white text-[10px] font-bold leading-none text-black">2</tt>
                            <tt class="inline-flex h-4 w-4 items-center justify-center bg-black text-[10px] font-bold leading-none text-white">3</tt>
                            <tt class="inline-flex h-4 w-4 items-center justify-center border border-gray-400 bg-white text-[10px] font-bold leading-none text-black">4</tt>
                            <span class="text-xs leading-none text-gray-300">…</span>
                          </span>
                        {/if}
                      </div>
                    </div>

                    <!-- Score row (mobile only) -->
                    {#if pairing.team_guest !== null}
                      <div class="flex items-center gap-2 px-2 py-0.5 sm:hidden">
                        <div class="h-px flex-1 bg-gray-100"></div>
                        <span class="text-xs font-medium text-gray-400">&nbsp;:&nbsp;</span>
                        <div class="h-px flex-1 bg-gray-100"></div>
                      </div>
                    {/if}

                    <!-- Separator (desktop only) -->
                    <div class="hidden sm:flex w-5 shrink-0 items-center justify-center border-x border-gray-100 text-xs font-bold text-gray-300">:</div>

                    <!-- Guest team (right-aligned) -->
                    <div class="flex min-w-0 flex-1 flex-col items-end justify-center px-2 pb-2 pt-1 sm:py-2 {hl_guest ? 'bg-amber-50' : ''}">
                      <div class="truncate font-semibold leading-tight {hl_guest ? 'text-gray-900' : 'text-gray-800'}" style="direction: rtl;">
                        {pairing.team_guest ?? "spielfrei"}
                      </div>
                      <div class="mt-0.5 flex flex-wrap items-center justify-end gap-x-1.5 gap-y-0.5">
                        {#if pairing.team_guest !== null}
                          <span class="flex items-center gap-0.5">
                            <tt class="inline-flex h-4 w-4 items-center justify-center border border-gray-400 bg-white text-[10px] font-bold leading-none text-black">1</tt>
                            <tt class="inline-flex h-4 w-4 items-center justify-center bg-black text-[10px] font-bold leading-none text-white">2</tt>
                            <tt class="inline-flex h-4 w-4 items-center justify-center border border-gray-400 bg-white text-[10px] font-bold leading-none text-black">3</tt>
                            <tt class="inline-flex h-4 w-4 items-center justify-center bg-black text-[10px] font-bold leading-none text-white">4</tt>
                            <span class="text-xs leading-none text-gray-300">…</span>
                          </span>
                        {/if}
                        {#if pairing.points_guest != null}
                          <span class="tabular-nums text-xs text-gray-500">({fmt(pairing.points_guest)})</span>
                        {/if}
                        {#if pairing.team_guest_org}
                          <span class="text-xs leading-tight {hl_guest ? 'text-amber-700' : 'text-gray-400'}">{pairing.team_guest_org}</span>
                        {/if}
                      </div>
                    </div>

                  </div>
                </div>
              </div>
            {/each}
            </div>
          {/if}

        </div>
      {/each}
    {/if}

  </main>
</div>
