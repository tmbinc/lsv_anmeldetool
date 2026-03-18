<script lang="ts">
  import { page } from "$app/state";
  import { onMount, onDestroy } from "svelte";
  import FetchErrors from "../../../../FetchErrors.svelte";
  import { getTemplate, setTemplate } from "../../../../../api/api";
  import { Button } from "flowbite-svelte";
  import { CheckOutline, ArrowLeftOutline } from "flowbite-svelte-icons";
  import type { Template } from "@pdfme/common";
  import { Designer } from "@pdfme/ui";
  import { BLANK_A4_PDF } from "@pdfme/common";
  import { getFontsData, plugins, placeholders } from "$lib/urkunde";

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

  const VARIANTS: { value: string; label: string }[] = [
    { value: "main",          label: "Standard" },
    { value: "no_background", label: "Ohne Hintergrund" },
  ];

  let template_variant = $state("main");

  type LayerEntry = { name: string; type: string };

  let designer: Designer | null = null;
  let designerContainer: HTMLDivElement;
  let fetch_errors: FetchErrors;
  let saved = $state(false);
  let saveTimer: ReturnType<typeof setTimeout>;
  let copyFeedback = $state(false);
  let pasteFeedback = $state(false);
  let layers: LayerEntry[] = $state([]);
  let ignoreBorder = $state(false);
  let savedPadding: [number, number, number, number] | null = null;

  function setIgnoreBorder(ignore: boolean) {
    if (!designer) return;
    const tmpl = designer.getTemplate();
    const bp = tmpl.basePdf as { width: number; height: number; padding: [number, number, number, number] };
    if (typeof bp !== "object" || !("padding" in bp)) return; // CustomPdf — nothing to do

    if (ignore) {
      savedPadding = [...bp.padding] as [number, number, number, number];
      designer.updateTemplate({ ...tmpl, basePdf: { ...bp, padding: [0, 0, 0, 0] } });
    } else if (savedPadding) {
      const current = designer.getTemplate();
      const cbp = current.basePdf as { width: number; height: number; padding: [number, number, number, number] };
      designer.updateTemplate({ ...current, basePdf: { ...cbp, padding: savedPadding! } });
      savedPadding = null;
    }
  }

  function syncLayers(tmpl: Template) {
    // Display in reverse: index 0 = topmost (last in array)
    layers = [...(tmpl.schemas[0] ?? [])].reverse().map((s) => ({
      name: s.name,
      type: s.type as string,
    }));
  }

  const TYPE_LABEL: Record<string, string> = {
    text: "T", multiVariableText: "Tv", image: "Img", urlImage: "URL",
    svg: "SVG", table: "Tab", line: "—", rectangle: "▭", ellipse: "◯",
    qrcode: "QR",
  };

  function moveLayer(visualIndex: number, direction: "up" | "down") {
    if (!designer) return;
    const tmpl = designer.getTemplate();
    const schemas = [...(tmpl.schemas[0] ?? [])];
    const n = schemas.length;
    // visual index 0 = schemas[n-1]; "up" raises z = moves toward end of array
    const ai = n - 1 - visualIndex;
    const swapWith = direction === "up" ? ai + 1 : ai - 1;
    if (swapWith < 0 || swapWith >= n) return;
    [schemas[ai], schemas[swapWith]] = [schemas[swapWith], schemas[ai]];
    designer.updateTemplate({ ...tmpl, schemas: [schemas, ...tmpl.schemas.slice(1)] });
  }

  async function copyTemplate() {
    if (!designer) return;
    await navigator.clipboard.writeText(JSON.stringify(designer.getTemplate()));
    copyFeedback = true;
    setTimeout(() => (copyFeedback = false), 2000);
  }

  async function pasteTemplate() {
    const text = await navigator.clipboard.readText();
    try {
      const tmpl = JSON.parse(text);
      designer?.updateTemplate(tmpl);
      pasteFeedback = true;
      setTimeout(() => (pasteFeedback = false), 2000);
    } catch {
      alert("Ungültige Vorlage in der Zwischenablage.");
    }
  }

  function loadVariant(variant: string) {
    getTemplate({ event: event_id, template_type, template_variant: variant }).resp.subscribe((resp) => {
      if (resp?.ok && resp.data) {
        try {
          designer?.updateTemplate(JSON.parse(resp.data));
        } catch (e) {
          console.error("Failed to parse saved template", e);
        }
      } else {
        // No template saved yet for this variant — reset to blank
        designer?.updateTemplate(fallbackTemplate);
      }
    });
    saved = false;
    ignoreBorder = false;
    savedPadding = null;
  }

  function switchVariant(variant: string) {
    template_variant = variant;
    loadVariant(variant);
  }

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

    designer.onChangeTemplate(syncLayers);
    syncLayers(fallbackTemplate);

    loadVariant(template_variant);
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
    <select
      value={template_variant}
      onchange={(e) => switchVariant((e.currentTarget as HTMLSelectElement).value)}
      class="rounded border border-gray-300 bg-white px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
    >
      {#each VARIANTS as v}
        <option value={v.value}>{v.label}</option>
      {/each}
    </select>
    <div class="ml-auto flex items-center gap-2">
      {#if copyFeedback}
        <span class="text-sm text-green-600">Kopiert ✓</span>
      {/if}
      {#if pasteFeedback}
        <span class="text-sm text-green-600">Eingefügt ✓</span>
      {/if}
      {#if saved}
        <span class="flex items-center gap-1 text-sm text-green-600">
          <CheckOutline class="h-4 w-4" /> Gespeichert
        </span>
      {/if}
      <Button size="sm" color="alternative" onclick={copyTemplate}>Kopieren</Button>
      <Button size="sm" color="alternative" onclick={pasteTemplate}>Einfügen</Button>
      <Button size="sm" onclick={save}>Speichern</Button>
    </div>
  </div>

  <!-- Body: designer + placeholder sidebar -->
  <div class="flex min-h-0 flex-1">
    <div class="min-h-0 min-w-0 flex-1" bind:this={designerContainer}></div>

    <!-- Right sidebar: layers + placeholders -->
    <div class="flex w-56 shrink-0 flex-col overflow-y-auto border-l border-gray-200 bg-gray-50">

      <!-- Layer panel -->
      <div class="border-b border-gray-200 px-3 py-3">
        <p class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-500">Ebenen</p>
        {#if layers.length === 0}
          <p class="text-xs text-gray-400">Keine Elemente</p>
        {:else}
          <div class="flex flex-col gap-0.5">
            {#each layers as layer, i}
              <div class="flex items-center gap-1 rounded px-1 py-0.5 hover:bg-gray-100">
                <span class="w-7 shrink-0 rounded bg-gray-200 px-1 text-center text-[10px] font-bold text-gray-500">
                  {TYPE_LABEL[layer.type] ?? layer.type}
                </span>
                <span class="min-w-0 flex-1 truncate text-xs text-gray-700" title={layer.name}>
                  {layer.name}
                </span>
                <button
                  onclick={() => moveLayer(i, "up")}
                  disabled={i === 0}
                  class="flex h-5 w-5 shrink-0 items-center justify-center rounded text-gray-400 hover:bg-gray-200 hover:text-gray-700 disabled:opacity-20"
                  title="Nach vorne"
                >▲</button>
                <button
                  onclick={() => moveLayer(i, "down")}
                  disabled={i === layers.length - 1}
                  class="flex h-5 w-5 shrink-0 items-center justify-center rounded text-gray-400 hover:bg-gray-200 hover:text-gray-700 disabled:opacity-20"
                  title="Nach hinten"
                >▼</button>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Border toggle -->
      <div class="border-b border-gray-200 px-3 py-3">
        <label class="flex cursor-pointer items-center gap-2 select-none">
          <input
            type="checkbox"
            bind:checked={ignoreBorder}
            onchange={() => setIgnoreBorder(ignoreBorder)}
            class="h-3.5 w-3.5 rounded border-gray-300 accent-red-500"
          />
          <span class="text-xs text-gray-600">Rand ignorieren</span>
        </label>
        {#if ignoreBorder}
          <p class="mt-1 text-[10px] text-red-500">Elemente können über den Seitenrand hinaus platziert werden.</p>
        {/if}
      </div>

      <!-- Placeholder reference -->
      <div class="flex flex-col gap-1 px-3 py-3">
        <p class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-500">Platzhalter</p>
        {#each placeholders as p}
          <div class="flex flex-col gap-0.5">
            <code class="rounded bg-gray-200 px-1 text-xs text-blue-700">{"{" + p.key + "}"}</code>
            <span class="text-xs text-gray-400">{p.label}</span>
          </div>
        {/each}
      </div>

    </div>
  </div>
</div>
