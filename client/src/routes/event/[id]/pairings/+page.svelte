<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import {
    getPairings,
    type Group,
    type PairingEntry,
    type Room,
  } from "../../../../api/api";
  import Loading from "../../../Loading.svelte";
  import LoadError from "../../../LoadError.svelte";
  import {
    Button,
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    type SelectOptionType,
  } from "flowbite-svelte";
  import { writable } from "svelte/store";
  import { browser } from "$app/environment";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  let pairings: PairingEntry[] = $state([]);
  let groups: Group[] = $state([]);
  let groups_select: SelectOptionType<string>[] = $state([]);
  let group_selected = $state(
    (browser && localStorage.getItem("group_selected")) ?? "*"
  );
  let rooms: Room[] = $state([]);
  let orgs: Set<string> = new Set();
  let orgs_select: SelectOptionType<string>[] = $state([]);
  let org_selected = $state(
    (browser && localStorage.getItem("org_selected")) ?? "*"
  );

  const event_id = page.params.id;

  onMount(async () => {
    getPairings({ event: event_id }).resp.subscribe((resp) => {
      if (resp?.ok) {
        pairings = resp.data.pairings;
        groups = resp.data.groups;
        event_name = resp.data.event_name;
        rooms = resp.data.rooms;
        pairings.sort((a, b) => a.table - b.table);
        for (let pairing of pairings) {
          if (pairing.team_home_org) {
            orgs.add(pairing.team_home_org);
          }
          if (pairing.team_guest_org) {
            orgs.add(pairing.team_guest_org);
          }
        }

        groups_select = groups
          .map((group) => ({
            name: group.name,
            value: group.id,
          }))
          .toSorted((a, b) => a.name.localeCompare(b.name));
        groups_select.unshift({ name: "Alle", value: "*" });

        orgs_select = orgs
          .keys()
          .toArray()
          .map((org) => ({
            name: org,
            value: org,
          }))
          .toSorted((a, b) => a.name.localeCompare(b.name));
        orgs_select.unshift({ name: "Alle", value: "*" });

        loading = false;
      }
    });
  });

  function findRoom(group_id: string, table: number) {
    for (let room of rooms) {
      if (
        room.group_id == group_id &&
        room.table_num_low <= table &&
        room.table_num_high >= table
      ) {
        return room.room;
      }
    }
    return null;
  }

  function update_local_storage() {
    localStorage.setItem("org_selected", org_selected || "*");
    localStorage.setItem("group_selected", group_selected || "*");
  }
</script>

<main class="m-6 md:mx-10">
  {#if loading}
    <Loading text="Lade Turnierdetails..." />
  {:else if failed_load}
    <LoadError text="Laden fehlgeschlagen!" />
  {:else}
    <div>{event_name}</div>
    <div>Aktuelle Paarungsliste:</div>

    <div class="flex">
      <Select
        items={orgs_select}
        bind:value={org_selected}
        on:change={update_local_storage}
      />
    </div>
    <div class="flex">
      <Select
        items={groups_select}
        bind:value={group_selected}
        on:change={update_local_storage}
      />
    </div>

    <div class="flex">
      {#each rooms as room}
        <div>
          <Button>{room.room}</Button>
        </div>
      {/each}
    </div>

    {#each groups as group}
      {#if group_selected == "*" || group_selected == group.id}
        <div class="text-1xl">{group.name}</div>
        <div class="table-div-class">
          <Table hoverable={true} shadow class="text-sm">
            <TableBody>
              {#each pairings as pairing}
                {#if pairing.group == group.id && (org_selected == "*" || org_selected == pairing.team_home_org || org_selected == pairing.team_guest_org)}
                  <TableBodyRow class="tr-class">
                    <TableBodyCell class="td-class"
                      >Brett {pairing.table} ({findRoom(
                        group.id,
                        pairing.table
                      )})</TableBodyCell
                    >
                    <TableBodyCell class="td-class">
                      {pairing.team_home} <sub>{pairing.team_home_org}</sub>
                      ({pairing.points_home})
                    </TableBodyCell>
                    <TableBodyCell class="td-class"
                      >{pairing.team_guest} <sub>{pairing.team_guest_org}</sub>
                      ({pairing.points_guest})
                    </TableBodyCell>
                  </TableBodyRow>
                {/if}
              {/each}
            </TableBody>
          </Table>
        </div>
      {/if}
    {/each}
  {/if}
</main>

<style lang="postcss">
  :global(.td-class) {
    @apply px-4 py-3;
  }
  :global(.tr-class) {
    @apply flex flex-col mb-4 sm:table-row;
  }
  :global(.table-div-class) {
    @apply flex sm:justify-normal justify-center ml-4 sm:ml-0;
  }
</style>
