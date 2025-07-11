<script lang="ts">
  import { onMount, tick } from "svelte";
  import {
    getEvent,
    getGroupsForEvent,
    type EventOrg,
    type Event,
    type Group,
    getResults,
    type ResultEntry,
    getPairings,
    type PairingEntry,
    getTimetable,
    type TimetableRow,
  } from "../../../../api/api";
  import FetchErrors from "../../../FetchErrors.svelte";
  import { page } from "$app/state";
  import Reveal from "reveal.js";
  import { Button } from "flowbite-svelte";
  import "reveal.js/dist/reset.css";
  import "reveal.js/dist/reveal.css";
  import "reveal.js/dist/theme/night.css";
  import img from "$lib/images/logo_mhs.svg";

  let event_id = page.params.id;
  let fetch_errors: FetchErrors;
  let loading = $state(true);
  let failed_load = $state(false);
  const event_orgs: EventOrg[] = $state([]);
  let groups: Group[] = $state([]);
  let event_name: String = $state("");
  let pairings: PairingEntry[] = $state([]);
  let reveal;
  let started = $state(false);
  let orgs: Set<string> = new Set();
  let start_times: string[] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());
  let last_update = $state("");

  let pairings_per_group: [Group, PairingEntry[][]][] = $state([]);

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

          last_update = currentTime();

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
        groups = resp.data.groups;
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

        // Split pairings into a.) groups and b.) chunks of PAGE_LEN;
        pairings.forEach((pairing) => {
          let group = groups.find((group) => group.id == pairing.group);
          if (group) {
            let entry = pairings_per_group.find((grp) => group == grp[0]);
            if (!entry) {
              let n = pairings_per_group.push([group, []]);
              entry = pairings_per_group[n - 1];
            }

            const PAGE_LEN = 2;
            let pages = entry[1];
            let page = pages.find((f) => f.length < PAGE_LEN);
            if (page == undefined) {
              let n = pages.push([]);
              page = pages[n - 1];
            }
            page.push(pairing);
          }
        });

        last_update = currentTime();

        goSlide();
        loading = false;
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

  function checkTime(i: number): string {
    if (i < 10) {
      return "0" + i;
    } else {
      return "" + i;
    }
  }

  function currentTime() {
    var today = new Date();
    var h = today.getHours();
    var m = today.getMinutes();
    var s = today.getSeconds();
    // add a zero in front of numbers<10
    let m_s = checkTime(m);
    let s_s = checkTime(s);
    return h + ":" + m_s + ":" + s_s;
  }

  function startTime() {
    let n = document.getElementById("time");
    if (n) {
      n.innerHTML = currentTime();
    }
    let t = setTimeout(function () {
      startTime();
    }, 500);
  }

  function goSlide() {
    const deck = new Reveal(reveal);
    deck.initialize({
      autoSlide: 20000,
      loop: true,
      help: false,
      controls: false,
      transition: "fade",
    });
    deck.configure({
      autoSlideMethod: () => deck.down(),
    });
    started = true;
    startTime();
  }

  function roundForGroup(group: Group) {
    return pairings.find((p) => p.group == group.id)?.round || "?";
  }

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

<FetchErrors bind:this={fetch_errors} />
<div class="reveal">
  <div class="header">
    <div id="time"></div>
  </div>

  <div class="header">
    <div id="time"></div>
  </div>

  <div class="footer">(Stand: {last_update})</div>
  <div class="footer_right">
    <img src={img} alt="Michael Haukohl Stiftung" />
  </div>
  <div class="slides">
    <section>
      <div>Willkommen!</div>
      <h2>{event_name}</h2>
      <h3>Zeitplan:</h3>
      <ul>
        {#each start_times
          .filter((key) => key.includes("_flag:!_"))
          .toSorted((key) => parseInt(key)) as key}
          {#each timetableFor(key) as row}
            <li>
              {new Date(parseInt(key)).toTimeString().split(" ")[0].slice(0, 5)}
              {row.name}
              {#if groupnames.size != row.groups.length}
                ({row.groups.sort().join(", ")})
              {/if}
            </li>
          {/each}
        {/each}
      </ul>
    </section>
    {#each pairings_per_group as [group, pages]}
      <section>
        {#each pages as page, index}
          <section>
            <p>{group.name}</p>
            <p>
              Paarungsliste der {roundForGroup(group)}. Runde (Seite {index + 1}
              / {pages.length})
            </p>

            <table width="100%">
              <tbody> </tbody><tbody>
                <tr
                  ><td>Tisch</td><td>Mannschaft 1 (Brett 1 schwarz)</td><td
                    >Mannschaft 2 (Brett 1 weiß)</td
                  ></tr
                >

                {#each page as pairing}
                  <tr
                    ><td>{pairing.table}</td><td
                      >{pairing.team_home}
                      <sub>{pairing.team_home_org}</sub></td
                    ><td
                      >{pairing.team_guest}
                      <sub>{pairing.team_guest_org}</sub></td
                    ></tr
                  >
                {/each}
              </tbody>
            </table>
          </section>
        {/each}
      </section>
    {/each}
  </div>
</div>

<style>
  .reveal section h1 {
    font-size: 300%;
  }

  .boxes {
    width: 100%;
  }

  .boxestd {
    border: 5px solid white;
    text-align: center;
  }

  .noboxestd {
    text-align: center;
  }

  .tournament_name {
    font-size: 80%;
  }

  .tournament_group {
    font-size: 100%;
  }

  .mannschaft {
    font-size: 200%;
  }

  .schule {
    font-size: 80%;
  }

  .rank {
    font-size: 150%;
  }
  .reveal .header {
    position: absolute;
    top: 1em;
    right: 1em;
    font-size: 1em;
  }
  .reveal .footer {
    position: absolute;
    bottom: 1em;
    left: 4em;
    font-size: 0.5em;
  }
  .reveal .footer_right {
    position: absolute;
    bottom: 1em;
    right: 7em;
    font-size: 0.5em;
  }
  .reveal section table {
    display: inline-block;
    font-size: 0.8em;
    line-height: 1.2em;
    vertical-align: top;
  }
  .reveal section h2 {
    display: inline-block;
    font-size: 1.5em;
    line-height: 1.2em;
    vertical-align: top;
  }
</style>
