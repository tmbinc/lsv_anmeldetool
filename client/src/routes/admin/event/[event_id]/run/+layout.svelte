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
    { href: `/admin/event/${event_id}/run/timetable/`,    label: "Zeitplanung"   },
    { href: `/admin/event/${event_id}/run/results/`,      label: "Ergebnisse"    },
    { href: `/admin/event/${event_id}/run/urkunde/`,      label: "Urkunden"      },
    { href: `/admin/event/${event_id}/pdftemplate/`,      label: "PDF-Vorlage"   },
  ];

  const beamerLinks = [
    { href: `/event/${event_id}/paarvis/`,              label: "Paarungsansicht" },
    { href: `/event/${event_id}/timetable_results/`,    label: "Ergebnisse"      },
    { href: `/event/${event_id}/timetable_pairing/`,    label: "Zeitplan"        },
    { href: `/admin/event/${event_id}/run/ceremony/`,   label: "Siegerehrung"    },
  ];

  const current = $derived(page.url.pathname);
</script>

<div class="flex min-h-screen flex-col">
  <header class="sticky top-0 z-30 border-b border-gray-200 bg-white shadow-sm">

    <!-- Row 1: breadcrumb + event name -->
    <div class="flex items-center gap-2 border-b border-gray-100 px-4 py-2">
      <a
        href="/admin/event/{event_id}/"
        class="flex shrink-0 items-center gap-1 text-sm text-gray-500 hover:text-gray-800 transition-colors"
      >
        <ArrowLeftOutline class="h-4 w-4" />
        <span class="hidden sm:inline">Administration</span>
      </a>
      <span class="text-gray-300">/</span>
      <a
        href="/admin/event/{event_id}/run/"
        class="truncate text-sm font-semibold text-gray-800 hover:text-blue-600 transition-colors"
      >{event_name || "…"}</a>
    </div>

    <!-- Row 2: admin tab links (scrollable on mobile) -->
    <div class="flex items-center gap-0.5 overflow-x-auto px-3 py-1.5 scrollbar-none">
      {#each adminLinks as link}
        <a
          href={link.href}
          class="shrink-0 rounded-md px-3 py-1.5 text-sm font-medium whitespace-nowrap transition-colors
            {current === link.href
              ? 'bg-blue-100 text-blue-700'
              : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'}"
        >{link.label}</a>
      {/each}

      <div class="mx-2 h-4 w-px shrink-0 bg-gray-200"></div>

      <span class="shrink-0 text-xs font-semibold uppercase tracking-widest text-gray-300 pr-1">Beamer</span>
      {#each beamerLinks as link}
        <a
          href={link.href}
          target="_blank"
          class="shrink-0 rounded-md px-3 py-1.5 text-sm font-medium whitespace-nowrap transition-colors
            {current === link.href
              ? 'bg-purple-100 text-purple-700'
              : 'text-gray-400 hover:bg-gray-100 hover:text-gray-700'}"
        >{link.label}</a>
      {/each}
    </div>

  </header>

  <div class="flex-1">
    <slot />
  </div>
</div>

<style>
  .scrollbar-none {
    scrollbar-width: none;
  }
  .scrollbar-none::-webkit-scrollbar {
    display: none;
  }
</style>
