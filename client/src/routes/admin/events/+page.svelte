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
    ButtonGroup,
    Checkbox,
    Datepicker,
    Input,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import type { ApiRequest } from "@cocreators-ee/apity";
  import type { components } from "../../../api/api.d";
  import { goto } from "$app/navigation";

  let loading = false;
  let events: Event[] = $state([]);
  let changed: string[] = $state([]);
  let edit_mode = $state(false);

  onMount(async () => {
    const request = listEvents({});
    const resp = await request.result;
    if (resp.ok) {
      events = resp.data;
    } else {
      if (resp.status == 401) {
        goto("/admin/login");
      }
    }
  });

  async function add() {
    const request = addEvent({ name: "new" });
    const resp = await request.result;
    if (resp.ok) {
      events.push(resp.data);
    }
  }

  async function update(id: string) {
    let event_to_update = events.find((event) => event.id == id);
    if (event_to_update) {
      let resp = await updateEvent({
        ...event_to_update,
        event: event_to_update.id,
      }).result;
      if (resp.ok) {
        changed = changed.filter((item) => item != id);
      }
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
              <Input
                disabled={!edit_mode}
                bind:value={event.name}
                oninput={() => change(event.id)}
              /></TableBodyCell
            >
            <TableBodyCell>
              <Input
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
              <ButtonGroup>
                {#if edit_mode}
                  <Button
                    on:click={() => update(event.id)}
                    color="green"
                    disabled={!changed.includes(event.id)}>Save</Button
                  >
                  <Button
                    color="red"
                    disabled={!changed.includes(event.id)}
                    on:click={() => revert(event.id)}>Undo</Button
                  >
                {/if}
                <Button color="blue" href="/admin/event/{event.id}"
                  >Edit Details</Button
                ><Button color="yellow" href="/admin/teams/{event.id}"
                  >Team List</Button
                ><Button color="green" href="/admin/event/{event.id}/run"
                  >Run</Button
                ></ButtonGroup
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
