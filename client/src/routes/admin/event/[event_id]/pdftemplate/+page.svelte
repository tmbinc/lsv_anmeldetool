<script lang="ts">
  import { page } from "$app/state";
  import { onMount, onDestroy } from "svelte";
  import FetchErrors from "../../../../FetchErrors.svelte";
  import { getTemplate, setTemplate } from "../../../../../api/api";
  import { Button } from "flowbite-svelte";
  import { CheckOutline, ArrowLeftOutline } from "flowbite-svelte-icons";
  import type { Font, Template } from "@pdfme/common";
  import { Designer } from "@pdfme/ui";
  import { BLANK_A4_PDF, getDefaultFont } from "@pdfme/common";
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
    schemas: [
      [
        {
          name: "title",
          type: "text",
          position: { x: 20, y: 20 },
          width: 170,
          height: 20,
        },
      ],
    ],
  };

  const event_id = page.params.event_id || "";
  const template_type = "urkunde";
  const template_variant = "main";

  let designer: Designer | null = null;
  let designerContainer: HTMLDivElement;
  let fetch_errors: FetchErrors;
  let saved = $state(false);
  let saveTimer: ReturnType<typeof setTimeout>;

  onMount(() => {
    designer = new Designer({
      domContainer: designerContainer,
      template: fallbackTemplate,
      plugins,
      options: {
        zoomLevel: 1,
        sidebarOpen: true,
        font: getFontsData(),
      },
    });

    getTemplate({ event: event_id, template_type, template_variant }).resp.subscribe((resp) => {
      if (resp?.ok && resp.data) {
        try {
          designer?.updateTemplate(JSON.parse(resp.data));
        } catch (e) {
          console.error("Failed to parse saved template", e);
        }
      }
    });
  });

  onDestroy(() => {
    designer?.destroy();
    clearTimeout(saveTimer);
  });

  function save() {
    if (!designer) return;
    const content = JSON.stringify(designer.getTemplate());
    setTemplate({ event: event_id, template_type, template_variant, content });
    saved = true;
    clearTimeout(saveTimer);
    saveTimer = setTimeout(() => (saved = false), 2000);
  }
</script>

<FetchErrors bind:this={fetch_errors} />

<div class="flex h-screen flex-col bg-white">
  <!-- Top bar -->
  <div class="flex shrink-0 items-center gap-3 border-b border-gray-200 bg-gray-50 px-4 py-2">
    <a
      href="/admin/event/{event_id}/run/"
      class="flex items-center gap-1 text-sm text-gray-500 hover:text-gray-800"
    >
      <ArrowLeftOutline class="h-4 w-4" />
      Zurück
    </a>
    <div class="mx-1 h-4 w-px bg-gray-300"></div>
    <span class="text-sm font-semibold text-gray-700">PDF-Vorlage: Urkunde</span>
    <div class="ml-auto flex items-center gap-2">
      {#if saved}
        <span class="flex items-center gap-1 text-sm text-green-600">
          <CheckOutline class="h-4 w-4" /> Gespeichert
        </span>
      {/if}
      <Button size="sm" onclick={save}>Speichern</Button>
    </div>
  </div>

  <!-- Body: designer + placeholder sidebar -->
  <div class="flex min-h-0 flex-1">
    <div class="min-h-0 min-w-0 flex-1" bind:this={designerContainer}></div>

    <!-- Placeholder reference -->
    <div class="flex w-56 shrink-0 flex-col gap-1 border-l border-gray-200 bg-gray-50 px-4 py-4">
      <p class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-500">Platzhalter</p>
      {#each [
        { key: "team",          label: "Team-Name" },
        { key: "team_name",     label: "Team-Name (Alias)" },
        { key: "org",           label: "Schule / Organisation" },
        { key: "team_genus",    label: "Genus (der/die/das)" },
        { key: "rank",          label: "Platz (1, 2, 3 …)" },
        { key: "points_team",   label: "Mannschaftspunkte" },
        { key: "points_player", label: "Einzelpunkte" },
        { key: "tie",           label: "Feinwertung" },
        { key: "group",         label: "Gruppe" },
        { key: "event",         label: "Veranstaltungsname" },
      ] as p}
        <div class="flex flex-col gap-0.5">
          <code class="rounded bg-gray-200 px-1 text-xs text-blue-700">{"{" + p.key + "}"}</code>
          <span class="text-xs text-gray-400">{p.label}</span>
        </div>
      {/each}
    </div>
  </div>
</div>
