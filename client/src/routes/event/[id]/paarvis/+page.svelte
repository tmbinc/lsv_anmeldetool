<script lang="ts">
  import { onDestroy, onMount, tick } from "svelte";
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
  import img_sponsor from "$lib/images/logo_mhs.svg";
  import img_schachverein from "$lib/images/Logo_Final_Schachverein.png";

  let event_id = page.params.id || "";
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
  let row_items_per_group: [string, TimetableRow[]][] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());
  let last_update = $state("");
  let pairings_per_group: [Group, PairingEntry[][]][] = $state([]);
  let evtSource: EventSource | null = null;
  $inspect(start_times);
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

          row_items_per_group = [];

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

              let r_group = row_items_per_group.find((r) => r[0] == row.group);
              let r_group_rows: TimetableRow[];
              if (!r_group) {
                r_group_rows = [];
                row_items_per_group.push([row.group, r_group_rows]);
              } else {
                r_group_rows = r_group[1];
              }
              r_group_rows.push(row);
            }
          }

          timetable = new_timetable;

          let groups = resp.data.groups;

          groupnames = new Map(groups.map((group) => [group.id, group.name]));

          start_times = [...new_start_times].sort((a, b) => a.localeCompare(b));

          last_update = currentTime();

          goSlide();
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

        orgs.clear();

        for (let pairing of pairings) {
          if (pairing.team_home_org) {
            orgs.add(pairing.team_home_org);
          }
          if (pairing.team_guest_org) {
            orgs.add(pairing.team_guest_org);
          }
          rounds.add(pairing.round);
        }

        pairings_per_group = [];

        // Split pairings into a.) groups and b.) chunks of PAGE_LEN;
        pairings.forEach((pairing) => {
          let group = groups.find((group) => group.id == pairing.group);
          if (group) {
            let entry = pairings_per_group.find((grp) => group == grp[0]);
            if (!entry) {
              let n = pairings_per_group.push([group, []]);
              entry = pairings_per_group[n - 1];
            }

            const PAGE_LEN = 8;
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

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = function (event) {
      var dataobj = JSON.parse(event.data);
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

  onDestroy(() => {
    evtSource?.close();
  });

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
      hash: true,
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

  <div class="header_left">(Stand: {last_update})</div>
  <div class="footer_right">
    <img src={img_sponsor} alt="Michael Haukohl Stiftung" />
  </div>
  <div class="slides">
    <section>
      <section>
        <div>Willkommen beim</div>
        <h2>{event_name}</h2>
      </section>
      <section>
        <div>Diese Veranstaltung wird ausgerichtet vom</div>
        <div>Lübecker Schachverein von 1873 e.V.</div>
        <div>
          <img src={img_schachverein} alt="Lübecker Schachverein" width="50%" />
        </div>
        <h2>www.lsv1873.de</h2>
      </section>
      <section>
        <h3>
          Diese Veranstaltung wird unterstützt von der Michael-Haukohl-Stiftung
        </h3>
        <h1>
          <div>
            <img src={img_sponsor} alt="Michael Haukohl Stiftung" width="50%" />
          </div>
        </h1>
      </section>
    </section>
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

    {#each groups as group}
      <section>
        <section>
          <h1>
            {#each row_items_per_group
              .filter((p) => p[0] == group.id)
              .map((p) => p[1]) as rows}
              {#each rows.filter((f) => f.state == "active") as row}
                {row.name}
              {/each}
            {/each}
          </h1>

          <p>{event_name}</p>
          <p><small>{group.name}</small></p>

          {#each row_items_per_group
            .filter((p) => p[0] == group.id)
            .map((p) => p[1]) as rows}
            {#each rows.filter((f) => f.state == "next") as row}
              {new Date(row.expected_time + "Z")
                .toTimeString()
                .split(" ")[0]
                .slice(0, 5)} Uhr:
              {row.name}
            {/each}
          {/each}
        </section>

        <section>
          <div>Diese Veranstaltung wird ausgerichtet vom</div>
          <div>Lübecker Schachverein von 1873 e.V.</div>
          <div>
            <img
              src={img_schachverein}
              alt="Lübecker Schachverein"
              width="50%"
            />
          </div>
          <h2>www.lsv1873.de</h2>
        </section>
        <section>
          <h3>
            Diese Veranstaltung wird unterstützt von der
            Michael-Haukohl-Stiftung
          </h3>
          <h1>
            <div>
              <img
                src={img_sponsor}
                alt="Michael Haukohl Stiftung"
                width="50%"
              />
            </div>
          </h1>
        </section>
      </section>
    {/each}

    {#each pairings_per_group as [group, pages]}
      <section>
        {#each pages as page, index}
          <section>
            <p>{group.name}</p>
            <p>
              Paarungsliste der {roundForGroup(group)}. Runde (Seite {index + 1}
              / {pages.length})
            </p>

            <table width="100%" class="table-fixed">
              <tbody>
                <tr
                  ><td width="5%" class="overflow-hidden">Tisch</td><td
                    width="40%"
                    class="overflow-hidden">Mannschaft 1 (Brett 1 schwarz)</td
                  ><td width="40%" class="overflow-hidden"
                    >Mannschaft 2 (Brett 1 weiß)</td
                  ></tr
                >

                {#each page as pairing}
                  <tr
                    ><td
                      width="5%"
                      class="overflow-hidden whitespace-nowrap p-0"
                      >{pairing.table}</td
                    ><td
                      width="40%"
                      class="overflow-hidden whitespace-nowrap p-0"
                    >
                      <div class="pairing_teamname">
                        {pairing.team_home}
                      </div>
                      <div class="pairing_orgname">
                        {pairing.team_home_org}
                      </div></td
                    ><td
                      width="40%"
                      class="overflow-hidden whitespace-nowrap p-0"
                      ><div class="pairing_teamname">
                        {pairing.team_guest || "spielfrei"}
                      </div>
                      <div class="pairing_orgname">
                        {pairing.team_guest_org}
                      </div></td
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

  .reveal .header_left {
    position: absolute;
    top: 1em;
    left: 1em;
    font-size: 0.5em;
  }

  .reveal .footer {
    position: absolute;
    bottom: 1em;
    left: 4em;
    font-size: 0.5em;
  }

  .reveal .footer_left {
    position: absolute;
    bottom: 1em;
    right: 7em;
    font-size: 0.5em;
  }

  .reveal .footer_right {
    position: absolute;
    bottom: 1em;
    right: 1em;
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

  .reveal table td {
    padding-top: 0px;
    padding-bottom: 0px;
  }

  .pairing_orgname {
    font-size: 0.4em;
    line-height: 1.4em;
  }

  .pairing_teamname {
    font-size: 0.8em;
  }
</style>
