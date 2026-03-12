<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    getResults,
    getTemplate,
    type Group,
    type ResultEntry,
  } from "../../../../../../api/api";
  import { page } from "$app/state";
  import { generate } from "@pdfme/generator";
  import { Viewer } from "@pdfme/ui";
  import { BLANK_A4_PDF, getDefaultFont } from "@pdfme/common";
  import type { Font, Template } from "@pdfme/common";
  import {
    text,
    multiVariableText,
    image,
    svg,
    table,
    barcodes,
    line,
    rectangle,
    ellipse,
  } from "@pdfme/schemas";
  import { Button } from "flowbite-svelte";
  import { ArrowLeftOutline, FilePdfOutline } from "flowbite-svelte-icons";

  const event_id = page.params.event_id || "";

  let event_name = $state("");
  let groups_results: Group[] = $state([]);
  let results: ResultEntry[] = $state([]);
  let selected_group = $state("");
  let selected_team_index = $state(0);
  let generating = $state(false);
  let evtSource: EventSource | null = null;

  let viewerContainer: HTMLDivElement;
  let viewer: Viewer | null = $state(null);

  const getFontsData = (): Font => ({
    ...getDefaultFont(),
    "PinyonScript-Regular": {
      fallback: false,
      data: "https://fonts.gstatic.com/s/pinyonscript/v22/6xKpdSJbL9-e9LuoeQiDRQR8aOLQO4bhiDY.ttf",
    },
    NotoSerifJP: {
      fallback: false,
      data: "https://fonts.gstatic.com/s/notoserifjp/v30/xn71YHs72GKoTvER4Gn3b5eMRtWGkp6o7MjQ2bwxOubAILO5wBCU.ttf",
    },
    NotoSansJP: {
      fallback: false,
      data: "https://fonts.gstatic.com/s/notosansjp/v53/-F6jfjtqLzI2JPCgQBnw7HFyzSD-AsregP8VFBEj75vY0rw-oME.ttf",
    },
  });

  const plugins = {
    Text: text,
    MultiVariableText: multiVariableText,
    Image: image,
    SVG: svg,
    Table: table,
    Line: line,
    Rectangle: rectangle,
    Ellipse: ellipse,
    QRCode: barcodes.qrcode,
  };

  const fallbackTemplate: Template = {
    basePdf: BLANK_A4_PDF,
    schemas: [[]],
  };

  let currentTemplate: Template = fallbackTemplate;

  // Available placeholder field names (for both text field names and multiVariableText variables)
  const placeholders = [
    { key: "team",          label: "Team-Name" },
    { key: "team_name",     label: "Team-Name (Alias)" },
    { key: "org",           label: "Schule / Organisation" },
    { key: "team_genus",    label: "Genus des Teams (der/die/das)" },
    { key: "rank",          label: "Platz (1, 2, 3 …)" },
    { key: "points_team",   label: "Mannschaftspunkte" },
    { key: "points_player", label: "Einzelpunkte" },
    { key: "tie",           label: "Feinwertung" },
    { key: "group",         label: "Gruppe" },
    { key: "event",         label: "Veranstaltungsname" },
  ];

  // Flat map of all known variable values for a given entry
  function dataMap(entry: ResultEntry, group_name: string): Record<string, string> {
    const team = entry.team ?? "";
    return {
      team,
      team_name:     team,
      org:           entry.team_org      ?? "",
      team_genus:    ({ f: "die", m: "der", n: "das" })[entry.team_genus ?? ""] ?? entry.team_genus ?? "",
      rank:          entry.rank          != null ? String(entry.rank)          : "",
      points_team:   entry.points_team   != null ? String(entry.points_team)   : "",
      points_player: entry.points_player != null ? String(entry.points_player) : "",
      tie:           entry.tie           != null ? String(entry.tie)           : "",
      group:         group_name,
      event:         event_name,
    };
  }

  // Build a pdfme input record for one entry, driven by the actual template schema.
  // - multiVariableText fields: input value is a JSON string of {variable: value} pairs
  // - text (and other) fields: input value is the data value matching the field name
  function buildInput(template: Template, entry: ResultEntry, group_name: string): Record<string, string> {
    const data = dataMap(entry, group_name);
    const result: Record<string, string> = {};
    for (const page of template.schemas) {
      for (const field of page) {
        if (field.type === "multiVariableText") {
          const vars: Record<string, string> = {};
          for (const v of (field as { variables?: string[] }).variables ?? []) {
            vars[v] = data[v] ?? "";
          }
          result[field.name] = JSON.stringify(vars);
        } else if (field.name in data) {
          result[field.name] = data[field.name];
        }
      }
    }
    return result;
  }

  // Teams for selected group, sorted by rank
  const groupTeams = $derived(
    results
      .filter((r) => r.group === selected_group)
      .sort((a, b) => (a.rank ?? 999) - (b.rank ?? 999))
  );

  const selectedGroupName = $derived(
    groups_results.find((g) => g.id === selected_group)?.name ?? ""
  );

  // Reset selection when group changes
  $effect(() => {
    selected_group;
    selected_team_index = 0;
  });

  // Update viewer when selection or template changes
  $effect(() => {
    if (!viewer) return;
    const entry = groupTeams[selected_team_index] ?? groupTeams[0];
    const inputs = entry ? [buildInput(currentTemplate, entry, selectedGroupName)] : [{}];
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

    const results_request = getResults({ event: event_id });
    results_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        results = resp.data.results;
        groups_results = resp.data.groups.filter((g) => !g.replacement);
        event_name = resp.data.event_name;
        if (!selected_group && groups_results.length > 0) {
          selected_group = groups_results[0].id;
        }
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
    const entry = groupTeams[selected_team_index] ?? groupTeams[0];
    if (!entry) return;
    try {
      const inputs = [buildInput(currentTemplate, entry, selectedGroupName)];
      const pdf = await generate({ template: currentTemplate, inputs, plugins });
      const blob = new Blob([pdf.buffer], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);
      const win = window.open(url);
      win?.addEventListener("load", () => win.print());
    } catch (e) {
      console.error("Print failed", e);
    }
  }

  async function generatePDF() {
    if (!groupTeams.length) return;
    generating = true;
    try {
      const inputs = groupTeams.map((entry) => buildInput(currentTemplate, entry, selectedGroupName));
      const pdf = await generate({ template: currentTemplate, inputs, plugins });
      const blob = new Blob([pdf.buffer], { type: "application/pdf" });
      window.open(URL.createObjectURL(blob));
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

      <!-- Team list -->
      <div class="min-h-0 flex-1 overflow-y-auto">
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

      <!-- Generate button -->
      <div class="border-t border-gray-200 p-4">
        <Button
          class="w-full"
          disabled={generating || groupTeams.length === 0}
          onclick={generatePDF}
        >
          <FilePdfOutline class="mr-2 h-4 w-4" />
          {generating ? "Generiere…" : `${groupTeams.length} Urkunden generieren`}
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
