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
    CloseButton,
    Drawer,
    Label,
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    type SelectOptionType,
  } from "flowbite-svelte";
  import { writable } from "svelte/store";
  import { browser } from "$app/environment";
  import { sineIn } from "svelte/easing";
  import {
    AdjustmentsHorizontalOutline,
    InfoCircleSolid,
  } from "flowbite-svelte-icons";

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
  let round: number | undefined = $state(undefined);
  let orgs: Set<string> = new Set();
  let orgs_select: SelectOptionType<string>[] = $state([]);
  let org_selected = $state(
    (browser && localStorage.getItem("org_selected")) ?? "*"
  );
  let hide_config = $state(org_selected != "*" || group_selected != "*");
  let transitionParams = {
    x: -320,
    duration: 200,
    easing: sineIn,
  };

  const event_id = page.params.id;

  onMount(async () => {
    getPairings({ event: event_id }).resp.subscribe((resp) => {
      if (resp?.ok) {
        pairings = resp.data.pairings;
        groups = resp.data.groups;
        event_name = resp.data.event_name;
        rooms = resp.data.rooms;
        pairings.sort((a, b) => a.table - b.table);
        let rounds = new Set<number>();

        for (let pairing of pairings) {
          if (pairing.team_home_org) {
            orgs.add(pairing.team_home_org);
          }
          if (pairing.team_guest_org) {
            orgs.add(pairing.team_guest_org);
          }
          rounds.add(pairing.round);
        }

        const [first] = rounds;

        round = first;

        groups_select = groups
          .filter((group) => !group.replacement)
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

<main class="md:mx-10">
  {#if loading}
    <Loading text="Lade Turnierdetails..." />
  {:else if failed_load}
    <LoadError text="Laden fehlgeschlagen!" />
  {:else}
    <div>{event_name}</div>
    <div><Label class="text-5xl m-10">Runde {round}</Label></div>

    <div class="text-center m-5">
      <Button on:click={() => (hide_config = false)}
        ><AdjustmentsHorizontalOutline /> Filter...</Button
      >
    </div>

    <Drawer
      transitionType="fly"
      {transitionParams}
      bind:hidden={hide_config}
      id="sidebar1"
    >
      <div class="flex items-center">
        <h5
          id="drawer-label"
          class="inline-flex items-center mb-4 text-base font-semibold text-gray-500 dark:text-gray-400"
        >
          <InfoCircleSolid class="w-5 h-5 me-2.5" />Filter
        </h5>
        <CloseButton
          on:click={() => (hide_config = true)}
          class="mb-4 dark:text-white"
        />
      </div>
      <div class="flex mt-5">
        <Label>Bitte die anzuzeigenden Gruppen auswählen:</Label>
      </div>

      <div class="flex mt-5">
        <Label>Schule:</Label>
      </div>
      <div class="flex mt-5">
        <Select
          items={orgs_select}
          bind:value={org_selected}
          on:change={update_local_storage}
        />
      </div>
      <div class="flex mt-5">
        <Label>Altersgruppe:</Label>
      </div>
      <div class="flex mt-5">
        <Select
          items={groups_select}
          bind:value={group_selected}
          on:change={update_local_storage}
        />
      </div>
      <div class="flex mt-5">
        <Button
          on:click={() => (hide_config = true)}
          class="mb-4 dark:text-white">OK</Button
        >
      </div>
    </Drawer>

    {#each groups as group}
      {#if group_selected == "*" || group_selected == group.id}
        <div class="text-xl">{group.name}</div>
        <div class="table-div-class">
          <Table striped={true} hoverable={true} shadow class="text-sm">
            <TableBody>
              {#each pairings as pairing}
                {#if pairing.group == group.id && (org_selected == "*" || org_selected == pairing.team_home_org || org_selected == pairing.team_guest_org)}
                  <TableBodyRow class="tr-class">
                    <TableBodyCell class="td-class">
                      <div>Runde {pairing.round}</div>
                      <div>
                        Brett {pairing.table} ({findRoom(
                          group.id,
                          pairing.table
                        )})
                      </div></TableBodyCell
                    >
                    <TableBodyCell class="td-class">
                      <div class="text-xl">{pairing.team_home}</div>
                      <div><sub>{pairing.team_home_org}</sub></div>
                      <div>
                        ({pairing.points_home})

                        <tt class="bg-black text-white">1</tt>
                        <tt class="text-black bg-white">2</tt>
                        <tt class="bg-black text-white">3</tt>
                        <tt class="text-black bg-white">4</tt>
                        ...
                      </div>
                    </TableBodyCell>
                    <TableBodyCell class="td-class">:</TableBodyCell>
                    <TableBodyCell class="td-class"
                      ><div class="text-xl">{pairing.team_guest}</div>
                      <div><sub>{pairing.team_guest_org}</sub></div>
                      <div>
                        ({pairing.points_guest})

                        <tt class="text-black bg-white">1</tt>
                        <tt class="bg-black text-white">2</tt>
                        <tt class="text-black bg-white">3</tt>
                        <tt class="bg-black text-white">4</tt>
                        ...
                      </div>
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
    @apply flex flex-col mb-4 sm:table-row border-black;
  }
  :global(.table-div-class) {
    @apply flex sm:justify-normal justify-center;
  }
</style>
