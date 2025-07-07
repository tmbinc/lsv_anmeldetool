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

          // Ignore anything before the first "active" for each group.
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
    flags: string;
    groups: string[];
  };

  function timetableFor(key: string): EntryWithName[] {
    const items = timetable.get(key) || [];

    let unique_names = [...new Set(items.map((item) => item.name))].sort();

    return unique_names.map((name) => ({
      name: name,
      flags: items.find((f) => f.name == name)?.flags || "",
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
        {#each start_times as key}
          <TableBodyRow
            class={key.endsWith("_active")
              ? "bg-green-400 text-4xl"
              : key.endsWith("_next")
                ? "bg-orange-400 text-2xl"
                : ""}
          >
            <TableBodyCell>
              {#if !key.endsWith("_active")}
                {new Date(parseInt(key))
                  .toTimeString()
                  .split(" ")[0]
                  .slice(0, 5)}
              {/if}
            </TableBodyCell>
            <TableBodyCell>
              {#each timetableFor(key) as row}
                {#if row.flags.includes("p") && key.endsWith("_active")}
                  <div>
                    <a href="/event/{event_id}/pairings">
                      {row.name}
                      {#if groupnames.size != row.groups.length}
                        ({row.groups.sort().join(", ")})
                      {/if}
                      <sub class="underline">(Paarungen)</sub>
                    </a>
                  </div>
                {:else}
                  <div>
                    {row.name}
                    {#if groupnames.size != row.groups.length}
                      ({row.groups.sort().join(", ")})
                    {/if}
                  </div>
                {/if}
              {/each}
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if}
</main>
