<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import FetchErrors from "../../../../FetchErrors.svelte";
  import {
    getEvent,
    getEventOrgs,
    getGroup,
    getGroupsForEvent,
    setPairings,
    type Event,
    type Group,
    type Org,
    type Pairing,
    type Team,
    type TeamResult,
    setResults,
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

  type GameResult = "1:0" | "0:1" | "0.5:0.5" | "-:-" | "+:-" | "-:+" | "";

  type PairingTeam = {
    team: Team;
    org: Org;
  };

  type PairingDisplay = {
    team_home: PairingTeam | undefined;
    team_guest: PairingTeam | null | undefined;
    points_home: number | null;
    points_guest: number | null;
    table: number;
    result: GameResult;
  };

  type ResultDisplay = {
    team: PairingTeam | undefined;
    rank: number | null;
    points_team: number | null;
    points_player: number | null;
    tie: number | null;
  };

  let event: Event | null = $state(null);
  let pairings: PairingDisplay[] = $state([]);
  let results: ResultDisplay[] = $state([]);
  let group: Group | null = $state(null);
  let fetch_errors: FetchErrors;
  let uploaded = $state(false);
  let round = $state(0);
  const event_id = page.params.event_id;
  const group_id = page.params.group;
  let teams: PairingTeam[] = $state([]);
  let groups: Group[] = $state([]);
  let group_replacement = $state(new Map<string, string>());

  let swisschess_list = $state(`


Mannschafts-Rangliste: Stand nach der 2. Runde 
Rang	MNr	Mannschaft		TWZ	ELO	NWZ	Attr.	Land	G	S	R	V	Man.Pkt.	Man.Pkt.		Brt.Pkt.	Brt.Pkt.		Buchh
1	1	Mannschaft 3		1400	1400	1400	 	C	1	1	0	0	2	-	0	3.0	-	1.0	3.0
2	2	Team 2		1400	1400	1400	 	C	1	0	1	0	1	-	1	2.0	-	2.0	5.0
3	4	Team 2		1400	1400	1400	 	A	1	0	1	0	1	-	1	2.0	-	2.0	3.0
4	3	Team 4		1400	1400	1400	 	C	1	0	0	1	0	-	2	1.0	-	3.0	5.0



Paarungsliste der 2. Runde  
PaarNr	MNr	Mannschaft		TWZ	T	Attr.	Verein	Land	Punkte	-	MNr	Mannschaft		TWZ	T	Attr.	Verein	Land	Punkte	Erg.	Ergebnis	Erg.	At.
1	1	Mannschaft 3		1400		 		C	(2)	-	2	Team 2		1400		 		C	(1)		-		 
2	4	Team 2		1400		 		A	(1)	-	3	Team 4		1400		 		C	(0)		-		 


`);

  onMount(async () => {
    const request = getEvent({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      event = resp.data;
    } else {
      fetch_errors.check(resp);
    }

    const resp_groups = await getGroupsForEvent({ event: event_id }).result;
    if (resp_groups.ok) {
      groups = resp_groups.data;
    } else {
      fetch_errors.check(resp_groups);
    }

    group_replacement = new Map(
      groups.map((group) => [group.id, group.replacement || group.id])
    );

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
        const new_teams = event_org.teams
          .filter(
            (team) => group_replacement.get(team.group_id || "") == group_id
          )
          .map((team) => ({
            team: team,
            org: event_org.org,
          }));
        team_list.push(...new_teams);
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
  ): PairingTeam | null | undefined {
    if (!name || !slug) {
      return null;
    }
    for (const team of teams) {
      if (team.team.name.startsWith(name) && team.org.slug == slug) {
        return team;
      }
    }

    if (slug == "spielfrei") {
      return null;
    }

    return undefined;
  }

  async function import_list() {
    const lines = swisschess_list.split("\n");
    let header: string[] | undefined = undefined;
    let pairing_list: PairingDisplay[] = [];
    let result_list: ResultDisplay[] = [];

    let have_pairing = false;
    let have_results = false;
    let in_pairing = false;
    let in_results = false;

    for (const line of lines) {
      if (line.startsWith("Paarungsliste")) {
        round = +line.split(" ")[2];
        have_pairing = true;
        in_pairing = true;
        in_results = false;
        header = undefined;
      }
      if (line.startsWith("Mannschafts-Rangliste")) {
        round = +line.split(" ")[4];
        have_results = true;
        in_results = true;
        in_pairing = false;
        header = undefined;
      }
      const columns = line.split("\t");
      if (columns.length < 3) {
        continue;
      }
      if (header) {
        if (in_pairing) {
          let data = new Map(
            columns.map((val, index) => [header![index], val])
          );
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
            team_home: team_home ?? undefined,
            points_home: +(data.get("Punkte_home") ?? "")
              .replace("(", "")
              .replace(")", ""),
            team_guest: team_guest,
            points_guest: +(data.get("Punkte_guest") ?? "")
              .replace("(", "")
              .replace(")", ""),
            result: "",
            table: table,
          });
        }

        if (in_results) {
          let data = new Map(
            columns.map((val, index) => [header![index], val])
          );
          console.log(data);
          const team = team_for_name(data.get("Mannschaft"), data.get("Land"));
          const rank = +(data.get("Rang") || "");
          const points_team = Math.floor(+(data.get("Man.Pkt._won") || ""));
          const points_player = Math.floor(+(data.get("Brt.Pkt._won") || ""));
          const tie = Math.floor(+(data.get("Buchh") || "")); // FIXME

          result_list.push({
            team: team ?? undefined,
            rank: rank,
            points_team: points_team,
            points_player: points_player,
            tie: tie,
          });
        }
      } else {
        header = columns;
        if (in_pairing) {
          header[header.findIndex((n) => n == "MNr")] += "_home";
          header[header.findIndex((n) => n == "MNr")] += "_guest";
          header[header.findIndex((n) => n == "Mannschaft")] += "_home";
          header[header.findIndex((n) => n == "Mannschaft")] += "_guest";
          header[header.findIndex((n) => n == "Land")] += "_home";
          header[header.findIndex((n) => n == "Land")] += "_guest";
          header[header.findIndex((n) => n == "Punkte")] += "_home";
          header[header.findIndex((n) => n == "Punkte")] += "_guest";
        }
        if (in_results) {
          header[header.findIndex((n) => n == "Man.Pkt.")] += "_won";
          header[header.findIndex((n) => n == "Man.Pkt.")] += "_lost";
          header[header.findIndex((n) => n == "Brt.Pkt.")] += "_won";
          header[header.findIndex((n) => n == "Brt.Pkt.")] += "_lost";
        }
      }
    }
    pairings = pairing_list;
    results = result_list;
  }

  async function uploadPairings() {
    let pairings_upload: Pairing[] = pairings.map((pairing) => ({
      event: event_id,
      group_id: group_id,
      result: pairing.result,
      round: round,
      table_num: pairing.table,
      team_home: pairing.team_home?.team.id,
      team_guest: pairing.team_guest?.team.id,
      points_home: pairing.points_home,
      points_guest: pairing.points_guest,
    }));

    let resp = await setPairings({
      event: event_id,
      group: group_id,
      round: round,
      pairings: pairings_upload,
    }).result;
    if (resp.ok) {
      uploaded = true;
    } else {
      fetch_errors.check(resp);
    }
  }

  async function uploadResults() {
    let results_upload: TeamResult[] = results.map((result) => ({
      event: event_id,
      group_id: group_id,
      round: round,
      team: result.team?.team.id,
      rank: result.rank,
      points_team: result.points_team,
      points_player: result.points_player,
      tie: result.tie,
    }));

    let resp = await setResults({
      event: event_id,
      group: group_id,
      round: round,
      results: results_upload,
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

  <div class="text-3xl font-bold">{group?.name} - Round {round}</div>

  <Button color="green" href="/event/{event_id}/pairings"
    >Public Pairing Link...</Button
  >
  <Button color="yellow" href="/admin/event/{event_id}/{group_id}/rooms"
    >Edit Room Mapping...</Button
  >

  <Button color="blue" disabled={pairings.length == 0} on:click={uploadPairings}
    >Upload PAIRINGS for round {round}...</Button
  >
  <Button color="blue" disabled={results.length == 0} on:click={uploadResults}
    >Upload Results of round {round}...</Button
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
            (pairing.team_home !== undefined && pairing.team_guest !== undefined
              ? "bg-green-200"
              : "bg-red-200")}
        >
          <TableBodyCell>{pairing.table}</TableBodyCell>
          <TableBodyCell>
            {#if pairing.team_home !== null}
              {pairing.team_home?.team.name}
              <sub>{pairing.team_home?.org.name}</sub>
              ({pairing.points_home})
            {:else}
              <i>spielfrei</i>
            {/if}
          </TableBodyCell>
          <TableBodyCell>{pairing.result}</TableBodyCell>
          <TableBodyCell>
            {#if pairing.team_guest !== null}
              {pairing.team_guest?.team.name}
              <sub>{pairing.team_guest?.org.name}</sub>
              ({pairing.points_guest})
            {:else}
              <i>spielfrei</i>
            {/if}
          </TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>

  <Table>
    <TableHead>
      <TableHeadCell>Team</TableHeadCell>
      <TableHeadCell>Rank</TableHeadCell>
      <TableHeadCell>Points (Team)</TableHeadCell>
      <TableHeadCell>Points (Player)</TableHeadCell>
      <TableHeadCell>Tie</TableHeadCell>
    </TableHead>
    <TableBody>
      {#each results as result}
        <TableBodyRow
          class={" " +
            (result.team !== undefined ? "bg-green-200" : "bg-red-200")}
        >
          <TableBodyCell>
            {#if result.team !== null}
              {result.team?.team.name}
              <sub>{result.team?.org.name}</sub>
            {/if}
          </TableBodyCell>
          <TableBodyCell>{result.rank}</TableBodyCell>
          <TableBodyCell>{result.points_team}</TableBodyCell>
          <TableBodyCell>{result.points_player}</TableBodyCell>
          <TableBodyCell>{result.tie}</TableBodyCell>
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
