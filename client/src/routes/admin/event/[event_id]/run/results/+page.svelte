<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { Button } from "flowbite-svelte";
  import { DownloadOutline } from "flowbite-svelte-icons";
  import {
    getGroupsForEvent,
    getResults,
    getEvent,
    type Group,
    type ResultEntry,
  } from "../../../../../../api/api";
  import FetchErrors from "../../../../../FetchErrors.svelte";

  let groups: Group[] = $state([]);
  let results: ResultEntry[] = $state([]);
  let event_name = $state("");
  let fetch_errors: FetchErrors;
  const event_id = page.params.event_id || "";

  onMount(async () => {
    const resp = await getGroupsForEvent({ event: event_id }).result;
    if (resp.ok) {
      groups = resp.data;
    } else {
      fetch_errors.check(resp);
    }

    const resp_results = await getResults({ event: event_id }).result;
    if (resp_results.ok) {
      results = resp_results.data.results;
      event_name = resp_results.data.event_name;
    } else {
      fetch_errors.check(resp_results);
    }

    const resp_event = await getEvent({ event: event_id }).result;
    if (resp_event.ok && !event_name) {
      event_name = resp_event.data.name;
    }
  });

  const fmt = (v: number | null | undefined): string =>
    v == null ? "" : Number.isInteger(v) ? String(v) : String(v);

  const esc = (v: string | number | null | undefined): string => {
    const s = v == null ? "" : String(v);
    if (s.includes('"') || s.includes(",") || s.includes("\n") || s.includes(";")) {
      return '"' + s.replaceAll('"', '""') + '"';
    }
    return s;
  };

  function download_csv() {
    const groupName = new Map(groups.map((g) => [g.id, g.name]));

    const sorted = [...results].sort((a, b) => {
      const ag = groupName.get(a.group) ?? a.group;
      const bg = groupName.get(b.group) ?? b.group;
      const gc = ag.localeCompare(bg);
      if (gc !== 0) return gc;
      return (a.rank ?? 9999) - (b.rank ?? 9999);
    });

    const cols = [
      "Gruppe",
      "Platz",
      "Mannschaft",
      "Schule",
      "Mannschaftspunkte",
      "Brettpunkte",
      "Buchholz",
      "Siege",
      "Remis",
      "Niederlagen",
      "Runde",
    ];
    const rows = [cols.join(",")];
    for (const r of sorted) {
      rows.push([
        esc(groupName.get(r.group) ?? r.group),
        esc(fmt(r.rank)),
        esc(r.team),
        esc(r.team_org),
        esc(fmt(r.points_team)),
        esc(fmt(r.points_player)),
        esc(fmt(r.tie)),
        esc(fmt(r.points_win)),
        esc(fmt(r.points_draw)),
        esc(fmt(r.points_lost)),
        esc(r.round),
      ].join(","));
    }

    // BOM so Excel detects UTF-8 correctly
    const blob = new Blob(["\ufeff" + rows.join("\n")], {
      type: "text/csv;charset=utf-8",
    });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = (event_name || "Ergebnisse") + ".csv";
    link.style.display = "none";
    document.body.appendChild(link);
    link.click();
    requestAnimationFrame(() => {
      URL.revokeObjectURL(link.href);
      link.remove();
    });
  }
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

  <div class="mt-10 border-t border-gray-200 pt-6">
    <h2 class="mb-3 text-xs font-semibold uppercase tracking-widest text-gray-400">
      Endergebnisse exportieren
    </h2>
    <Button
      color="alternative"
      disabled={results.length === 0}
      on:click={download_csv}
    >
      <DownloadOutline class="me-2 h-4 w-4" />
      Ergebnisse als CSV herunterladen
    </Button>
    {#if results.length === 0}
      <p class="mt-2 text-xs text-gray-400">Noch keine Ergebnisse vorhanden.</p>
    {/if}
  </div>
</div>
