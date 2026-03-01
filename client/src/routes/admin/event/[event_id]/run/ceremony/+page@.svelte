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
  } from "../../../../../../api/api";
  import FetchErrors from "../../../../../FetchErrors.svelte";
  import { page } from "$app/state";
  import Reveal from "reveal.js";
  import { Button } from "flowbite-svelte";
  import "reveal.js/dist/reset.css";
  import "reveal.js/dist/reveal.css";
  import "reveal.js/dist/theme/night.css";

  let event_id = page.params.event_id || "";
  let fetch_errors: FetchErrors;
  let loading = $state(true);
  const event_orgs: EventOrg[] = $state([]);
  let groups: Group[] = $state([]);
  let event_name: String = $state("");
  let results: ResultEntryCeremony[] = $state([]);
  let reveal;
  let started = $state(false);

  interface ResultEntryCeremony extends ResultEntry {
    next_left: ResultEntryCeremony | null;
    next_right: ResultEntryCeremony | null;
  }

  onMount(async () => {
    const resp_results = await getResults({
      event: page.params.event_id || "",
    }).result;

    console.log(resp_results);

    if (resp_results.ok) {
      event_name = resp_results.data.event_name;
      groups = resp_results.data.groups;

      // Sort results by (inverted) rank
      results = resp_results.data.results
        .sort(
          (a, b) =>
            a.group.localeCompare(b.group) || (b.rank || 0) - (a.rank || 0),
        )
        .map((entry) => ({
          ...entry,
          next_left: null,
          next_right: null,
        }));

      function findRank(
        results: ResultEntryCeremony[],
        result: ResultEntryCeremony,
        delta: number,
      ): ResultEntryCeremony | undefined {
        let index = results.findIndex((r) => r == result);
        return results[index + delta];
      }

      results.forEach((result, index) => {
        let delta_left = 0,
          delta_right = 0;

        if (index % 2 == 1) {
          delta_left = 1;
          delta_right = 2;
        } else {
          delta_right = 1;
          delta_left = 2;
        }

        let team_left = results[index + delta_left];
        let team_right = results[index + delta_right];

        result.next_left =
          team_left?.group == result.group && (team_left?.rank ?? 0) > 3
            ? team_left
            : null;
        result.next_right =
          team_right?.group == result.group && (team_right?.rank ?? 0) > 3
            ? team_right
            : null;
      });

      goSlide();
    } else {
      fetch_errors.check(resp_results);
    }
    loading = false;
  });

  function goSlide() {
    const deck = new Reveal(reveal);
    deck.initialize({
      hash: false, // always restart from scratch,
      overview: false,
      help: false,
      controls: false,
      transition: "fade",
    });
    started = true;
  }
</script>

<FetchErrors bind:this={fetch_errors} />
<!-- 
{#if !started}
  <Button on:click={goSlide}>Go Slideshow!</Button>
{/if} -->
<div class="reveal">
  <div class="slides">
    {#each groups as group}
      <section>
        <div class="tournament_name">{event_name}</div>
        <div class="tournament_group">{group.name}</div>

        <section>
          <h1>Preisverleihung</h1>
        </section>

        {#each results as result}
          {#if result.group == group.id}
            <section>
              <p class="rank">{result.rank} Platz:</p>
              <p class="mannschaft">{result.team}</p>
              <p class="schule">
                {result.team_genus == "f"
                  ? "der"
                  : result.team_genus == "m"
                    ? "des"
                    : result.team_genus == "n"
                      ? "des"
                      : ""}
                {result.team_org}
              </p>
              <small
                >mit {result.points_team}
                {result.points_team == 1
                  ? "Mannschaftspunkt"
                  : "Mannschaftspunkten"}, {result.points_team}
                {result.points_team == 1 ? "Brettpunkt" : "Brettpunkten"} und {result.tie}
                {result.tie == 1 ? "Buchholz-Punkt" : "Buchholz-Punkten"}</small
              >

              {#if result.next_left || result.next_right}
                <hr />

                <p>
                  <small
                    >Es machen sich bitte fertig, und kommen auf der linken
                    Seite zur Bühne:</small
                  >
                </p>

                <ul>
                  {#if result.next_left}
                    <li>
                      <span class="team_small">
                        {result.next_left.team}
                      </span>
                      <span class="schule_small">
                        ({result.next_left.team_org})
                      </span>
                    </li>
                  {/if}
                  {#if result.next_right}
                    <li>
                      <span class="team_small">
                        {result.next_right.team}
                      </span>
                      <span class="schule_small">
                        ({result.next_right.team_org})
                      </span>
                    </li>
                  {/if}
                </ul>
              {/if}
            </section>
          {/if}
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

  .schule_small {
    font-size: 80%;
  }

  .team_small {
    font-size: 100%;
  }

  .team_badge {
    width: 50px;
    height: 50px;
    line-height: 50px;
    border-radius: 50%;
    font-size: 50px;
    color: #000;
    text-align: center;
    background: #fff;
  }

  .rank {
    font-size: 150%;
  }
</style>
