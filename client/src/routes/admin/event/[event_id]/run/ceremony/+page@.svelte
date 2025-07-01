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

  let event_id = page.params.event_id;
  let fetch_errors: FetchErrors;
  let loading = $state(true);
  const event_orgs: EventOrg[] = $state([]);
  let groups: Group[] = $state([]);
  let event_name: String = $state("");
  let results: ResultEntry[] = $state([]);
  let reveal;
  let started = $state(false);

  onMount(async () => {
    const resp_results = await getResults({
      event: page.params.event_id,
    }).result;

    console.log(resp_results);

    if (resp_results.ok) {
      event_name = resp_results.data.event_name;
      groups = resp_results.data.groups;

      // Sort results by (inverted) rank
      results = resp_results.data.results.sort(
        (a, b) => (b.rank || 0) - (a.rank || 0)
      );

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

              <!-- {% if result.next_left is defined or result.next_right is defined %}
<hr />
<small>N&auml;chste Mannschaften:

<table class="reveal boxes">
<tr>
<td width="30%" class="boxestd">
{% if result.next_left is defined %}
{{ result.next_left.Mannschaft }}
{% endif %}
</td>
<td width="40%" class="noboxestd">
B&uuml;hne
</td>
<td width="30%" class="boxestd">
{% if result.next_right is defined %}
{{ result.next_right.Mannschaft }}
{% endif %}
</td>
</tr>
</table>

</small>
{% endif %} -->
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

  .rank {
    font-size: 150%;
  }
</style>
