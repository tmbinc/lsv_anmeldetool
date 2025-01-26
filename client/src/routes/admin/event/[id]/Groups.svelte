<script lang="ts">
  let props = $props();
  import { onMount } from "svelte";
  import {
    createGroup,
    deleteGroup,
    getGroupsForEvent,
    updateGroup,
    type Group,
  } from "../../../../api/api";
  import {
    Button,
    Checkbox,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";

  let groups: Group[] = $state([]);
  let edit_mode = $state(false);
  let changed: string[] = $state([]);

  onMount(async () => {
    const resp_event_orgs = await getGroupsForEvent({ event: props.event_id })
      .result;
    if (resp_event_orgs.ok) {
      groups = resp_event_orgs.data;
    }
  });

  async function new_group() {
    const res = await createGroup({
      event: props.event_id,
      name: "Neue Altergruppe",
    }).result;
    if (res.ok) {
      groups.push(res.data);
    }
  }

  function change(id: string) {
    if (!changed.includes(id)) {
      changed.push(id);
    }
  }

  function update(id: string) {
    let group_to_update = groups.find((group) => group.id == id);
    if (group_to_update) {
      updateGroup(group_to_update);
      changed = changed.filter((item) => item != id);
    }
  }

  async function delete_group(id: string) {
    deleteGroup({ group: id });

    changed = changed.filter((item) => item != id);
    groups = groups.filter((group) => group.id != id);
  }
</script>

<div class="mb-6">
  Altergruppen
  <Table>
    <TableBody>
      {#each groups as group}
        <TableBodyRow>
          <TableBodyCell>
            <input
              disabled={!edit_mode}
              bind:value={group.name}
              oninput={() => change(group.id)}
            /></TableBodyCell
          >
          <TableBodyCell>
            {#if edit_mode}
              <Button
                on:click={() => update(group.id)}
                class="px-3 py-2 text-xs font-medium text-center text-white bg-green-700 rounded-lg hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
                disabled={!changed.includes(group.id)}>Save</Button
              >

              <Button
                class="px-3 py-2 text-xs font-medium text-center text-white bg-red-700 rounded-lg hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800"
                on:click={() => delete_group(group.id)}>Remove</Button
              >
            {/if}
          </TableBodyCell>
        </TableBodyRow>
      {/each}
      <TableBodyRow>
        <TableBodyCell
          ><Button disabled={!edit_mode} on:click={() => new_group()}
            >New...</Button
          >
          <Button
            disabled={edit_mode}
            onclick={() => {
              edit_mode = true;
            }}>Edit</Button
          ></TableBodyCell
        >
      </TableBodyRow>
    </TableBody>
  </Table>
</div>
