<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    getResults,
    getTemplate,
    getGroupsForEvent,
    getEventOrgs,
    getEvent,
    type Group,
    type ResultEntry,
    type EventOrg,
  } from "../../../../../../api/api";
  import { page } from "$app/state";
  import { generate } from "@pdfme/generator";
  import { Viewer } from "@pdfme/ui";
  import { BLANK_A4_PDF } from "@pdfme/common";
  import type { Template } from "@pdfme/common";
  import { getFontsData, plugins, placeholders, buildDataMap, buildInput } from "$lib/urkunde";
  import { Button } from "flowbite-svelte";
  import { ArrowLeftOutline, FilePdfOutline } from "flowbite-svelte-icons";

  const event_id = page.params.event_id || "";

  let event_name = $state("");
  let groups_results: Group[] = $state([]);
  let results: ResultEntry[] = $state([]);
  let event_orgs: EventOrg[] = $state([]);
  let selected_group = $state("");
  let selected_team_index = $state(0);
  let generating = $state(false);
  let source_mode = $state<"results" | "teams">("results");

  // Teams mode: flat list of {team, org_name} for selected group, sorted alphabetically
  const groupTeamList = $derived(
    event_orgs
      .flatMap((eo) => eo.teams.map((t) => ({ team: t, org_name: eo.org.name })))
      .filter(({ team }) => team.group_id === selected_group)
      .sort((a, b) => a.team.name.localeCompare(b.team.name))
  );

  // Teams for selected group, sorted by rank
  const groupTeams = $derived(
    results
      .filter((r) => r.group === selected_group)
      .sort((a, b) => (a.rank ?? 999) - (b.rank ?? 999))
  );

  // Unified data rows for the current mode: {data, rank?}
  // rank is only set in results mode, used for subset filtering
  type Row = { data: Record<string, string>; rank?: number };

  const allRows = $derived<Row[]>(
    source_mode === "results"
      ? groupTeams.map((e) => ({
          data: buildDataMap(e, selectedGroupName, event_name),
          rank: e.rank ?? undefined,
        }))
      : groupTeamList.map(({ team, org_name }) => ({
          data: {
            team:          team.name,
            team_name:     team.name,
            team_id:       team.id,
            org:           org_name,
            team_genus:    "",
            rank:          "",
            points_team:   "",
            points_player: "",
            tie:           "",
            group:         selectedGroupName,
            event:         event_name,
          },
        }))
  );

  let subset_expr = $state("");

  function parseSubset(expr: string): Set<number> | null {
    const trimmed = expr.trim();
    if (!trimmed) return null;
    const ranks = new Set<number>();
    for (const part of trimmed.split(",")) {
      const range = part.trim().match(/^(\d+)-(\d+)$/);
      if (range) {
        const lo = parseInt(range[1]), hi = parseInt(range[2]);
        for (let i = lo; i <= hi; i++) ranks.add(i);
      } else {
        const n = parseInt(part.trim());
        if (!isNaN(n)) ranks.add(n);
      }
    }
    return ranks.size > 0 ? ranks : null;
  }

  // Subset-filtered rows: results mode filters by rank, teams mode by 1-based position
  const activeRows = $derived(
    (() => {
      const subset = parseSubset(subset_expr);
      if (!subset) return allRows;
      return allRows.filter((row, i) =>
        source_mode === "results" ? row.rank != null && subset.has(row.rank) : subset.has(i + 1)
      );
    })()
  );

  const BLANK_KEYS = ["rank", "points_team", "points_player", "tie"];
  let vorausdruck = $state(false);

  function applyBlanking(data: Record<string, string>): Record<string, string> {
    if (!vorausdruck && source_mode !== "teams") return data;
    const out = { ...data };
    for (const key of BLANK_KEYS) out[key] = "     ";
    return out;
  }
  let evtSource: EventSource | null = null;

  let viewerContainer: HTMLDivElement;
  let viewer: Viewer | null = $state(null);

  let currentTemplate: Template = { basePdf: BLANK_A4_PDF, schemas: [[]] };

  const selectedGroupName = $derived(
    groups_results.find((g) => g.id === selected_group)?.name ?? ""
  );

  // Reset selection when group or mode changes
  $effect(() => {
    selected_group;
    source_mode;
    selected_team_index = 0;
  });

  // Update viewer when selection, mode, blanking, or template changes
  $effect(() => {
    if (!viewer) return;
    const row = allRows[selected_team_index] ?? allRows[0];
    const inputs = row ? [buildInput(currentTemplate, applyBlanking(row.data))] : [{}];
    viewer.updateTemplate(currentTemplate);
    viewer.setInputs(inputs);
  });

  async function loadTemplate() {
    const resp = await getTemplate({
      event: event_id,
      template_type: "urkunde",
      template_variant: "main",
    }).result;
    if (resp.ok && resp.data) {
      try {
        currentTemplate = JSON.parse(resp.data);
      } catch {
        console.error("Failed to parse template");
      }
    }
  }

  onMount(async () => {
    await loadTemplate();

    viewer = new Viewer({
      domContainer: viewerContainer,
      template: currentTemplate,
      inputs: [{}],
      plugins,
      options: { font: getFontsData() },
    });

    const resp_event = await getEvent({ event: event_id }).result;
    if (resp_event.ok) event_name = resp_event.data.name;

    // Load groups directly so the selector works even without any results uploaded
    const resp_groups = await getGroupsForEvent({ event: event_id }).result;
    if (resp_groups.ok) {
      groups_results = resp_groups.data.filter((g) => !g.replacement);
      if (!selected_group && groups_results.length > 0) {
        selected_group = groups_results[0].id;
      }
    }

    const resp_orgs = await getEventOrgs({ event: event_id }).result;
    if (resp_orgs.ok) {
      event_orgs = resp_orgs.data;
    }

    const results_request = getResults({ event: event_id });
    results_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        results = resp.data.results;
        event_name = resp.data.event_name;
      }
    });

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = (e) => {
      const data = JSON.parse(e.data);
      if (data.kind === "results") results_request.reload();
    };
  });

  onDestroy(() => {
    evtSource?.close();
    viewer?.destroy();
  });

  async function printCurrent() {
    const row = allRows[selected_team_index] ?? allRows[0];
    if (!row) return;
    try {
      const pdf = await generate({
        template: currentTemplate,
        inputs: [buildInput(currentTemplate, applyBlanking(row.data))],
        plugins,
        options: { font: getFontsData() },
      });
      const win = window.open(URL.createObjectURL(new Blob([pdf.buffer], { type: "application/pdf" })));
      win?.addEventListener("load", () => win.print());
    } catch (e) {
      console.error("Print failed", e);
    }
  }

  async function generatePDF() {
    if (!activeRows.length) return;
    generating = true;
    try {
      const inputs = activeRows.map((row) => buildInput(currentTemplate, applyBlanking(row.data)));
      const pdf = await generate({ template: currentTemplate, inputs, plugins, options: { font: getFontsData() } });
      window.open(URL.createObjectURL(new Blob([pdf.buffer], { type: "application/pdf" })));
    } catch (e) {
      console.error("PDF generation failed", e);
    } finally {
      generating = false;
    }
  }
</script>

<div class="flex h-screen flex-col bg-white text-black">
  <!-- Top bar -->
  <div class="flex shrink-0 items-center gap-3 border-b border-gray-200 bg-gray-50 px-4 py-2">
    <a href="/admin/event/{event_id}/run/" class="flex items-center gap-1 text-sm text-gray-500 hover:text-gray-800">
      <ArrowLeftOutline class="h-4 w-4" />
      Zurück
    </a>
    <div class="mx-1 h-4 w-px bg-gray-300"></div>
    <span class="text-sm font-semibold text-gray-700">{event_name || "Urkunden"}</span>
    <a href="/admin/event/{event_id}/pdftemplate/" class="ml-auto text-sm text-gray-400 hover:text-gray-700">
      Vorlage bearbeiten →
    </a>
  </div>

  <!-- Body -->
  <div class="flex min-h-0 flex-1">

    <!-- Left panel -->
    <div class="flex w-72 shrink-0 flex-col border-r border-gray-200 bg-gray-50">

      <!-- Group selector -->
      <div class="border-b border-gray-200 px-4 py-3">
        <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Gruppe</label>
        <select
          bind:value={selected_group}
          class="w-full rounded border border-gray-300 bg-white px-2 py-1.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
        >
          {#each groups_results as group}
            <option value={group.id}>{group.name}</option>
          {/each}
        </select>
      </div>

      <!-- Mode toggle -->
      <div class="border-b border-gray-200 px-4 py-2 flex gap-1">
        <button
          onclick={() => (source_mode = "results")}
          class="flex-1 rounded py-1 text-xs font-medium transition-colors {source_mode === 'results' ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}"
        >
          Ergebnisse
        </button>
        <button
          onclick={() => (source_mode = "teams")}
          class="flex-1 rounded py-1 text-xs font-medium transition-colors {source_mode === 'teams' ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}"
        >
          Teamliste
        </button>
      </div>

      <!-- Team list -->
      <div class="min-h-0 flex-1 overflow-y-auto">
        {#if source_mode === "results"}
          {#if groupTeams.length === 0}
            <p class="px-4 py-6 text-sm text-gray-400">Keine Ergebnisse für diese Gruppe.</p>
          {:else}
            <table class="w-full text-sm">
              <thead>
                <tr class="border-b border-gray-200 text-left text-xs font-semibold uppercase tracking-wide text-gray-500">
                  <th class="px-4 py-2">#</th>
                  <th class="px-2 py-2">Team</th>
                  <th class="px-2 py-2 text-right">Pkt</th>
                </tr>
              </thead>
              <tbody>
                {#each groupTeams as entry, i}
                  <tr
                    class="cursor-pointer border-b border-gray-100 {i === selected_team_index ? 'bg-blue-50' : 'hover:bg-white'}"
                    onclick={() => (selected_team_index = i)}
                  >
                    <td class="px-4 py-2 font-medium text-gray-500">{entry.rank ?? "–"}</td>
                    <td class="px-2 py-2">
                      <div class="font-medium text-gray-900">{entry.team ?? "–"}</div>
                      <div class="text-xs text-gray-400">{entry.team_org ?? ""}</div>
                    </td>
                    <td class="px-2 py-2 text-right text-gray-700">{entry.points_team ?? "–"}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        {:else}
          {#if groupTeamList.length === 0}
            <p class="px-4 py-6 text-sm text-gray-400">Keine Teams in dieser Gruppe.</p>
          {:else}
            <table class="w-full text-sm">
              <thead>
                <tr class="border-b border-gray-200 text-left text-xs font-semibold uppercase tracking-wide text-gray-500">
                  <th class="px-4 py-2">#</th>
                  <th class="px-2 py-2">Team</th>
                </tr>
              </thead>
              <tbody>
                {#each groupTeamList as item, i}
                  <tr
                    class="cursor-pointer border-b border-gray-100 {i === selected_team_index ? 'bg-blue-50' : 'hover:bg-white'}"
                    onclick={() => (selected_team_index = i)}
                  >
                    <td class="px-4 py-2 font-medium text-gray-500">{i + 1}</td>
                    <td class="px-2 py-2">
                      <div class="font-medium text-gray-900">{item.team.name}</div>
                      <div class="text-xs text-gray-400">{item.org_name}</div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        {/if}
      </div>

      <!-- Placeholders reference -->
      <div class="border-t border-gray-200 px-4 py-3">
        <p class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-500">Platzhalter</p>
        <div class="space-y-0.5">
          {#each placeholders as p}
            <div class="flex items-baseline gap-2">
              <code class="shrink-0 rounded bg-gray-200 px-1 text-xs text-blue-700">{"{" + p.key + "}"}</code>
              <span class="text-xs text-gray-500">{p.label}</span>
            </div>
          {/each}
        </div>
      </div>

      <!-- Vorausdruck -->
      <div class="border-t border-gray-200 px-4 py-3">
        <p class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-500">Vorausdruck</p>
        {#if source_mode === "teams"}
          <p class="text-xs text-gray-400 italic">Immer aktiv im Teamliste-Modus</p>
        {:else}
          <label class="flex cursor-pointer items-center gap-2">
            <input
              type="checkbox"
              class="h-3.5 w-3.5 rounded border-gray-300 text-blue-600"
              bind:checked={vorausdruck}
            />
            <span class="text-xs text-gray-600">Platz und Punkte ausblenden</span>
          </label>
        {/if}
      </div>

      <!-- Seitenauswahl -->
      <div class="border-t border-gray-200 px-4 py-3">
        <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">
          Seitenauswahl
        </label>
        <input
          type="text"
          bind:value={subset_expr}
          placeholder="z. B. 1-3 oder 1,3,5"
          class="w-full rounded border border-gray-300 bg-white px-2 py-1.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
        />
        <p class="mt-1 text-xs text-gray-400">
          {#if subset_expr.trim()}
            {activeRows.length} von {allRows.length} Teams ausgewählt
          {:else}
            Leer = alle {allRows.length} Teams
          {/if}
        </p>
      </div>

      <!-- Generate button -->
      <div class="border-t border-gray-200 p-4">
        <Button
          class="w-full"
          disabled={generating || activeRows.length === 0}
          onclick={generatePDF}
        >
          <FilePdfOutline class="mr-2 h-4 w-4" />
          {generating ? "Generiere…" : `${activeRows.length} Urkunden generieren`}
        </Button>
      </div>
    </div>

    <!-- Preview -->
    <div class="flex min-h-0 min-w-0 flex-1 flex-col">
      <div class="min-h-0 flex-1" bind:this={viewerContainer}></div>
      <div class="flex shrink-0 justify-center border-t border-gray-200 bg-gray-50 py-2">
        <Button size="sm" color="alternative" onclick={printCurrent}>Drucken</Button>
      </div>
    </div>
  </div>
</div>
