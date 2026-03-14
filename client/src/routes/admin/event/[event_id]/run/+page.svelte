<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import {
    getEvent,
    getGroupsForEvent,
    getTeamCountForEvent,
    getResults,
    getPairings,
    type Group,
    type ResultEntry,
    type PairingEntry,
  } from "../../../../../api/api";
  import FetchErrors from "../../../../FetchErrors.svelte";
  import {
    ClipboardListOutline,
    CalendarMonthOutline,
    ClipboardCheckOutline,
    AwardOutline,
    FilePdfOutline,
    CogOutline,
    DesktopPcOutline,
    ChartMixedOutline,
    CalendarWeekOutline,
    StarOutline,
    UsersGroupOutline,
    GridOutline,
    PlusOutline,
    TableColumnOutline,
  } from "flowbite-svelte-icons";

  const event_id = page.params.event_id || "";

  let event_name = $state("");
  let event_date = $state<string | null>(null);
  let groups: Group[] = $state([]);
  let team_count = $state<number | null>(null);
  let results: ResultEntry[] = $state([]);
  let pairings: PairingEntry[] = $state([]);
  let fetch_errors: FetchErrors;

  // Latest result round per group id
  const latestResultRound = $derived(() => {
    const map = new Map<string, number>();
    for (const r of results) {
      const cur = map.get(r.group) ?? 0;
      if (r.round > cur) map.set(r.group, r.round);
    }
    return map;
  });

  // Latest pairing round per group id
  const latestPairingRound = $derived(() => {
    const map = new Map<string, number>();
    for (const p of pairings) {
      const cur = map.get(p.group) ?? 0;
      if (p.round > cur) map.set(p.group, p.round);
    }
    return map;
  });

  onMount(async () => {
    getEvent({ event: event_id }).resp.subscribe((resp) => {
      if (resp?.ok) {
        event_name = resp.data.name;
        event_date = resp.data.begin ?? null;
      }
    });

    const resp_groups = await getGroupsForEvent({ event: event_id }).result;
    if (resp_groups.ok) {
      groups = resp_groups.data;
    } else {
      fetch_errors.check(resp_groups);
    }

    const resp_teams = await getTeamCountForEvent({ event: event_id }).result;
    if (resp_teams.ok) {
      team_count = resp_teams.data;
    }

    const resp_results = await getResults({ event: event_id }).result;
    if (resp_results.ok) {
      results = resp_results.data.results;
    }

    const resp_pairings = await getPairings({ event: event_id }).result;
    if (resp_pairings.ok) {
      pairings = resp_pairings.data.pairings;
    }
  });

  const adminCards = [
    {
      href: `/admin/event/${event_id}/run/registration/`,
      label: "Registrierung",
      description: "Anwesenheit und Einchecken der Teams verwalten",
      icon: ClipboardListOutline,
      color: "blue",
    },
    {
      href: `/admin/event/${event_id}/run/timetable/`,
      label: "Zeitplanung",
      description: "Rundenzeiten und Spielplan festlegen",
      icon: CalendarMonthOutline,
      color: "blue",
    },
    {
      href: `/admin/event/${event_id}/run/results/`,
      label: "Ergebnisse melden",
      description: "Spielergebnisse der einzelnen Runden eintragen",
      icon: ClipboardCheckOutline,
      color: "blue",
    },
    {
      href: `/admin/event/${event_id}/run/urkunde/`,
      label: "Urkunden",
      description: "Urkunden für alle Teilnehmer generieren",
      icon: AwardOutline,
      color: "blue",
    },
    {
      href: `/admin/event/${event_id}/pdftemplate/`,
      label: "PDF-Vorlage",
      description: "Urkundenvorlage bearbeiten und speichern",
      icon: FilePdfOutline,
      color: "blue",
    },
    {
      href: `/admin/event/${event_id}/`,
      label: "Turnier-Administration",
      description: "Zurück zur allgemeinen Turnierverwaltung",
      icon: CogOutline,
      color: "gray",
    },
  ];

  const beamerCards = [
    {
      href: `/event/${event_id}/paarvis/`,
      label: "Paarungsansicht",
      description: "Aktuelle Paarungen auf dem Beamer anzeigen",
      icon: DesktopPcOutline,
      color: "purple",
    },
    {
      href: `/event/${event_id}/results/`,
      label: "Ergebnisse",
      description: "Rangliste und Ergebnisse für das Publikum",
      icon: ChartMixedOutline,
      color: "purple",
    },
    {
      href: `/event/${event_id}/timetable_pairing/`,
      label: "Zeitplan & Paarungen",
      description: "Kombinierte Ansicht für den Aushang",
      icon: CalendarWeekOutline,
      color: "purple",
    },
    {
      href: `/admin/event/${event_id}/run/ceremony/`,
      label: "Siegerehrung",
      description: "Animierte Siegerehrung für die Abschlussveranstaltung",
      icon: StarOutline,
      color: "purple",
    },
  ];

  const colorClasses = {
    blue: {
      card: "border-blue-100 hover:border-blue-300 hover:bg-blue-50",
      icon: "bg-blue-100 text-blue-600",
    },
    purple: {
      card: "border-purple-100 hover:border-purple-300 hover:bg-purple-50",
      icon: "bg-purple-100 text-purple-600",
    },
    gray: {
      card: "border-gray-200 hover:border-gray-400 hover:bg-gray-50",
      icon: "bg-gray-100 text-gray-500",
    },
  };
</script>

<FetchErrors bind:this={fetch_errors} />

<main class="mx-auto max-w-5xl px-6 py-10">
  <!-- Header -->
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-gray-900">{event_name || "…"}</h1>
    {#if event_date}
      <p class="mt-1 text-sm text-gray-500">{event_date}</p>
    {/if}
  </div>

  <!-- Stats -->
  <div class="mb-10 grid grid-cols-2 gap-4 sm:grid-cols-3">
    <div class="rounded-xl border border-gray-200 bg-white px-5 py-4 shadow-sm">
      <div class="flex items-center gap-3">
        <div class="rounded-lg bg-blue-100 p-2 text-blue-600">
          <GridOutline class="h-5 w-5" />
        </div>
        <div>
          <div class="text-2xl font-bold text-gray-900">{groups.filter(g => !g.replacement).length}</div>
          <div class="text-xs text-gray-500">Gruppen</div>
        </div>
      </div>
    </div>

    <div class="rounded-xl border border-gray-200 bg-white px-5 py-4 shadow-sm">
      <div class="flex items-center gap-3">
        <div class="rounded-lg bg-blue-100 p-2 text-blue-600">
          <UsersGroupOutline class="h-5 w-5" />
        </div>
        <div>
          <div class="text-2xl font-bold text-gray-900">{team_count ?? "…"}</div>
          <div class="text-xs text-gray-500">Teams</div>
        </div>
      </div>
    </div>
  </div>

  <!-- Groups / results status -->
  {#if groups.filter(g => !g.replacement).length > 0}
  <section class="mb-10">
    <h2 class="mb-4 text-xs font-semibold uppercase tracking-widest text-gray-400">Gruppen & Ergebnisse</h2>
    <div class="flex flex-col gap-2">
      {#each groups.filter(g => !g.replacement) as group}
        {@const resultRound = latestResultRound().get(group.id) ?? 0}
        {@const pairingRound = latestPairingRound().get(group.id) ?? 0}
        <div class="flex items-center gap-3 rounded-xl border border-gray-200 bg-white px-4 py-3 shadow-sm">
          <div class="h-3 w-3 shrink-0 rounded-full" style="background-color: {group.color};"></div>
          <span class="flex-1 font-medium text-gray-900">{group.name}</span>

          <!-- Pairing round -->
          {#if pairingRound > 0}
            <a href="/event/{event_id}/pairings" class="rounded-full bg-blue-50 px-2.5 py-0.5 text-xs font-medium text-blue-700 hover:bg-blue-100 transition-colors">
              Paarung Runde {pairingRound}
            </a>
          {:else}
            <span class="rounded-full bg-gray-50 px-2.5 py-0.5 text-xs text-gray-300">Keine Paarungen</span>
          {/if}

          <!-- Result round -->
          {#if resultRound > 0}
            <a href="/event/{event_id}/results" class="rounded-full bg-green-50 px-2.5 py-0.5 text-xs font-medium text-green-700 hover:bg-green-100 transition-colors">
              Ergebnisse Runde {resultRound}
            </a>
          {:else}
            <span class="rounded-full bg-gray-50 px-2.5 py-0.5 text-xs text-gray-300">Keine Ergebnisse</span>
          {/if}

          <a
            href="/admin/event/{event_id}/{group.id}/"
            class="flex items-center gap-1 rounded-lg border border-gray-200 px-3 py-1 text-xs font-medium text-gray-600 hover:bg-gray-50 transition-colors"
          >
            <PlusOutline class="h-3.5 w-3.5" />
            Eintragen
          </a>
        </div>
      {/each}
    </div>
  </section>
  {/if}

  <!-- Admin tools -->
  <section class="mb-10">
    <h2 class="mb-4 text-xs font-semibold uppercase tracking-widest text-gray-400">Administration</h2>
    <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
      {#each adminCards as card}
        {@const colors = colorClasses[card.color as keyof typeof colorClasses]}
        <a
          href={card.href}
          class="group flex items-start gap-4 rounded-xl border bg-white p-4 shadow-sm transition-all {colors.card}"
        >
          <div class="mt-0.5 shrink-0 rounded-lg p-2 {colors.icon}">
            <card.icon class="h-5 w-5" />
          </div>
          <div>
            <div class="font-medium text-gray-900 group-hover:text-gray-900">{card.label}</div>
            <div class="mt-0.5 text-sm text-gray-500">{card.description}</div>
          </div>
        </a>
      {/each}
    </div>
  </section>

  <!-- Public links -->
  <section class="mb-10">
    <h2 class="mb-4 text-xs font-semibold uppercase tracking-widest text-gray-400">Öffentliche Seiten</h2>
    <div class="grid gap-3 sm:grid-cols-3">
      <a
        href="/event/{event_id}/timetable"
        target="_blank"
        class="group flex items-center gap-3 rounded-xl border border-gray-100 bg-white p-4 shadow-sm transition-all hover:border-gray-300 hover:bg-gray-50"
      >
        <div class="shrink-0 rounded-lg bg-gray-100 p-2 text-gray-500">
          <CalendarWeekOutline class="h-5 w-5" />
        </div>
        <div>
          <div class="font-medium text-gray-900">Zeitplan</div>
          <div class="mt-0.5 text-xs text-gray-400">/event/…/timetable</div>
        </div>
      </a>
      <a
        href="/event/{event_id}/pairings"
        target="_blank"
        class="group flex items-center gap-3 rounded-xl border border-gray-100 bg-white p-4 shadow-sm transition-all hover:border-gray-300 hover:bg-gray-50"
      >
        <div class="shrink-0 rounded-lg bg-gray-100 p-2 text-gray-500">
          <TableColumnOutline class="h-5 w-5" />
        </div>
        <div>
          <div class="font-medium text-gray-900">Paarungen</div>
          <div class="mt-0.5 text-xs text-gray-400">/event/…/pairings</div>
        </div>
      </a>
      <a
        href="/event/{event_id}/results"
        target="_blank"
        class="group flex items-center gap-3 rounded-xl border border-gray-100 bg-white p-4 shadow-sm transition-all hover:border-gray-300 hover:bg-gray-50"
      >
        <div class="shrink-0 rounded-lg bg-gray-100 p-2 text-gray-500">
          <ChartMixedOutline class="h-5 w-5" />
        </div>
        <div>
          <div class="font-medium text-gray-900">Ergebnisse</div>
          <div class="mt-0.5 text-xs text-gray-400">/event/…/results</div>
        </div>
      </a>
    </div>
  </section>

  <!-- Beamer pages -->
  <section>
    <h2 class="mb-4 text-xs font-semibold uppercase tracking-widest text-gray-400">Beamer / Anzeige</h2>
    <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
      {#each beamerCards as card}
        {@const colors = colorClasses[card.color as keyof typeof colorClasses]}
        <a
          href={card.href}
          class="group flex flex-col gap-3 rounded-xl border bg-white p-4 shadow-sm transition-all {colors.card}"
        >
          <div class="shrink-0 self-start rounded-lg p-2 {colors.icon}">
            <card.icon class="h-5 w-5" />
          </div>
          <div>
            <div class="font-medium text-gray-900">{card.label}</div>
            <div class="mt-0.5 text-sm text-gray-500">{card.description}</div>
          </div>
        </a>
      {/each}
    </div>
  </section>
</main>
