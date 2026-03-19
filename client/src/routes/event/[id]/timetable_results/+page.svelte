<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    getResults,
    getTimetable,
    type Group,
    type ResultEntry,
    type TimetableRow,
  } from "../../../../api/api";
  import { page } from "$app/state";
  import QrOverlay from "$lib/QrOverlay.svelte";

  let event_name = $state("");
  const event_id = page.params.id || "";
  let start_times: string[] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());
  let groups_results: Group[] = $state([]);
  let results: ResultEntry[] = $state([]);
  let evtSource: EventSource | null = null;
  let enabled_groups: string[] = $state([]);

  onMount(async () => {
    const timetable_request = getTimetable({ event: event_id });

    timetable_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        event_name = resp.data.event_name;
        const new_timetable = new Map<string, TimetableRow[]>();

        resp.data.rows.sort((a, b) => a.expected_time.localeCompare(b.expected_time));

        const seen_active = new Set<string>();
        const new_start_times = new Set<string>();

        for (const row of resp.data.rows) {
          if (seen_active.has(row.group) || row.state == "active") {
            seen_active.add(row.group);
            const time = Date.parse(row.expected_time + "Z");
            const key =
              time.toString() +
              "_" +
              row.flags.split("").map((c) => "flag:" + c).join("_") +
              "_" +
              row.state;

            const r = new_timetable.get(key);
            if (!r) new_timetable.set(key, [row]);
            else r.push(row);
            new_start_times.add(key);
          }
        }

        timetable = new_timetable;
        groupnames = new Map(resp.data.groups.map((g) => [g.id, g.name]));
        if (enabled_groups.length === 0) {
          enabled_groups = resp.data.groups.map((g) => g.id);
        }
        start_times = [...new_start_times].sort((a, b) => a.localeCompare(b));
      }
    });

    const results_request = getResults({ event: event_id });

    results_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        results = resp.data.results.sort((a, b) => (a.rank ?? 0) - (b.rank ?? 0));
        groups_results = resp.data.groups;
        event_name = resp.data.event_name;
        if (enabled_groups.length === 0) {
          enabled_groups = groups_results.map((g) => g.id);
        }
      }
    });

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.kind == "results") results_request.reload();
      if (data.kind == "timetable") timetable_request.reload();
    };
  });

  onDestroy(() => evtSource?.close());

  type EntryWithName = { name: string; flags: string; groups: string[] };

  function timetableFor(key: string): EntryWithName[] {
    const items = timetable.get(key) || [];
    const unique_names = [...new Set(items.map((item) => item.name))].sort();
    return unique_names.map((name) => ({
      name,
      flags: items.find((f) => f.name == name)?.flags || "",
      groups: items.filter((f) => f.name == name).map((r) => groupnames.get(r.group) || ""),
    }));
  }

  function roundForGroup(group: Group): string {
    return results.find((r) => r.group == group.id)?.round?.toString() || "?";
  }

  function timeLabel(key: string): string {
    const t = new Date(parseInt(key)).toTimeString().split(" ")[0].slice(0, 5);
    if (key.endsWith("_next")) return t;
    return "ca.\u00a0" + t;
  }

  // Format a nullable float: integers without decimal, others to 1 dp
  function fmt(v: number | null | undefined): string {
    if (v == null) return "–";
    return Number.isInteger(v) ? String(v) : v.toFixed(1);
  }

  const visibleGroups = $derived(
    groups_results.filter((g) => !g.replacement && enabled_groups.includes(g.id))
  );
</script>

<QrOverlay url="{typeof window !== 'undefined' ? window.location.origin : ''}/event/{event_id}/timetable" />
<main class="h-screen w-screen flex flex-col overflow-hidden bg-gray-950 text-white">

  <!-- ── Timetable bar ──────────────────────────────────────────── -->
  <header class="shrink-0 flex items-center gap-3 border-b border-white/10 px-5 py-2 min-w-0">
    <span class="shrink-0 text-xs font-semibold uppercase tracking-widest text-white/30">{event_name}</span>
    <span class="shrink-0 text-white/10">|</span>
    <div class="flex min-w-0 items-center gap-1 overflow-hidden text-sm">
      {#each start_times as key, i}
        {#if key.includes("_flag:!_") || key.endsWith("_active") || key.endsWith("_next")}
          {@const isActive = key.endsWith("_active")}
          {@const isNext = key.endsWith("_next")}
          {@const entries = timetableFor(key)}

          {#if i > 0}<span class="shrink-0 text-white/20 px-0.5">›</span>{/if}

          <div class="flex shrink-0 items-baseline gap-1.5 {isActive ? 'text-white' : isNext ? 'text-white/60' : 'text-white/35'}">
            {#if isActive}
              <span class="rounded bg-amber-400 px-1.5 py-0.5 text-xs font-bold leading-none text-gray-900">Jetzt</span>
            {:else}
              <span class="tabular-nums text-xs {isNext ? 'text-amber-400/80' : 'text-white/30'}">{timeLabel(key)}</span>
            {/if}
            {#each entries as row}
              <span class="font-medium {isActive ? 'text-white' : ''}">
                {row.name}{#if groupnames.size !== row.groups.length}<span class="ml-1 text-xs opacity-50">({row.groups.sort().join(", ")})</span>{/if}
              </span>
            {/each}
          </div>
        {/if}
      {/each}
    </div>
  </header>

  <!-- ── Results columns ───────────────────────────────────────── -->
  <!--
    Same CSS multi-column layout as timetable_pairing: content flows
    top-to-bottom, fills each column to container height, then spills into
    the next.  Groups can span column boundaries; small groups share columns.
  -->
  <div
    class="flex-1 min-h-0 overflow-hidden p-3"
    style="columns: 4; column-gap: 0.75rem; column-fill: auto;"
  >
    {#each visibleGroups as group (group.id)}
      <!-- Header: break-after-avoid keeps it with its first result row -->
      <div
        class="break-after-avoid break-inside-avoid flex items-baseline justify-between rounded-t-sm px-3 py-1.5 text-gray-900 mt-1 first:mt-0"
        style="background-color: {group.color};"
        role="button"
        tabindex="0"
        ondblclick={() => { enabled_groups = enabled_groups.filter((f) => f != group.id); }}
      >
        <span class="font-bold text-sm">{group.name}</span>
        <span class="text-xs font-medium opacity-70">Runde {roundForGroup(group)}</span>
      </div>

      {#each results.filter((r) => r.group === group.id) as result}
        <div
          class="flex items-stretch border-b border-white/25 text-sm break-inside-avoid"
          style="border-left: 3px solid {group.color};"
        >
          <!-- Rank: large, group-colored background, dark text, vertically centred -->
          <div
            class="flex w-10 shrink-0 items-center justify-center text-2xl font-black text-gray-900 tabular-nums"
            style="background-color: {group.color};"
          >
            {result.rank ?? "–"}
          </div>

          <!-- Team name + org -->
          <div class="flex min-w-0 flex-1 flex-col justify-center px-2 py-1.5">
            <div class="truncate font-bold leading-tight text-white" style="direction: rtl;">{result.team ?? "–"}</div>
            {#if result.team_org}
              <div class="truncate text-xs leading-tight text-white/55">{result.team_org}</div>
            {/if}
          </div>

          <!-- Points: Man.Pkt. / Brt.Pkt. / Buchholz -->
          <div class="flex shrink-0 items-center gap-3 pr-2 text-xs tabular-nums">
            <div class="text-right">
              <div class="font-bold leading-tight text-white">{fmt(result.points_team)}</div>
              <div class="leading-tight text-white/35">Man</div>
            </div>
            <div class="text-right">
              <div class="font-bold leading-tight text-white">{fmt(result.points_player)}</div>
              <div class="leading-tight text-white/35">Brt</div>
            </div>
            <div class="text-right">
              <div class="leading-tight text-white/70">{fmt(result.tie)}</div>
              <div class="leading-tight text-white/35">Buch</div>
            </div>
          </div>
        </div>
      {/each}

      <!-- Gap between groups -->
      <div class="h-2"></div>
    {/each}
  </div>

</main>
