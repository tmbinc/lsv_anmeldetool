<script lang="ts">
  import { onMount } from "svelte";
  import {
    getPairings,
    getTimetable,
    readSse,
    type EventOrg,
    type Group,
    type PairingEntry,
    type TimetableRow,
  } from "../../../../api/api";
  import { page } from "$app/state";
  import Loading from "../../../Loading.svelte";
  import LoadError from "../../../LoadError.svelte";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  const event_id = page.params.id;
  let start_times: string[] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());

  const event_orgs: EventOrg[] = $state([]);
  let groups_pairings: Group[] = $state([]);

  let pairings: PairingEntry[] = $state([]);
  let orgs: Set<string> = new Set();

  onMount(async () => {
    let timetable_request = getTimetable({ event: event_id });

    timetable_request.resp.subscribe((resp) => {
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
              let key =
                time.toString() +
                "_" +
                row.flags
                  .split("")
                  .map((c) => "flag:" + c)
                  .join("_") +
                "_" +
                row.state;

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
    let pairing_request = getPairings({ event: event_id });

    pairing_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        pairings = resp.data.pairings;
        groups_pairings = resp.data.groups;
        event_name = resp.data.event_name;
        pairings.sort((a, b) => a.table - b.table);
        let rounds = new Set<number>();

        for (let pairing of pairings) {
          if (pairing.team_home_org) {
            orgs.add(pairing.team_home_org);
          }
          if (pairing.team_guest_org) {
            orgs.add(pairing.team_guest_org);
          }
          rounds.add(pairing.round);
        }
      }
    });

    const evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = function (event) {
      console.log(event);
      var dataobj = JSON.parse(event.data);
      console.log(dataobj);
      if (dataobj.kind == "pairing") {
        pairing_request.reload();
      }
      if (dataobj.kind == "timetable") {
        timetable_request.reload();
      }
    };
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
  function roundForGroup(group: Group) {
    return pairings.find((p) => p.group == group.id)?.round || "?";
  }
</script>

<!-- {#if loading}
    <Loading text="Lade Zeitplan..." />
  {:else if failed_load}
    <LoadError text="Laden fehlgeschlagen!" />
  {:else} -->

<main class="h-screen w-screen overflow-hidden bg-black">
  <div class="float:top; height: 80% overflow-hidden text-white">
    <table class="w-screen">
      <tbody>
        {#each start_times as key}
          {#if key.includes("_flag:!_") || key.endsWith("_active") || key.endsWith("_next")}
            <tr
              class={"" +
                (key.endsWith("_active")
                  ? "text-red-600 text-2xl"
                  : key.endsWith("_next")
                    ? "text-yellow-300 text-l"
                    : "text-slate-300")}
            >
              <td class="py-px">
                {#if key.endsWith("_next")}
                  Danach: {new Date(parseInt(key))
                    .toTimeString()
                    .split(" ")[0]
                    .slice(0, 5)}
                {:else if key.endsWith("_active")}
                  Jetzt
                {:else}
                  ca. {new Date(parseInt(key))
                    .toTimeString()
                    .split(" ")[0]
                    .slice(0, 5)}
                {/if}
              </td>
              <td class="py-px">
                {#each timetableFor(key) as row}
                  <div>
                    {row.name}
                    {#if groupnames.size != row.groups.length}
                      ({row.groups.sort().join(", ")})
                    {/if}
                  </div>
                {/each}
              </td>
            </tr>
          {/if}
        {/each}
      </tbody>
    </table>
  </div>

  <div class="float:bottom columns-4 text-white">
    {#each groups_pairings.filter((f) => !f.replacement) as group, group_index (group.id)}
      <div
        class={"mb-10 overflow-clip " +
          ["bg-blue-700", "bg-green-700", "bg-orange-700"][group_index % 3]}
      >
        <p>{group.name}</p>
        <p>
          Paarungsliste der {roundForGroup(group)}. Runde
        </p>
        <table
          class="table-fixed border-separate border-spacing-2 border border-gray-400 dark:border-gray-500"
        >
          <thead>
            <tr>
              <th class="w-1">Tisch</th>
              <th class="w-40">Mannschaft 1 (Brett 1 schwarz)</th><th
                >Mannschaft 2 (Brett 1 weiß)</th
              >
            </tr>
          </thead>
          <tbody>
            {#each { length: 10 }}
              {#each pairings as pairing}
                {#if pairing.group == group.id}
                  <tr class="">
                    <td class="py-0 px-1">{pairing.table}</td><td
                      class="py-0 px-1 overflow-clip"
                    >
                      <div>{pairing.team_home}</div>
                      <div>
                        <sub>{pairing.team_home_org}</sub>
                      </div></td
                    >
                    <td class="py-0 px-1 overflow-clip">
                      <div>{pairing.team_guest}</div>
                      <div><sub>{pairing.team_guest_org}</sub></div>
                    </td></tr
                  >
                {/if}
              {/each}
            {/each}
          </tbody>
        </table>
      </div>
    {/each}
  </div>
</main>
