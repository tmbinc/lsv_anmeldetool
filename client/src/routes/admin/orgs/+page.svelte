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

  let import_tsv =
    $state(`	Name der Schule	E-Mail-Adresse	postalische Adresse	Homepage	1
Die	Test-Schule	 Test-Schule.Luebeck@schule.landsh.de	 Test-Straße 1a,  12345 Lübeck, Hansestadt 	keine Angabe	1
`);

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
      updateOrg({ ...org_to_update, org: org_to_update.id });
      changed = changed.filter((item) => item != id);
    }
  }

  async function revert(id: String) {
    let i = orgs.findIndex((org) => org.id == id);
    if (i != -1) {
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

  async function import_list() {
    const lines = import_tsv.split("\n");
    let header: string[] | undefined = undefined;

    for (const line of lines) {
      const columns = line.split("\t");
      if (header) {
        const h = header;
        let data = new Map(columns.map((val, index) => [h[index], val]));
        let org_name = data.get("Name der Schule") || "";
        let org_email = data.get("E-Mail-Adresse") || "";
        let org_address = data.get("postalische Adresse") || "";
        let org_genus = data.get("Artikel") || "";

        if (org_name) {
          orgs.push({
            contact_email: org_email.trim(),
            genus: { Der: "m", Die: "f", Das: "n" }[org_genus] || "",
            id: "import",
            last_update: "",
            name: org_name.trim(),
            name_additional: org_address.trim(),
            public: false,
            slug: "",
          });
        }
      } else {
        header = columns;
        header[header.findIndex((n) => n == "")] = "Artikel";
      }
    }
  }

  async function finish_import(org: Org) {
    const resp = await addOrg({ name: org.name }).result;
    if (resp.ok) {
      let new_org = resp.data;
      org.id = new_org.id;
      org.last_update = new_org.last_update;

      const resp_org_update = await updateOrg({ ...org, org: org.id }).result;
      if (!resp_org_update.ok) {
        alert("org update during import for " + org.name + "failed");
      }
    } else {
      alert("org new during import for " + org.name + "failed");
      fetch_error.check(resp);
    }
  }
  async function finish_import_all() {
    orgs.forEach((org) => {
      if (org.id == "import") {
        finish_import(org);
      }
    });
  }
</script>

<main class="px-2 md:px-10 pt-6 flex flex-col gap-4">
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
              class={orgs.filter((o) => o.slug == org.slug).length != 1
                ? "bg-red-600"
                : ""}
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
                {#if org.id != "import"}
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
                {:else}
                  <Button color="red" on:click={() => finish_import(org)}
                    >Import</Button
                  >
                {/if}
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

  <textarea rows="20" bind:value={import_tsv}> </textarea>

  <Button disabled={!edit_mode} on:click={import_list}>Import...</Button>

  <Button disabled={!edit_mode} on:click={finish_import_all}
    >Finish import...</Button
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
