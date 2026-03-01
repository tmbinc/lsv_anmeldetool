<script lang="ts">
  import { onMount } from "svelte";
  import { getRooms, setRooms, type Room } from "../../../../../../api/api";
  import { page } from "$app/state";
  import FetchErrors from "../../../../../FetchErrors.svelte";
  import {
    Button,
    Input,
    Modal,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { CheckCircleOutline } from "flowbite-svelte-icons";

  const event_id = page.params.event_id || "";
  const group_id = page.params.group || "";

  let uploaded = $state(false);

  let fetch_errors: FetchErrors;

  let rooms: Room[] = $state([]);

  onMount(async () => {
    const request = getRooms({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      rooms = resp.data.filter((room) => room.group_id == group_id);
    } else {
      fetch_errors.check(resp);
    }
  });

  async function new_room() {
    rooms.push({
      event: event_id,
      group_id: group_id,
      room: "New Room",
      table_num_low: 0,
      table_num_high: 0,
    });
  }
  async function save_rooms() {
    let result = await setRooms({
      event: event_id,
      group: group_id,
      rooms: rooms,
    }).result;

    if (result.ok) {
      uploaded = true;
    } else {
      alert("warning - failed to save: " + result.data);
    }
  }
</script>

<main>
  <FetchErrors bind:this={fetch_errors} />

  <Table>
    <TableHead>
      <TableHeadCell>Table #low..#high</TableHeadCell>
      <TableHeadCell>Room Name</TableHeadCell>
    </TableHead>
    <TableBody>
      {#each rooms as room}
        <TableBodyRow>
          <TableBodyCell
            ><div class="inline-block">
              <Input
                type="number"
                class="inline-block w-16"
                bind:value={room.table_num_low}
              />
              to
              <Input
                type="number"
                class="inline-block w-16"
                bind:value={room.table_num_high}
              />
            </div>
          </TableBodyCell>
          <TableBodyCell><Input bind:value={room.room} /></TableBodyCell>
        </TableBodyRow>
      {/each}
      <TableBodyRow>
        <TableBodyCell></TableBodyCell>
        <TableBodyCell>
          <Button on:click={new_room}>New Room Mapping...</Button>
          <Button on:click={save_rooms}>Upload</Button>
        </TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>

  <Modal bind:open={uploaded} size="xs" autoclose>
    <div class="text-center">
      <CheckCircleOutline
        class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
      />
      <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
        Room setup saved!
      </h3>
      <Button color="alternative">Close</Button>
    </div>
  </Modal>
</main>
