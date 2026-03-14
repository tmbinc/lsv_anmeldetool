<script lang="ts">
  import { onMount } from "svelte";
  import { listEvents, type Event } from "../api/api";
  import FetchErrors from "./FetchErrors.svelte";

  let events: Event[] = $state([]);
  let fetch_errors: FetchErrors;

  onMount(async () => {
    const resp = await listEvents({}).result;
    if (resp.ok) {
      events = resp.data;
    } else {
      fetch_errors.check(resp);
    }
  });
</script>

<FetchErrors admin={false} bind:this={fetch_errors} />

<div class="mx-auto max-w-2xl px-6 py-12">
  <h1 class="mb-8 text-2xl font-bold text-gray-900">Turnierkalender</h1>

  <div class="flex flex-col gap-3">
    {#each events as event}
      <div class="rounded-xl border border-gray-200 bg-white p-5 shadow-sm">
        <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
          <div>
            <div class="font-semibold text-gray-900">{event.name}</div>
            {#if event.begin}
              <div class="mt-0.5 text-sm text-gray-400">
                {new Date(event.begin).toLocaleDateString("de-DE", { day: "numeric", month: "long", year: "numeric" })}
              </div>
            {/if}
          </div>

          {#if !event.registration_active}
            <div class="shrink-0 rounded-lg border border-gray-200 px-3 py-1.5 text-sm text-gray-400">
              {#if event.registration_start_date}
                Anmeldung ab {new Date(event.registration_start_date).toLocaleDateString("de-DE", { day: "numeric", month: "long" })}
              {:else}
                Anmeldung noch nicht aktiv
              {/if}
            </div>
          {:else if event.allow_set_present}
            <a
              href="/event/{event.id}/timetable"
              class="shrink-0 rounded-lg bg-blue-600 px-4 py-1.5 text-sm font-medium text-white hover:bg-blue-700 transition-colors"
            >
              Zeitplan →
            </a>
          {:else}
            <a
              href="/event/{event.id}"
              class="shrink-0 rounded-lg bg-blue-600 px-4 py-1.5 text-sm font-medium text-white hover:bg-blue-700 transition-colors"
            >
              Zur Anmeldung →
            </a>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>
