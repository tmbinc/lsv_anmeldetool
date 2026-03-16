<script lang="ts">
  import { onMount } from "svelte";
  import { getRooms, setRooms, type Room } from "../../../../../../api/api";
  import { page } from "$app/state";
  import FetchErrors from "../../../../../FetchErrors.svelte";

  const event_id = page.params.event_id || "";
  const group_id = page.params.group || "";

  let fetch_errors: FetchErrors;
  let rooms: Room[] = $state([]);
  let saved = $state(false);
  let saving = $state(false);

  onMount(async () => {
    const resp = await getRooms({ event: event_id }).result;
    if (resp.ok) {
      rooms = resp.data.filter((r) => r.group_id == group_id);
    } else {
      fetch_errors.check(resp);
    }
  });

  function new_room() {
    rooms.push({
      event: event_id,
      group_id: group_id,
      room: "",
      table_num_low: 0,
      table_num_high: 0,
    });
    saved = false;
  }

  function remove_room(i: number) {
    rooms.splice(i, 1);
    saved = false;
  }

  async function save_rooms() {
    saving = true;
    saved = false;
    const result = await setRooms({ event: event_id, group: group_id, rooms }).result;
    saving = false;
    if (result.ok) {
      saved = true;
    } else {
      alert("Speichern fehlgeschlagen: " + result.data);
    }
  }
</script>

<FetchErrors bind:this={fetch_errors} />

<main class="px-4 py-8 sm:px-8">

  <div class="mb-6 flex items-center justify-between gap-4">
    <h1 class="text-xl font-bold text-gray-900">Raumzuordnung</h1>
    <div class="flex items-center gap-3">
      {#if saved}
        <span class="text-sm font-medium text-green-600">Gespeichert ✓</span>
      {/if}
      <button
        onclick={save_rooms}
        disabled={saving}
        class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm transition-colors hover:bg-blue-700 disabled:opacity-50"
      >{saving ? "Speichern…" : "Speichern"}</button>
    </div>
  </div>

  <!-- Room list -->
  {#if rooms.length === 0}
    <p class="rounded-xl border border-dashed border-gray-300 py-10 text-center text-sm text-gray-400">
      Noch keine Raumzuordnungen. Klicke auf „Raum hinzufügen".
    </p>
  {:else}
    <div class="overflow-hidden rounded-xl border border-gray-200 bg-white shadow-sm">

      <!-- Header row -->
      <div class="grid items-center gap-4 border-b border-gray-100 bg-gray-50 px-4 py-2 text-xs font-semibold uppercase tracking-wide text-gray-400"
           style="grid-template-columns: 1fr 3fr auto;">
        <div>Tische (von – bis)</div>
        <div>Raumname</div>
        <div class="w-7"></div>
      </div>

      {#each rooms as room, i}
        <div
          class="grid items-center gap-4 border-b border-gray-100 px-4 py-2.5 last:border-0 hover:bg-gray-50/50"
          style="grid-template-columns: 1fr 3fr auto;"
        >
          <!-- Table range -->
          <div class="flex items-center gap-1.5">
            <input
              type="number"
              bind:value={room.table_num_low}
              oninput={() => (saved = false)}
              class="w-16 rounded-md border border-gray-200 px-2 py-1.5 text-center text-sm tabular-nums focus:outline-none focus:ring-2 focus:ring-blue-400"
            />
            <span class="text-gray-400">–</span>
            <input
              type="number"
              bind:value={room.table_num_high}
              oninput={() => (saved = false)}
              class="w-16 rounded-md border border-gray-200 px-2 py-1.5 text-center text-sm tabular-nums focus:outline-none focus:ring-2 focus:ring-blue-400"
            />
          </div>

          <!-- Room name -->
          <input
            type="text"
            bind:value={room.room}
            oninput={() => (saved = false)}
            placeholder="z.B. Aula"
            class="w-full rounded-md border border-gray-200 px-2.5 py-1.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
          />

          <!-- Delete -->
          <button
            onclick={() => remove_room(i)}
            class="flex h-7 w-7 items-center justify-center rounded-md text-gray-300 transition-colors hover:bg-red-50 hover:text-red-500"
            title="Entfernen"
          >
            <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Add button -->
  <button
    onclick={new_room}
    class="mt-3 flex w-full items-center justify-center gap-2 rounded-xl border border-dashed border-gray-300 py-2.5 text-sm font-medium text-gray-500 transition-colors hover:border-blue-400 hover:bg-blue-50 hover:text-blue-600"
  >
    <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
    </svg>
    Raum hinzufügen
  </button>

</main>
