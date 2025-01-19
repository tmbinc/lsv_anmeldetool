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
    Datepicker,
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
  let changed: string[] = $state([]);
  let edit_mode = $state(false);

  onMount(async () => {
    const request = listEvents({});
    const resp = await request.result;
    if (resp.ok) {
      events = resp.data;
    }
  });

  async function add() {
    const request = addEvent({ name: "new" });
    const resp = await request.result;
    if (resp.ok) {
      events.push(resp.data);
    }
  }

  function update(id: string) {
    let event_to_update = events.find((event) => event.id == id);
    if (event_to_update) {
      updateEvent({ ...event_to_update, event: event_to_update.id });
      changed = changed.filter((item) => item != id);
    }
  }

  async function revert(id: string) {
    let i = events.findIndex((event) => event.id == id);
    if (i != -1) {
      let resp = await getEvent({ event: id }).result;
      if (resp.ok) {
        events[i] = resp.data;
        changed = changed.filter((item) => item != id);
      }
    }
  }

  function change(id: string) {
    if (!changed.includes(id)) {
      changed.push(id);
    }
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
          <TableBodyRow>
            <TableBodyCell>
              <input
                disabled={!edit_mode}
                bind:value={event.name}
                oninput={() => change(event.id)}
              /></TableBodyCell
            >
            <TableBodyCell>
              <!-- <Datepicker bind:value={fakedate.get(event.id} /> -->
              <input
                disabled={!edit_mode}
                bind:value={event.begin}
                oninput={() => change(event.id)}
              />
            </TableBodyCell>
            <TableBodyCell>
              <Checkbox
                disabled={!edit_mode}
                bind:checked={event.public}
                oninput={() => change(event.id)}
              ></Checkbox>
            </TableBodyCell>
            <TableBodyCell>
              {#if edit_mode}
                <Button
                  on:click={() => update(event.id)}
                  class="px-3 py-2 text-xs font-medium text-center text-white bg-green-700 rounded-lg hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
                  disabled={!changed.includes(event.id)}>Save</Button
                >
                <Button
                  class="px-3 py-2 text-xs font-medium text-center text-white bg-red-700 rounded-lg hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800"
                  disabled={!changed.includes(event.id)}
                  on:click={() => revert(event.id)}>Undo</Button
                >
              {/if}
              <Button
                class="px-3 py-2 text-xs font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                href="/admin/event/{event.id}">Edit Details</Button
              ></TableBodyCell
            >
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
    <Button onclick={add}>Add new...</Button>
    <Button
      disabled={edit_mode}
      onclick={() => {
        edit_mode = true;
      }}>Edit</Button
    >
  {/if}

  <!-- <Button
      on:click={() => {
        request.reload();
      }}
    >
      Reload orgs
    </Button> -->
</main>
