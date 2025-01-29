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
    ButtonGroup,
    Checkbox,
    Input,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
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
  <h1>Altergruppen</h1>
  <Table>
    <TableHead>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>Edit</TableHeadCell>
    </TableHead>
    <TableBody>
      {#each groups as group}
        <TableBodyRow>
          <TableBodyCell>
            <Input
              disabled={!edit_mode}
              bind:value={group.name}
              oninput={() => change(group.id)}
            /></TableBodyCell
          >
          <TableBodyCell>
            {#if edit_mode}
              <ButtonGroup>
                <Button
                  on:click={() => update(group.id)}
                  color="green"
                  disabled={!changed.includes(group.id)}>Save</Button
                >

                <Button color="red" on:click={() => delete_group(group.id)}
                  >Remove</Button
                >
              </ButtonGroup>
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
        <TableBodyCell></TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
</div>
