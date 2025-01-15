<script lang="ts">
  import { onMount } from "svelte";
  import {
    listEvents,
    getEvent,
    type Event,
    updateEvent,
    addEvent,
  } from "../../../api/api";
  import {
    Button,
    Checkbox,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import type { ApiRequest } from "@cocreators-ee/apity";
  import type { components } from "../../../api/api.d";

  let loading = false;
  let events: Event[] = $state([]);
  let event_changed: Set<string> = $state(new Set<string>());

  onMount(async () => {
    const request = listEvents(undefined);
    const resp = await request.result;
    if (resp.ok) {
      events = resp.data;
    }
  });

  const add = async () => {
    const request = addEvent({ name: "new" });
    const resp = await request.result;
    if (resp.ok) {
      events.push(resp.data);
    }
  };

  function update(id: string) {
    let event_to_update = events.find((event) => event.id == id);
    if (event_to_update) {
      updateEvent({ ...event_to_update, event: event_to_update.id });
      event_changed.delete(id);
    }
  }

  function changed(event: Event) {
    event_changed.add(event.id);
  }
</script>

<main>
  {#if loading}
    <div class="w-full mt-24 flex items-center justify-center gap-4">
      <div
        class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-blue-600"
      ></div>

      <h1 class="text-2xl font-semibold text-gray-900">Loading Details...</h1>
    </div>
  {:else}
    <Table>
      <TableHead>
        <TableHeadCell>Name</TableHeadCell>
        <TableHeadCell>Date</TableHeadCell>
        <TableHeadCell>Public</TableHeadCell>
        <TableHeadCell>Edit</TableHeadCell>
      </TableHead>

      <TableBody>
        {#each events as event}
          {event_changed.has(event.id)}
          <TableBodyRow>
            <TableBodyCell
              ><div
                contenteditable="true"
                onchange={() => changed(event)}
                bind:innerText={event.name}
              ></div></TableBodyCell
            >
            <TableBodyCell
              ><div
                contenteditable="true"
                onchange={() => changed(event)}
                bind:innerText={event.begin}
              ></div></TableBodyCell
            >
            <TableBodyCell>
              <Checkbox
                bind:checked={event.public}
                onchange={() => changed(event)}
              ></Checkbox>
            </TableBodyCell>
            <TableBodyCell
              ><Button on:click={() => update(event.id)}>Update</Button>
              <Button href="/admin/event/{event.id}">Edit Details</Button
              ></TableBodyCell
            >
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
    <Button onclick={add}>Add new...</Button>
  {/if}

  <!-- <Button
      on:click={() => {
        request.reload();
      }}
    >
      Reload orgs
    </Button> -->
</main>
