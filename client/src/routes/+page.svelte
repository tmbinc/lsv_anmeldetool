<script lang="ts">
  import { onMount } from "svelte";
  import { listEvents, type Event } from "../api/api";
  import {
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import Loading from "./Loading.svelte";
  import FetchErrors from "./FetchErrors.svelte";

  let loading = $state(false);
  let events: Event[] = $state([]);
  let fetch_errors: FetchErrors;

  onMount(async () => {
    const request = listEvents({});
    const resp = await request.result;
    if (resp.ok) {
      events = resp.data;
    } else {
      fetch_errors.check(resp);
    }
  });
</script>

<main
  class="mx-auto items-center gap-x-4 rounded-xl bg-white p-6
shadow-lg outline outline-black/5 dark:bg-slate-800 dark:shadow-none
dark:-outline-offset-1 dark:outline-white/10"
>
  <div>
    <FetchErrors admin={false} bind:this={fetch_errors} />
    <!-- {#if loading}
      <Loading />
    {:else}
      <div class="text-xl font-medium text-black dark:text-white">
        Turnierkalender
      </div>
      <div class="flex items-center sm:justify-center ml-4 sm:ml-0">
        <Table>
          <TableBody>
            {#each events as event}
              <TableBodyRow class="flex flex-col md:flex-row  mb-4">
                <TableBodyCell>
                  {event.name}
                </TableBodyCell>
                <TableBodyCell>
                  {#if event.begin}
                    Datum: {new Date(event.begin).toLocaleDateString()}
                  {/if}
                </TableBodyCell>
                <TableBodyCell
                  ><Button href="/event/{event.id}">Zur Anmeldung...</Button
                  ></TableBodyCell
                >
              </TableBodyRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    {/if} -->

    <div class="text-center">
      <p class="text-4xl">Willkommen auf der Anmeldeseite zum MUK-Turnier!</p>
      <p class="m-10">
        Vielen Dank für euer Interesse am diesjährigen MUK-Turnier! Die
        Anmeldung startet am 15. Juni 2025 – alle weiteren Informationen folgen
        in Kürze auf dieser Seite.
      </p>

      <img
        class="w-3xl mx-auto object-center"
        alt="Logo MuK-Turnier"
        src="https://lsv1873.de/images/MuK/2023/logo-nobackground-500.png"
      />
    </div>
  </div>
</main>
