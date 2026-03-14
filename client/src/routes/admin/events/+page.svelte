<script lang="ts">
  import { onMount } from "svelte";
  import {
    listEvents,
    getEvent,
    type Event,
    updateEvent,
    addEvent,
  } from "../../../api/api";
  import { goto } from "$app/navigation";

  let events: Event[] = $state([]);
  let changed: string[] = $state([]);
  let edit_mode = $state(false);

  onMount(async () => {
    const resp = await listEvents({}).result;
    if (resp.ok) {
      events = resp.data;
    } else {
      if (resp.status == 401) {
        goto("/admin/login");
      }
    }
  });

  async function add() {
    const resp = await addEvent({ name: "Neues Turnier" }).result;
    if (resp.ok) {
      events.push(resp.data);
      changed.push(resp.data.id);
      edit_mode = true;
    }
  }

  async function update(id: string) {
    const event_to_update = events.find((e) => e.id === id);
    if (event_to_update) {
      const resp = await updateEvent({
        ...event_to_update,
        event: event_to_update.id,
      }).result;
      if (resp.ok) {
        changed = changed.filter((item) => item !== id);
      }
    }
  }

  async function revert(id: string) {
    const i = events.findIndex((e) => e.id === id);
    if (i !== -1) {
      const resp = await getEvent({ event: id }).result;
      if (resp.ok) {
        events[i] = resp.data;
        changed = changed.filter((item) => item !== id);
      }
    }
  }

  function change(id: string) {
    if (!changed.includes(id)) changed.push(id);
  }
</script>

<div class="mx-auto max-w-4xl px-6 py-10">

  <div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-bold text-gray-900">Turniere</h1>
    <div class="flex gap-2">
      <button
        onclick={() => { edit_mode = !edit_mode; }}
        class="rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 transition-colors"
      >
        {edit_mode ? "Bearbeiten beenden" : "Bearbeiten"}
      </button>
      <button
        onclick={add}
        class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 transition-colors"
      >
        + Neues Turnier
      </button>
    </div>
  </div>

  <div class="flex flex-col gap-3">
    {#each events as event}
      <div class="rounded-xl border border-gray-200 bg-white p-5 shadow-sm {changed.includes(event.id) ? 'border-amber-300 bg-amber-50' : ''}">
        <div class="flex flex-col gap-4 sm:flex-row sm:items-center">

          <!-- Name + date -->
          <div class="flex-1 flex flex-col gap-2 sm:flex-row sm:items-center sm:gap-4">
            {#if edit_mode}
              <input
                type="text"
                bind:value={event.name}
                oninput={() => change(event.id)}
                class="w-full rounded-lg border border-gray-300 px-3 py-1.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400 sm:max-w-xs"
              />
              <input
                type="text"
                bind:value={event.begin}
                oninput={() => change(event.id)}
                placeholder="Datum (ISO)"
                class="w-full rounded-lg border border-gray-300 px-3 py-1.5 text-sm text-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-400 sm:max-w-48"
              />
              <label class="flex items-center gap-1.5 text-sm text-gray-600 cursor-pointer shrink-0">
                <input
                  type="checkbox"
                  bind:checked={event.public}
                  onchange={() => change(event.id)}
                  class="h-4 w-4 rounded border-gray-300 text-blue-600"
                />
                Öffentlich
              </label>
            {:else}
              <div>
                <div class="font-semibold text-gray-900">{event.name}</div>
                {#if event.begin}
                  <div class="mt-0.5 text-sm text-gray-400">
                    {new Date(event.begin).toLocaleDateString("de-DE", { day: "numeric", month: "long", year: "numeric" })}
                  </div>
                {/if}
              </div>
              {#if event.public}
                <span class="shrink-0 rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-700">Öffentlich</span>
              {:else}
                <span class="shrink-0 rounded-full bg-gray-100 px-2.5 py-0.5 text-xs font-medium text-gray-500">Nicht öffentlich</span>
              {/if}
            {/if}
          </div>

          <!-- Action buttons -->
          <div class="flex shrink-0 flex-wrap items-center gap-2">
            {#if edit_mode && changed.includes(event.id)}
              <button
                onclick={() => update(event.id)}
                class="rounded-lg bg-green-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-green-700 transition-colors"
              >
                Speichern
              </button>
              <button
                onclick={() => revert(event.id)}
                class="rounded-lg border border-gray-300 px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-50 transition-colors"
              >
                Zurücksetzen
              </button>
            {/if}
            <a
              href="/admin/event/{event.id}"
              class="rounded-lg border border-gray-300 px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-50 transition-colors"
            >
              Einstellungen
            </a>
            <a
              href="/admin/event/{event.id}/run/registration"
              class="rounded-lg border border-gray-300 px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-50 transition-colors"
            >
              Anmeldungen
            </a>
            <a
              href="/admin/event/{event.id}/run"
              class="rounded-lg bg-blue-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-blue-700 transition-colors"
            >
              Turnier leiten →
            </a>
          </div>

        </div>
      </div>
    {/each}

    {#if events.length === 0}
      <p class="py-8 text-center text-sm text-gray-400">Keine Turniere vorhanden.</p>
    {/if}
  </div>

</div>
