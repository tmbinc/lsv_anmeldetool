<script lang="ts">
  import { onMount } from "svelte";
  import { listOrgs, getOrg, type Org, updateOrg } from "../../../api/api";
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
  let orgs: Org[] = $state([]);

  onMount(async () => {
    loading = true;
    const request = listOrgs(undefined);
    const resp = await request.result;
    if (resp.ok) {
      orgs = resp.data;
    }
    loading = false;
  });

  function add() {
    orgs.push({
      name: "new",
      id: "",
      public: false,
    });
    console.log(orgs);
  }

  function update(id: String) {
    let org_to_update = orgs.find((org) => org.id == id);
    if (org_to_update) {
      console.log("updating", org_to_update.id);
      updateOrg({ ...org_to_update, org: org_to_update.id });
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
        <TableHeadCell>Contact Email</TableHeadCell>
        <TableHeadCell>Contact Phone</TableHeadCell>
        <TableHeadCell>Public</TableHeadCell>
        <TableHeadCell>Edit</TableHeadCell>
      </TableHead>

      <TableBody>
        {#each orgs as org}
          <TableBodyRow>
            <TableBodyCell
              ><div
                contenteditable="true"
                bind:innerText={org.name}
              ></div></TableBodyCell
            >
            <TableBodyCell
              ><div
                contenteditable="true"
                bind:innerText={org.contact_email}
              ></div></TableBodyCell
            >
            <TableBodyCell
              ><div
                contenteditable="true"
                bind:innerText={org.contact_phone}
              ></div></TableBodyCell
            >
            <TableBodyCell>
              <Checkbox bind:checked={org.public}></Checkbox>
            </TableBodyCell>
            <TableBodyCell
              ><Button on:click={() => update(org.id)}>Update</Button
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
