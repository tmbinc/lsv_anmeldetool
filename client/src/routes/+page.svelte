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

<main>
  <FetchErrors admin={false} bind:this={fetch_errors} />
  {#if loading}
    <Loading />
  {:else}
    <h1>Turnierkalender</h1>
    <Table>
      <TableBody>
        {#each events as event}
          <TableBodyRow>
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
  {/if}
</main>
