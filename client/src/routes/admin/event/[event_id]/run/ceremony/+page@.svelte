<script lang="ts">
  import { onMount } from "svelte";
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
  let event_id = page.params.event_id;
  let fetch_errors: FetchErrors;
  let loading = $state(true);
  const event_orgs: EventOrg[] = $state([]);
  let groups: Group[] = $state([]);
  let event_name: String = $state("");
  let results: ResultEntry[] = $state([]);

  onMount(async () => {
    const resp_results = await getResults({
      event: page.params.event_id,
    }).result;

    if (resp_results.ok) {
      event_name = resp_results.data.event_name;
      groups = resp_results.data.groups;
      results = resp_results.data.results;
    } else {
      fetch_errors.check(resp_results);
    }
    loading = false;
  });
</script>

<FetchErrors bind:this={fetch_errors} />

<head>
  <!-- <meta charset="utf-8">
		<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">

		<title>{{ event?.name }}</title>

		<link rel="stylesheet" href="m/dist/reset.css">
		<link rel="stylesheet" href="m/dist/reveal.css">
		<link rel="stylesheet" href="m/dist/theme/night.css">

  <link rel="stylesheet" href="m/plugin/highlight/monokai.css" />

  -->

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

    .tournament_subtitle {
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
</head>
<body>
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
                <p class="schule">der {result.team_org}</p>
                <small
                  >mit {result.points_team} Mannschaftspunkten, {result.points_team}
                  Brettpunkten und {result.tie} Buchholz-Punkten</small
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

  <!-- <script src="m/dist/reveal.js"></script>
  <script src="m/plugin/notes/notes.js"></script>
  <script src="m/plugin/markdown/markdown.js"></script>
  <script src="m/plugin/highlight/highlight.js"></script> -->
  <!-- <script>
    // More info about initialization & config:
    // - https://revealjs.com/initialization/
    // - https://revealjs.com/config/
    Reveal.initialize({
      hash: false, // always restart from scratch,
      overview: false,
      help: false,
      controls: false,
      transition: "fade",

      // Learn about plugins: https://revealjs.com/plugins/
      plugins: [RevealMarkdown, RevealHighlight, RevealNotes],
    });
    Reveal.configure({
      autoSlideMethod: () => Reveal.down(),
    });
  </script> -->
</body>
