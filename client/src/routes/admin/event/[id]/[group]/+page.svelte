<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import FetchErrors from "../../../../FetchErrors.svelte";
  import {
    getEvent,
    getEventOrgs,
    getGroup,
    getGroupsForEvent,
    getTeamsForOrgEvent,
    setPairings,
    type Event,
    type Group,
    type Org,
    type Pairing,
    type Team,
  } from "../../../../../api/api";
  import {
    Button,
    Modal,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { validators } from "tailwind-merge";
  import {
    CheckCircleOutline,
    ExclamationCircleOutline,
  } from "flowbite-svelte-icons";

  type Result = "1:0" | "0:1" | "0.5:0.5" | "-:-" | "+:-" | "-:+" | "";

  type PairingTeam = {
    team: Team;
    org: Org;
  };

  type PairingDisplay = {
    team_home: PairingTeam | null;
    team_guest: PairingTeam | null;
    points_home: number | null;
    points_guest: number | null;
    table: number;
    result: Result;
  };

  let event: Event | null = $state(null);
  let pairings: PairingDisplay[] = $state([]);
  let group: Group | null = $state(null);
  let fetch_errors: FetchErrors;
  let uploaded = $state(false);
  const event_id = page.params.id;
  const group_id = page.params.group;
  let teams: PairingTeam[] = $state([]);

  $inspect(teams);

  let swisschess_list = $state(`

`);

  onMount(async () => {
    const request = getEvent({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      event = resp.data;
    } else {
      fetch_errors.check(resp);
    }

    const resp_group = await getGroup({
      group: group_id,
    }).result;

    if (resp_group.ok) {
      group = resp_group.data;
    } else {
      fetch_errors.check(resp_group);
    }

    const resp_event_orgs = await getEventOrgs({ event: event_id }).result;

    if (resp_event_orgs.ok) {
      const team_list: PairingTeam[] = [];

      for (const event_org of resp_event_orgs.data) {
        let org_teams = await getTeamsForOrgEvent({
          org: event_org.org.id,
          event: event_id,
        }).result;

        if (org_teams.ok) {
          const new_teams = org_teams.data
            .filter((team) => team.group_id == group_id)
            .map((team) => ({
              team: team,
              org: event_org.org,
            }));
          team_list.push(...new_teams);
        } else {
          fetch_errors.check(org_teams);
        }
      }

      // todo: we should have the team index
      team_list.sort(
        (a, b) =>
          a.team.name.localeCompare(b.team.name) ||
          a.team.name.localeCompare(b.team.name)
      );
      teams = team_list;
    } else {
      fetch_errors.check(resp_event_orgs);
    }
  });

  function team_for_name(
    name: string | undefined,
    slug: string | undefined
  ): PairingTeam | null {
    if (!name || !slug) {
      return null;
    }
    for (const team of teams) {
      if (team.team.name.startsWith(name) && team.org.slug == slug) {
        return team;
      }
    }
    return null;
  }

  async function import_list() {
    const lines = swisschess_list.split("\n");
    let header: string[] | undefined = undefined;
    let pairing_list: PairingDisplay[] = [];
    for (const line of lines) {
      const columns = line.split("\t");
      if (columns.length < 3) {
        continue;
      }
      if (header) {
        let data = new Map(columns.map((val, index) => [header[index], val]));
        const team_home = team_for_name(
          data.get("Mannschaft_home"),
          data.get("Land_home")
        );
        const team_guest = team_for_name(
          data.get("Mannschaft_guest"),
          data.get("Land_guest")
        );
        const table = +(data.get("PaarNr") || "");

        pairing_list.push({
          team_home: team_home,
          points_home: +(data.get("Punkte_home") ?? ""),
          team_guest: team_guest,
          points_guest: +(data.get("Punkte_guest") ?? ""),
          result: "",
          table: table,
        });
      } else {
        header = columns;
        header[header.findIndex((n) => n == "MNr")] += "_home";
        header[header.findIndex((n) => n == "MNr")] += "_guest";
        header[header.findIndex((n) => n == "Mannschaft")] += "_home";
        header[header.findIndex((n) => n == "Mannschaft")] += "_guest";
        header[header.findIndex((n) => n == "Land")] += "_home";
        header[header.findIndex((n) => n == "Land")] += "_guest";
        header[header.findIndex((n) => n == "Punkte")] += "_home";
        header[header.findIndex((n) => n == "Punkte")] += "_guest";
      }
    }
    pairings = pairing_list;
  }

  async function upload_pairings() {
    let pairings_upload: Pairing[] = pairings.map((pairing) => ({
      event: event_id,
      group_id: group_id,
      result: pairing.result,
      round: 0,
      table_num: pairing.table,
      team_guest: pairing.team_guest?.team.id,
      team_home: pairing.team_home?.team.id,
    }));

    let resp = await setPairings({
      event: event_id,
      group: group_id,
      round: 0,
      pairings: pairings_upload,
    }).result;
    if (resp.ok) {
      uploaded = true;
    } else {
      fetch_errors.check(resp);
    }
  }
</script>

<main class="px-2 md:px-10 pt-6 flex flex-col gap-4">
  <FetchErrors bind:this={fetch_errors} />

  <div class="text-3xl font-bold">{group?.name}</div>

  <Button color="yellow" href="/admin/event/{event_id}/{group_id}/rooms"
    >Edit Room Mapping...</Button
  >

  <Button
    color="blue"
    disabled={pairings.length == 0}
    on:click={upload_pairings}>Upload this round...</Button
  >
  <div class="text-5xl">Preview</div>
  <Table shadow>
    <TableHead>
      <TableHeadCell>Table</TableHeadCell>
      <TableHeadCell>Home</TableHeadCell>
      <TableHeadCell>Result</TableHeadCell>
      <TableHeadCell>Guest</TableHeadCell>
    </TableHead>
    <TableBody>
      {#each pairings as pairing}
        <TableBodyRow
          class={" " +
            (pairing.team_home && pairing.team_guest
              ? "bg-green-200"
              : "bg-red-200")}
        >
          <TableBodyCell>{pairing.table}</TableBodyCell>
          <TableBodyCell
            >{pairing.team_home?.team.name}
            <sub>{pairing.team_home?.org.name}</sub></TableBodyCell
          >
          <TableBodyCell>{pairing.result}</TableBodyCell>
          <TableBodyCell
            >{pairing.team_guest?.team.name}
            <sub>{pairing.team_guest?.org.name}</sub></TableBodyCell
          >
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>

  <textarea rows="20" bind:value={swisschess_list}> </textarea>

  <Button on:click={import_list}>Import...</Button>

  <Modal bind:open={uploaded} size="xs" autoclose>
    <div class="text-center">
      <CheckCircleOutline
        class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
      />
      <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
        Upload ok!
      </h3>
      <Button color="alternative">Close</Button>
    </div>
  </Modal>
</main>
