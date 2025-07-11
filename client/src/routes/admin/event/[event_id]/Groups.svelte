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
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    type SelectOptionType,
  } from "flowbite-svelte";

  let groups: Group[] = $state([]);
  let edit_mode = $state(false);
  let changed: string[] = $state([]);

  let group_selection: SelectOptionType<string>[] = $derived(
    [{ name: "", value: "" }].concat(
      groups.map((group) => ({
        name: group.name,
        value: group.id,
      }))
    )
  );

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
      if (
        group_to_update.replacement == "" ||
        group_to_update.replacement == group_to_update.id
      ) {
        group_to_update.replacement = null;
      }
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
      <TableHeadCell>Slug ("Attribute" in swiss-chess)</TableHeadCell>
      <TableHeadCell>Color</TableHeadCell>
      <TableHeadCell>Merge into...</TableHeadCell>
      <TableHeadCell>Edit</TableHeadCell>
    </TableHead>
    <TableBody>
      {#each groups as group}
        <TableBodyRow class="bg-{group.color}-100">
          <TableBodyCell class="w-6/12">
            <Input
              disabled={!edit_mode}
              bind:value={group.name}
              oninput={() => change(group.id)}
            />
          </TableBodyCell>
          <TableBodyCell class="w-1/12">
            <Input
              disabled={!edit_mode}
              bind:value={group.slug}
              oninput={() => change(group.id)}
            />
          </TableBodyCell>
          <TableBodyCell class="w-1/12">
            <Input
              disabled={!edit_mode}
              bind:value={group.color}
              oninput={() => change(group.id)}
            />
          </TableBodyCell>
          <TableBodyCell class="w-4/12">
            <Select
              disabled={!edit_mode}
              items={group_selection}
              bind:value={group.replacement}
              oninput={() => change(group.id)}
            />
          </TableBodyCell>
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
