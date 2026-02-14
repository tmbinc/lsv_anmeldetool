<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    getPairings,
    getResults,
    getTimetable,
    readSse,
    type EventOrg,
    type Group,
    type PairingEntry,
    type ResultEntry,
    type TimetableRow,
  } from "../../../../api/api";
  import { page } from "$app/state";
  import Loading from "../../../Loading.svelte";
  import LoadError from "../../../LoadError.svelte";
  import { Checkbox } from "flowbite-svelte";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  const event_id = page.params.id;
  let start_times: string[] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());

  const event_orgs: EventOrg[] = $state([]);
  let groups_results: Group[] = $state([]);

  let results: ResultEntry[] = $state([]);
  let orgs: Set<string> = new Set();
  let evtSource: EventSource | null = null;
  let enabled_groups: string[] = $state([]);

  onMount(async () => {
    let results_request = getResults({ event: event_id });

    results_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        results = resp.data.results;
        groups_results = resp.data.groups;
        event_name = resp.data.event_name;
        results.sort((a, b) => (a.rank || 0) - (b.rank || 0));

        for (let result of results) {
          if (result.team) {
            orgs.add(result.team);
          }
        }

        groupnames = new Map(
          groups_results.map((group) => [group.id, group.name]),
        );

        if (enabled_groups) {
          enabled_groups = groups_results.map((group) => group.id);
        }
      }
    });

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = function (event) {
      console.log(event);
      var dataobj = JSON.parse(event.data);
      console.log(dataobj);
      if (dataobj.kind == "result") {
        results_request.reload();
      }
    };
  });

  onDestroy(() => {
    evtSource?.close();
  });

  type EntryWithName = {
    name: string;
    flags: string;
    groups: string[];
  };

  function roundForGroup(group: Group) {
    return results.find((p) => p.group == group.id)?.round || "?";
  }
</script>

<main class="h-screen w-screen overflow-hidden bg-black text-white">
  <div class="float:bottom columns-1 text-white">
    {#each groups_results.filter((f) => !f.replacement) as group, group_index (group.id)}
      <div
        class={"mb-10 overflow-clip bg-" + group.color + "-500"}
        role="button"
        tabindex="0"
        ondblclick={() => {
          enabled_groups = enabled_groups?.filter((f) => f != group.id);
        }}
        hidden={!enabled_groups?.includes(group.id)}
      >
        <p class="text-3xl">
          {group.name} - Ergebnisse (Stand: Runde {roundForGroup(group)})
        </p>
        <table
          class="table-fixed border-separate border-spacing-0 border border-gray-400 dark:border-gray-500"
        >
          <thead>
            <tr>
              <th class="w-1">Platz</th>
              <th class="w-40">Name</th>
              <th class="w-40">MPkt.</th>
              <th class="w-40">Brettpunkte</th>
              <th class="w-40">Zweitwertung</th>
            </tr>
          </thead>
          <tbody>
            {#each results.filter((p) => p.group == group.id) as result, result_index}
              <tr
                class={"break-inside-avoid " +
                  (result_index % 2
                    ? "bg-" + group.color + "-800"
                    : "bg-" + group.color + "-400")}
              >
                <td class="py-0 px-1 border-b-2 border-b-black">
                  {result.rank}</td
                ><td class="py-0 px-1 overflow-clip border-b-2 border-b-black">
                  <div>{result.team}</div>
                  <div>
                    <sub>{result.team_org}</sub>
                  </div></td
                >
                <td>
                  {result.points_team}
                </td>
                <td>
                  {result.points_player}
                </td>
                <td>
                  {result.tie}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/each}
  </div>
</main>
