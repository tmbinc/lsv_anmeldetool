<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { getEvent } from "../../../../../api/api";
  import { ArrowLeftOutline } from "flowbite-svelte-icons";

  const event_id = page.params.event_id || "";
  let event_name = $state("");

  onMount(() => {
    getEvent({ event: event_id }).resp.subscribe((resp) => {
      if (resp?.ok) event_name = resp.data.name;
    });
  });

  const adminLinks = [
    { href: `/admin/event/${event_id}/run/registration/`, label: "Registrierung" },
    { href: `/admin/event/${event_id}/run/timetable/`, label: "Zeitplanung" },
    { href: `/admin/event/${event_id}/run/results/`, label: "Ergebnisse" },
    { href: `/admin/event/${event_id}/run/urkunde/`, label: "Urkunden" },
    { href: `/admin/event/${event_id}/pdftemplate/`, label: "PDF-Vorlage" },
  ];

  const beamerLinks = [
    { href: `/event/${event_id}/paarvis/`, label: "Paarungsansicht" },
    { href: `/event/${event_id}/results/`, label: "Ergebnisse" },
    { href: `/event/${event_id}/timetable_pairing/`, label: "Zeitplan" },
    { href: `/admin/event/${event_id}/run/ceremony/`, label: "Siegerehrung" },
  ];

  const current = $derived(page.url.pathname);
</script>

<div class="flex min-h-screen flex-col">
  <header class="flex items-center gap-3 border-b border-gray-200 bg-white px-4 py-2 text-sm shadow-sm">
    <a
      href="/admin/event/{event_id}/"
      class="flex items-center gap-1 text-gray-500 hover:text-gray-800"
    >
      <ArrowLeftOutline class="h-4 w-4" />
      <span class="hidden sm:inline">Administration</span>
    </a>

    <a href="/admin/event/{event_id}/run/" class="font-semibold text-gray-800 hover:text-blue-600">
      {event_name || "…"}
    </a>

    <div class="mx-2 h-5 w-px bg-gray-300"></div>

    <nav class="flex items-center gap-1">
      {#each adminLinks as link}
        <a
          href={link.href}
          class="rounded px-2 py-1 transition-colors {current === link.href
            ? 'bg-blue-100 text-blue-700 font-medium'
            : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'}"
        >{link.label}</a>
      {/each}
    </nav>

    <div class="mx-2 h-5 w-px bg-gray-300"></div>

    <span class="text-xs font-medium uppercase tracking-wide text-gray-400">Beamer</span>
    <nav class="flex items-center gap-1">
      {#each beamerLinks as link}
        <a
          href={link.href}
          class="rounded px-2 py-1 transition-colors {current === link.href
            ? 'bg-purple-100 text-purple-700 font-medium'
            : 'text-gray-400 hover:bg-gray-100 hover:text-gray-700'}"
        >{link.label}</a>
      {/each}
    </nav>
  </header>

  <div class="flex-1">
    <slot />
  </div>
</div>
