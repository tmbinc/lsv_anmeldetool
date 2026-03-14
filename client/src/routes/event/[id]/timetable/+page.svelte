<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { getTimetable, type TimetableRow } from "../../../../api/api";
  import { page } from "$app/state";
  import Loading from "../../../Loading.svelte";
  import LoadError from "../../../LoadError.svelte";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  const event_id = page.params.id || "";
  let start_times: string[] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());
  let evtSource: EventSource | null = null;

  onMount(async () => {
    let timetable_request = getTimetable({ event: event_id });
    timetable_request.resp.subscribe((resp) => {
      if (resp) {
        if (resp.ok) {
          event_name = resp.data.event_name;
          let new_timetable = new Map<string, TimetableRow[]>();

          resp.data.rows.sort((a, b) =>
            a.expected_time.localeCompare(b.expected_time),
          );

          let seen_active = new Set<string>();
          let new_start_times = new Set<string>();

          for (const row of resp.data.rows) {
            if (seen_active.has(row.group) || row.state == "active") {
              seen_active.add(row.group);
              const time = Date.parse(row.expected_time + "Z");
              let key = time.toString() + row.flags + "_" + row.state;

              let r = new_timetable.get(key);
              if (!r) {
                new_timetable.set(key, [row]);
              } else {
                r.push(row);
              }
              new_start_times.add(key);
            }
          }

          timetable = new_timetable;
          groupnames = new Map(resp.data.groups.map((g) => [g.id, g.name]));
          start_times = [...new_start_times].sort((a, b) => a.localeCompare(b));
          loading = false;
        } else {
          failed_load = true;
        }
      }
    });

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.kind == "timetable") timetable_request.reload();
    };
  });

  onDestroy(() => evtSource?.close());

  type EntryWithName = { name: string; flags: string; groups: string[] };

  function timetableFor(key: string): EntryWithName[] {
    const items = timetable.get(key) || [];
    const unique_names = [...new Set(items.map((item) => item.name))].sort();
    return unique_names.map((name) => ({
      name,
      flags: items.find((f) => f.name == name)?.flags || "",
      groups: items.filter((f) => f.name == name).map((r) => groupnames.get(r.group) || ""),
    }));
  }

  function timeStr(key: string): string {
    return new Date(parseInt(key)).toTimeString().split(" ")[0].slice(0, 5);
  }
</script>

{#if loading}
  <Loading text="Lade Zeitplan..." />
{:else if failed_load}
  <LoadError text="Laden fehlgeschlagen!" />
{:else}
  <div class="min-h-screen bg-gray-50 px-4 py-8">
    <div class="mx-auto max-w-md">

      <!-- Header -->
      <div class="mb-8 text-center">
        <p class="text-sm font-medium uppercase tracking-widest text-gray-400">{event_name}</p>
        <h1 class="mt-1 text-3xl font-bold text-gray-900">Zeitplan</h1>
      </div>

      <!-- Timeline -->
      <div class="relative flex flex-col gap-0">
        <!-- Vertical line -->
        <div class="absolute left-[3.25rem] top-0 h-full w-px bg-gray-200"></div>

        {#each start_times as key, i}
          {@const entries = timetableFor(key)}
          {@const isActive = key.endsWith("_active")}
          {@const isNext = key.endsWith("_next")}

          <div class="relative flex items-start gap-4 pb-4">

            <!-- Time + dot column -->
            <div class="flex w-[3.25rem] shrink-0 flex-col items-center pt-0.5">
              {#if isActive}
                <span class="text-sm font-bold text-green-600">Jetzt</span>
              {:else}
                <span class="text-sm font-semibold tabular-nums text-gray-500">{timeStr(key)}</span>
              {/if}
            </div>

            <!-- Dot on the line -->
            <div class="absolute left-[3.25rem] top-[0.45rem] -translate-x-1/2">
              {#if isActive}
                <div class="h-3 w-3 rounded-full bg-green-500 ring-2 ring-green-200 ring-offset-1"></div>
              {:else if isNext}
                <div class="h-2.5 w-2.5 rounded-full bg-amber-400 ring-2 ring-amber-100 ring-offset-1"></div>
              {:else}
                <div class="h-2 w-2 rounded-full bg-gray-300"></div>
              {/if}
            </div>

            <!-- Card -->
            <div class="flex-1 rounded-xl px-4 py-3 {isActive
                ? 'bg-green-500 shadow-md shadow-green-200'
                : isNext
                  ? 'bg-amber-50 border border-amber-200'
                  : 'bg-white border border-gray-200'}">
              {#each entries as row}
                <div class="flex flex-wrap items-baseline gap-x-2">
                  {#if row.flags.includes("p") && isActive}
                    <a
                      href="/event/{event_id}/pairings"
                      class="text-base font-semibold text-white underline decoration-green-300 underline-offset-2"
                    >
                      {row.name}
                    </a>
                  {:else}
                    <span class="text-base font-semibold {isActive ? 'text-white' : isNext ? 'text-amber-900' : 'text-gray-800'}">
                      {row.name}
                    </span>
                  {/if}

                  {#if groupnames.size !== row.groups.length}
                    <span class="text-xs {isActive ? 'text-green-100' : 'text-gray-400'}">
                      {row.groups.sort().join(", ")}
                    </span>
                  {/if}
                </div>
              {/each}

              {#if isActive}
                <p class="mt-1 text-xs text-green-100">Läuft gerade</p>
              {:else if isNext}
                <p class="mt-1 text-xs text-amber-500">Als nächstes</p>
              {/if}
            </div>

          </div>
        {/each}
      </div>

    </div>
  </div>
{/if}
