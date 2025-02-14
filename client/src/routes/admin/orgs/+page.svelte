<script lang="ts">
  import { onMount } from "svelte";
  import {
    listOrgs,
    getOrg,
    type Org,
    updateOrg,
    addOrg,
  } from "../../../api/api";
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
  import type { ApiRequest } from "@cocreators-ee/apity";
  import type { components } from "../../../api/api.d";
  import FetchErrors from "../../FetchErrors.svelte";

  let loading = $state(false);
  let orgs: Org[] = $state([]);
  let changed: string[] = $state([]);
  let edit_mode = $state(false);
  let fetch_error: FetchErrors;

  onMount(async () => {
    loading = true;
    const request = listOrgs({}).resp.subscribe((resp) => {
      if (resp?.ok) {
        orgs = resp.data;
        changed = [];
        loading = false;
      } else {
        fetch_error.check(resp);
      }
    });
  });

  async function add() {
    const resp = await addOrg({ name: "new" }).result;
    if (resp.ok) {
      orgs.push(resp.data);
    } else {
      fetch_error.check(resp);
    }
  }

  function update(id: String) {
    let org_to_update = orgs.find((org) => org.id == id);
    if (org_to_update) {
      console.log("updating", org_to_update.id);
      updateOrg({ ...org_to_update, org: org_to_update.id });
      changed = changed.filter((item) => item != id);
    }
  }

  async function revert(id: String) {
    let i = orgs.findIndex((org) => org.id == id);
    if (i != -1) {
      console.log("revert", orgs[i].id);
      let resp = await getOrg({ org: orgs[i].id }).result;
      if (resp.ok) {
        orgs[i] = resp.data;
        changed = changed.filter((item) => item != id);
      } else {
        fetch_error.check(resp);
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
  <FetchErrors bind:this={fetch_error} />
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
        <TableHeadCell>Contact Name</TableHeadCell>
        <TableHeadCell>Contact Email</TableHeadCell>
        <TableHeadCell>Contact Phone</TableHeadCell>
        <TableHeadCell>Slug</TableHeadCell>
        <TableHeadCell>Public</TableHeadCell>
        <TableHeadCell>Edit</TableHeadCell>
      </TableHead>

      <TableBody>
        {#each orgs as org}
          <TableBodyRow
            class="org {changed.includes(org.id) ? 'changed' : 'unchanged'}"
          >
            <TableBodyCell
              ><Input
                disabled={!edit_mode}
                oninput={() => change(org.id)}
                bind:value={org.name}
              /></TableBodyCell
            >
            <TableBodyCell
              ><Input
                disabled={!edit_mode}
                oninput={() => change(org.id)}
                bind:value={org.contact_name}
              /></TableBodyCell
            >
            <TableBodyCell
              ><Input
                disabled={!edit_mode}
                oninput={() => change(org.id)}
                bind:value={org.contact_email}
              /></TableBodyCell
            >
            <TableBodyCell
              ><Input
                disabled={!edit_mode}
                oninput={() => change(org.id)}
                bind:value={org.contact_phone}
              /></TableBodyCell
            >
            <TableBodyCell
              ><Input
                disabled={!edit_mode}
                oninput={() => change(org.id)}
                bind:value={org.slug}
              /></TableBodyCell
            >
            <TableBodyCell>
              <Checkbox
                disabled={!edit_mode}
                oninput={() => change(org.id)}
                bind:checked={org.public}
              ></Checkbox>
            </TableBodyCell>
            <TableBodyCell>
              <ButtonGroup>
                <Button color="blue" href="/admin/org/{org.id}/"
                  >Details...</Button
                >
                <Button
                  on:click={() => update(org.id)}
                  color="green"
                  disabled={!changed.includes(org.id)}>Save</Button
                >
                <Button
                  color="red"
                  disabled={!changed.includes(org.id)}
                  on:click={() => revert(org.id)}>Undo</Button
                >
              </ButtonGroup>
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
    <Button disabled={!edit_mode} onclick={add}>Add new...</Button>
  {/if}

  <Button
    disabled={edit_mode}
    onclick={() => {
      edit_mode = true;
    }}>Edit</Button
  >
</main>

<style>
  .changed {
    background-color: "red";
  }
  .unchanged {
    background-color: "blue";
  }
</style>
