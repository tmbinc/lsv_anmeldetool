<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    getPairings,
    getTimetable,
    type EventOrg,
    type Group,
    type PairingEntry,
    type TimetableRow,
  } from "../../../../api/api";
  import { page } from "$app/state";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  const event_id = page.params.id || "";
  let start_times: string[] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());

  const event_orgs: EventOrg[] = $state([]);
  let groups_pairings: Group[] = $state([]);

  let pairings: PairingEntry[] = $state([]);
  let orgs: Set<string> = new Set();
  let evtSource: EventSource | null = null;
  let enabled_groups: string[] = $state([]);

  onMount(async () => {
    let timetable_request = getTimetable({ event: event_id });

    timetable_request.resp.subscribe((resp) => {
      if (resp) {
        if (resp.ok) {
          event_name = resp.data.event_name;
          let new_timetable = new Map<string, TimetableRow[]>();

          resp.data.rows.sort((a, b) =>
            a.expected_time.localeCompare(b.expected_time),
          );

          let seen_active = new Set<string>();
          let new_start_times = new Set<string>();

          for (const row of resp.data.rows) {
            if (seen_active.has(row.group) || row.state == "active") {
              seen_active.add(row.group);
              const time = Date.parse(row.expected_time + "Z");
              let key =
                time.toString() +
                "_" +
                row.flags
                  .split("")
                  .map((c) => "flag:" + c)
                  .join("_") +
                "_" +
                row.state;

              let r = new_timetable.get(key);
              if (!r) {
                new_timetable.set(key, [row]);
              } else {
                r.push(row);
              }
              new_start_times.add(key);
            }
          }

          timetable = new_timetable;
          groupnames = new Map(resp.data.groups.map((g) => [g.id, g.name]));

          if (enabled_groups) {
            enabled_groups = resp.data.groups.map((g) => g.id);
          }

          start_times = [...new_start_times].sort((a, b) => a.localeCompare(b));
          loading = false;
        } else {
          failed_load = true;
        }
      }
    });

    let pairing_request = getPairings({ event: event_id });

    pairing_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        pairings = resp.data.pairings;
        groups_pairings = resp.data.groups;
        event_name = resp.data.event_name;
        pairings.sort((a, b) => a.table - b.table);
        for (let p of pairings) {
          if (p.team_home_org) orgs.add(p.team_home_org);
          if (p.team_guest_org) orgs.add(p.team_guest_org);
        }
      }
    });

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.kind == "pairing") pairing_request.reload();
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

  function roundForGroup(group: Group) {
    return pairings.find((p) => p.group == group.id)?.round || "?";
  }

  function timeLabel(key: string): string {
    const t = new Date(parseInt(key)).toTimeString().split(" ")[0].slice(0, 5);
    if (key.endsWith("_next")) return t;
    return "ca.\u00a0" + t;
  }

  const visibleGroups = $derived(
    groups_pairings.filter((g) => !g.replacement && enabled_groups.includes(g.id))
  );
</script>

<main class="h-screen w-screen flex flex-col overflow-hidden bg-gray-950 text-white">

  <!-- ── Timetable bar ──────────────────────────────────────────── -->
  <header class="shrink-0 flex items-center gap-3 border-b border-white/10 px-5 py-2 min-w-0">

    <!-- Event name -->
    <span class="shrink-0 text-xs font-semibold uppercase tracking-widest text-white/30">
      {event_name}
    </span>

    <span class="shrink-0 text-white/10">|</span>

    <!-- Timeline items -->
    <div class="flex min-w-0 items-center gap-1 overflow-hidden text-sm">
      {#each start_times as key, i}
        {#if key.includes("_flag:!_") || key.endsWith("_active") || key.endsWith("_next")}
          {@const isActive = key.endsWith("_active")}
          {@const isNext = key.endsWith("_next")}
          {@const entries = timetableFor(key)}

          {#if i > 0}
            <span class="shrink-0 text-white/20 px-0.5">›</span>
          {/if}

          <div class="flex shrink-0 items-baseline gap-1.5 {isActive ? 'text-white' : isNext ? 'text-white/60' : 'text-white/35'}">
            <!-- Time / state pill -->
            {#if isActive}
              <span class="rounded bg-amber-400 px-1.5 py-0.5 text-xs font-bold leading-none text-gray-900">
                Jetzt
              </span>
            {:else}
              <span class="tabular-nums text-xs {isNext ? 'text-amber-400/80' : 'text-white/30'}">
                {timeLabel(key)}
              </span>
            {/if}

            <!-- Entry names -->
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

  <!-- ── Pairing columns ───────────────────────────────────────── -->
  <div
    class="flex-1 overflow-hidden grid gap-3 p-3"
    style="grid-template-columns: repeat({visibleGroups.length || 1}, minmax(0, 1fr));"
  >
    {#each groups_pairings.filter((f) => !f.replacement) as group (group.id)}
      <div
        class="flex flex-col overflow-hidden rounded-lg"
        hidden={!enabled_groups.includes(group.id)}
        role="button"
        tabindex="0"
        ondblclick={() => { enabled_groups = enabled_groups.filter((f) => f != group.id); }}
      >
        <!-- Group header -->
        <div
          class="shrink-0 flex items-baseline justify-between px-3 py-2 text-gray-900"
          style="background-color: {group.color};"
        >
          <span class="font-bold leading-tight">{group.name}</span>
          <span class="text-sm font-medium opacity-70">Runde {roundForGroup(group)}</span>
        </div>

        <!-- Pairing table -->
        <div class="flex-1 overflow-auto">
          <table class="w-full text-sm border-collapse">
            <thead>
              <tr class="border-b border-white/10 text-left text-xs font-semibold uppercase tracking-wide text-white/40">
                <th class="px-2 py-1.5 w-8 text-center">#</th>
                <th class="px-2 py-1.5">Brett 1 schwarz</th>
                <th class="px-2 py-1.5">Brett 1 weiß</th>
              </tr>
            </thead>
            <tbody>
              {#each pairings.filter((p) => p.group == group.id) as pairing, i}
                <tr class="border-b border-white/5 {i % 2 === 0 ? 'bg-white/5' : ''}">
                  <td class="px-2 py-1 text-center text-white/40 tabular-nums">{pairing.table}</td>
                  <td class="px-2 py-1">
                    <div class="font-medium leading-tight">{pairing.team_home ?? "–"}</div>
                    {#if pairing.team_home_org}
                      <div class="text-xs text-white/40 leading-tight">{pairing.team_home_org}</div>
                    {/if}
                  </td>
                  <td class="px-2 py-1">
                    <div class="font-medium leading-tight">{pairing.team_guest ?? "spielfrei"}</div>
                    {#if pairing.team_guest_org}
                      <div class="text-xs text-white/40 leading-tight">{pairing.team_guest_org}</div>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/each}
  </div>

</main>
