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
  const template_variant = "main";

  let designer: Designer | null = null;
  let designerContainer: HTMLDivElement;
  let fetch_errors: FetchErrors;
  let saved = $state(false);
  let saveTimer: ReturnType<typeof setTimeout>;
  let copyFeedback = $state(false);
  let pasteFeedback = $state(false);

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

    <!-- Placeholder reference -->
    <div class="flex w-56 shrink-0 flex-col gap-1 border-l border-gray-200 bg-gray-50 px-4 py-4">
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
