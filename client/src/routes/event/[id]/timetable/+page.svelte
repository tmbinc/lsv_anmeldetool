<script lang="ts">
  import { onMount } from "svelte";
  import { getTimetable, type TimetableRow } from "../../../../api/api";
  import { page } from "$app/state";
  import Loading from "../../../Loading.svelte";
  import LoadError from "../../../LoadError.svelte";
  import {
    Label,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  const event_id = page.params.id;
  let start_times: string[] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());

  onMount(async () => {
    getTimetable({ event: event_id }).resp.subscribe((resp) => {
      if (resp) {
        if (resp.ok) {
          event_name = resp.data.event_name;
          let new_timetable = new Map<string, TimetableRow[]>();

          resp.data.rows.sort((a, b) =>
            a.expected_time.localeCompare(b.expected_time)
          );

          let seen_active = new Set<string>();
          let new_start_times = new Set<string>();

          for (const row of resp.data.rows) {
            if (seen_active.has(row.group) || row.state == "active") {
              seen_active.add(row.group);
              const time = Date.parse(row.expected_time + "Z");
              let key = time.toString() + row.state;

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

          let groups = resp.data.groups;

          groupnames = new Map(groups.map((group) => [group.id, group.name]));

          start_times = [...new_start_times].sort((a, b) => a.localeCompare(b));

          loading = false;
        } else {
          failed_load = true;
        }
      }
    });
  });

  type EntryWithName = {
    name: string;
    groups: string[];
  };

  function timetableFor(time: string): EntryWithName[] {
    const items = timetable.get(time) || [];

    let unique_names = [...new Set(items.map((item) => item.name))].sort();

    return unique_names.map((name) => ({
      name: name,
      groups: items
        .filter((f) => f.name == name)
        .map((r) => groupnames.get(r.group) || ""),
    }));
  }
</script>

<main class="md:mx-10">
  {#if loading}
    <Loading text="Lade Zeitplan..." />
  {:else if failed_load}
    <LoadError text="Laden fehlgeschlagen!" />
  {:else}
    <Label class="text-5xl m-10">Zeitplan</Label>
    <Label class="text-3xl m-10">{event_name}</Label>

    <Table>
      <TableBody>
        {#each start_times as start_time}
          <TableBodyRow
            class={start_time.endsWith("active")
              ? "bg-green-400 text-4xl"
              : start_time.endsWith("next")
                ? "bg-orange-400 text-4xl"
                : ""}
          >
            <TableBodyCell
              >{new Date(parseInt(start_time))
                .toTimeString()
                .split(" ")[0]
                .slice(0, 5)}</TableBodyCell
            >
            <TableBodyCell>
              {#each timetableFor(start_time) as row}
                <div>
                  {row.name} ({row.groups.sort().join(", ")})
                </div>
              {/each}
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if}
</main>
