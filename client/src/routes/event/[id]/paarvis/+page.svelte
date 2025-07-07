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
  const event_orgs: EventOrg[] = $state([]);
  let groups: Group[] = $state([]);
  let event_name: String = $state("");
  let pairings: PairingEntry[] = $state([]);
  let reveal;
  let started = $state(false);
  let orgs: Set<string> = new Set();

  onMount(async () => {
    getPairings({ event: event_id }).resp.subscribe((resp) => {
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

        goSlide();
        loading = false;
      }
    });
  });

  function checkTime(i: number): string {
    if (i < 10) {
      return "0" + i;
    } else {
      return "" + i;
    }
  }

  function startTime() {
    var today = new Date();
    var h = today.getHours();
    var m = today.getMinutes();
    var s = today.getSeconds();
    // add a zero in front of numbers<10
    let m_s = checkTime(m);
    let s_s = checkTime(s);
    let n = document.getElementById("time");
    if (n) {
      n.innerHTML = h + ":" + m_s + ":" + s_s;
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
</script>

<FetchErrors bind:this={fetch_errors} />
<div class="reveal">
  <div class="header">
    <div id="time"></div>
  </div>

  <div class="header">
    <div id="time"></div>
  </div>

  <div class="footer">(Stand: xx:xx:xx)</div>
  <div class="footer_right">
    <img src={img} alt="Michael Haukohl Stiftung" />
  </div>
  <div class="slides">
    <section>
      <div>Willkommen beim</div>
      <h2>{event_name}</h2>
      <h3>Zeitplan:</h3>
      <ul>
        <li>8:00 Uhr bis 8:30 Uhr: Anwesenheitsmeldung</li>
        <li>9:00 Uhr: Begrüßung</li>
        <li>9:15 Uhr: Beginn der 1. Runde</li>
        <li>ca. 15 Uhr: Siegerehrung</li>
        <li>15:30 Uhr: Ende</li>
      </ul>
    </section>
    {#each groups as group}
      <section>
        <section>
          <p>{group.name}</p>
          <p>
            Paarungsliste der {roundForGroup(group)}. Runde - 13:20 Uhr (Seite 1
            / 1)
          </p>
          <table width="100%">
            <tbody> </tbody><tbody>
              <tr
                ><td>Tisch</td><td>Mannschaft 1 (Brett 1 schwarz)</td><td
                  >Mannschaft 2 (Brett 1 weiß)</td
                ></tr
              >

              {#each pairings as pairing}
                {#if pairing.group == group.id}
                  <tr
                    ><td>{pairing.table}</td><td
                      >{pairing.team_home}
                      <sub>{pairing.team_home_org}</sub></td
                    ><td
                      >{pairing.team_guest}
                      <sub>{pairing.team_guest_org}</sub></td
                    ></tr
                  >
                {/if}
              {/each}
            </tbody>
          </table>
        </section>
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
