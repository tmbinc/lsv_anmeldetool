<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { getTeam, getResults, getTemplate } from "../../../api/api";
  import { generate } from "@pdfme/generator";
  import { BLANK_A4_PDF } from "@pdfme/common";
  import type { Template } from "@pdfme/common";
  import { getFontsData, plugins, buildDataMap, buildInput } from "$lib/urkunde";

  const team_id = page.params.team_id || "";

  let status = $state<"loading" | "generating" | "done" | "error">("loading");
  let error_msg = $state("");

  const fallbackTemplate: Template = { basePdf: BLANK_A4_PDF, schemas: [[]] };

  onMount(async () => {
    // 1. Fetch team
    const team_resp = await getTeam({ team: team_id }).result;
    if (!team_resp.ok) {
      error_msg = "Team nicht gefunden.";
      status = "error";
      return;
    }
    const team = team_resp.data;
    const event_id = team.event;
    const group_id = team.group_id;

    // 2. Fetch results + template in parallel
    const [results_resp, template_resp] = await Promise.all([
      getResults({ event: event_id }).result,
      getTemplate({ event: event_id, template_type: "urkunde", template_variant: "main" }).result,
    ]);

    if (!results_resp.ok) {
      error_msg = "Ergebnisse konnten nicht geladen werden.";
      status = "error";
      return;
    }

    // 3. Find matching result entry
    const entry = results_resp.data.results.find(
      (r) => r.group === group_id && r.team === team.name,
    );
    if (!entry) {
      error_msg = "Kein Ergebniseintrag für dieses Team gefunden.";
      status = "error";
      return;
    }

    const group_name =
      results_resp.data.groups.find((g) => g.id === group_id)?.name ?? "";
    const event_name = results_resp.data.event_name;

    // 4. Parse template
    let tmpl: Template = fallbackTemplate;
    if (template_resp.ok && template_resp.data) {
      try {
        tmpl = JSON.parse(template_resp.data);
      } catch {
        console.error("Failed to parse template");
      }
    }

    // 5. Build input data
    const data = buildDataMap(entry, group_name, event_name);

    status = "generating";

    try {
      const inputs = [buildInput(tmpl, data)];
      const pdf = await generate({
        template: tmpl,
        inputs,
        plugins,
        options: { font: getFontsData() },
      });
      const blob = new Blob([pdf.buffer], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);
      // Replace the current page with the PDF so it renders inline / triggers download
      window.location.href = url;
      status = "done";
    } catch (e) {
      console.error("PDF generation failed", e);
      error_msg = "PDF-Generierung fehlgeschlagen.";
      status = "error";
    }
  });
</script>

<div class="flex h-screen items-center justify-center bg-white">
  {#if status === "loading" || status === "generating"}
    <div class="text-center text-gray-500">
      <div class="mb-3 text-4xl">📄</div>
      <p class="text-sm font-medium">
        {status === "loading" ? "Lade Daten…" : "Generiere PDF…"}
      </p>
    </div>
  {:else if status === "error"}
    <div class="text-center text-red-600">
      <div class="mb-3 text-4xl">⚠️</div>
      <p class="text-sm font-medium">{error_msg}</p>
    </div>
  {/if}
</div>
