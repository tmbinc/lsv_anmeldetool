<script lang="ts">
  import { onDestroy, onMount, type Component } from "svelte";
  import {
    Button,
    ButtonGroup,
    Checkbox,
    Label,
    Modal,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Textarea,
  } from "flowbite-svelte";
  import {
    clearChangeSinceExport,
    getEventOrgs,
    getGroup,
    getGroupsForEvent,
    setTeamPresenceState,
    type EventOrg,
    type EventOrgState,
    type Group,
    type Org,
    type Team,
  } from "../../../../../../../api/api";

  import { page } from "$app/state";
  import { Table } from "flowbite-svelte";
  import FetchErrors from "../../../../../../FetchErrors.svelte";
  import {
    BadgeCheckOutline,
    BadgeCheckSolid,
    ExclamationCircleOutline,
    QuestionCircleSolid,
  } from "flowbite-svelte-icons";

  let event_id = page.params.event_id;
  let group_id = page.params.group_id;
  let fetch_errors: FetchErrors;
  let filter_ready = $state(false);

  let group: Group | undefined = $state(undefined);
  let groups: Group[] = $state([]);
  let group_names = $state(new Map<string, string>());
  let group_slugs = $state(new Map<string, string>());
  let group_replacement = $state(new Map<string, string>());
  let slug_org_decode = $state("");
  let slug_group_decode = $state("");
  let confirm_changes = $state(true);
  let verify_change = $state(false);
  let verify_team_new_state: string | null = $state(null);
  let verify_team: OrgTeam | null = $state(null);
  let also_clear_changes_modal = $state(false);
  let evtSource: EventSource | null = null;

  type OrgTeam = {
    org: Org;
    team: Team;
    org_state: EventOrgState;
  };

  let teams: OrgTeam[] = $state([]);

  type SwissChessTeam = {
    number: string;
    teamname: string;
    federation: string;
    rank: string;
    state: string;
  };

  onMount(async () => {
    if (group_id != "all") {
      const resp_group = await getGroup({
        group: group_id,
      }).result;

      if (resp_group.ok) {
        group = resp_group.data;
      } else {
        fetch_errors.check(resp_group);
      }
    }

    const resp_groups = await getGroupsForEvent({
      event: event_id,
    }).result;

    if (resp_groups.ok) {
      groups = resp_groups.data;
    } else {
      fetch_errors.check(resp_groups);
    }
    group_names = new Map(groups.map((group) => [group.id, group.name]));
    group_slugs = new Map(groups.map((group) => [group.id, group.slug]));
    group_replacement = new Map(
      groups.map((group) => [group.id, group.replacement || group.id])
    );

    const resp_event_orgs = await getEventOrgs({ event: event_id }).result;

    if (resp_event_orgs.ok) {
      const team_list: OrgTeam[] = [];

      let slugdecode = 'Alias$(XManLand, "';

      for (const event_org of resp_event_orgs.data) {
        const new_teams = event_org.teams
          .filter(
            (team) =>
              group_id == "all" ||
              group_replacement.get(team.group_id || "") == group_id
          )
          .map((team) => ({
            org: event_org.org,
            team: team,
            org_state: event_org.state,
          }));
        team_list.push(...new_teams);
        slugdecode += event_org.org.slug + "=" + event_org.org.name + "|";
      }
      slug_org_decode =
        slugdecode.substring(0, slugdecode.length - 1) + '")' + "\n";
      slug_group_decode =
        'Alias$(XManAttr, "' +
        groups.map((group) => group.slug + "=" + group.name).join("|") +
        '")';

      team_list.sort(
        (a, b) =>
          a.org.name.localeCompare(b.org.name) ||
          a.team.name.localeCompare(b.team.name)
      );
      teams = team_list;
    } else {
      fetch_errors.check(resp_event_orgs);
    }

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = function (event) {
      var dataobj = JSON.parse(event.data);
      if (dataobj.kind == "team_changed") {
        let ch_team = dataobj.team;
        let ch_state = dataobj.state;
        teams.forEach((team) => {
          if (team.team.id == ch_team) {
            team.team.presence_state = ch_state;
          }
        });
      }
    };
  });

  onDestroy(() => {
    evtSource?.close();
  });

  async function set_team_presence_state(team: OrgTeam, new_state: string) {
    team.team.presence_state = new_state;
    let res = await setTeamPresenceState({
      team_id: team.team.id,
      presence_state: team.team.presence_state,
    }).result;
    if (res.ok) {
    } else {
      fetch_errors.check(res);
      alert("failed to set team readiness");
    }
  }

  async function set_team_presence_state_verify(
    team: OrgTeam,
    new_state: string
  ) {
    verify_team = team;
    verify_change = true;
    verify_team_new_state = new_state;
  }

  async function download_swisschess(clear_flags: boolean) {
    // TODO: write back team numbers to database.
    // TODO: build slug -> name lookup for printing
    // TODO: build shortened name -> full name lookup for printing
    let exported_groups: string[] = [];

    const swiss_teams: SwissChessTeam[] = teams.map((n, i) => ({
      number: (i + 1).toString(),
      teamname: n.team.name,
      federation: n.org.slug,
      select: group_slugs.get(n.team.group_id ?? ""),
      rank: "0",
      state: "",
    }));

    if (clear_flags) {
      teams.forEach((team) => {
        if (!exported_groups.includes(team.team.group_id ?? "")) {
          exported_groups.push(team.team.group_id ?? "");
        }
        team.team.changed_since_export = false;
      });
    }

    let data = { teams: swiss_teams };
    let data_string = JSON.stringify(data);

    var uint8 = new Uint8Array(data_string.length);
    for (var i = 0; i < uint8.length; i++) {
      uint8[i] = data_string.charCodeAt(i);
    }

    const blob = new Blob([uint8], {
      type: "text/plain;charset=ISO-8859-1",
    });

    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = (group?.name || "WK") + ".json";
    link.style.display = "none";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);

    if (clear_flags) {
      exported_groups.forEach((group_id) => {
        clearChangeSinceExport({ group: group_id }).resp;
      });
    }
  }
</script>

<div class="m-6 md:mx-10">
  <div class="text-4xl items-center">Teams</div>
  <div>
    {#if group_id != "all"}
      <div class="inline text-4xl">Gruppe</div>
      <div class="inline text-4xl">{group?.name}</div>
    {:else}
      <div class="inline text-4xl">All Groups</div>
    {/if}
  </div>
  <FetchErrors bind:this={fetch_errors} />
  <div class="m-5">
    <Checkbox bind:checked={filter_ready}>Show only ready</Checkbox>
    <Checkbox bind:checked={confirm_changes}>Confirm Changes</Checkbox>
  </div>
  <div class="table-div-class">
    <Table hoverable={true} shadow class="text-sm">
      <TableHead class="sm:table-header-group hidden">
        <TableHeadCell class="td-class">#</TableHeadCell>
        <TableHeadCell class="td-class">Name</TableHeadCell>
        <TableHeadCell class="td-class">Schule</TableHeadCell>
        <TableHeadCell class="td-class">State</TableHeadCell>
        {#if group_id == "all"}
          <TableHeadCell class="td-class">Group</TableHeadCell>
        {/if}
        <TableHeadCell class="td-class">Present</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each teams.filter((t) => !filter_ready || (t.team.presence_state == "present" && t.org_state == "Verified")) as team, index}
          <TableBodyRow
            class={"tr-class " +
              (team.org_state == "Verified" &&
              team.team.presence_state == "present"
                ? "bg-green-300 hover:bg-green-200"
                : team.team.presence_state == "absent"
                  ? "bg-slate-300 hover:bg-slate-200 line-through"
                  : "bg-red-400 hover:bg-red-300")}
          >
            <TableBodyCell class="td-class"
              >{index + 1}

              {#if team.team.changed_since_export}
                <BadgeCheckOutline />
              {:else}
                <BadgeCheckSolid />
              {/if}
            </TableBodyCell>
            <TableBodyCell class="td-class">
              <span class="font-bold">
                {team.team.name.substring(0, 32)}</span
              ><span class="text-red-900">
                {team.team.name.substring(32)}</span
              ></TableBodyCell
            >
            <TableBodyCell class="td-class">
              <div>
                <a href="/org/{team.org.id}/{event_id}">{team.org.name}</a>
                <sup class={team.org.slug.length > 3 ? "text-red-600" : ""}
                  >{team.org.slug}</sup
                >
                <sub>{group_slugs.get(team.team.group_id || "")}</sub>
              </div>
              <div class="text-[10px] overflow-clip">
                {team.team.contact_name}
                (<a href="tel:{team.team.contact_phone}"
                  >{team.team.contact_phone}</a
                >)
              </div>
            </TableBodyCell>
            <TableBodyCell class="td-class">
              <a href="/admin/event/{event_id}">{team.org_state}</a>
            </TableBodyCell>

            {#if group_id == "all"}
              <TableBodyCell class="td-class">
                {group_names.get(team.team.group_id || "") || "??"}
              </TableBodyCell>
            {/if}
            <TableBodyCell class="td-class">
              {#if !confirm_changes}
                <Checkbox
                  disabled={team.org_state != "Verified" ||
                    (team.team.presence_state != "present" &&
                      team.team.presence_state != "")}
                  checked={team.team.presence_state == "present"}
                  on:change={() => {
                    set_team_presence_state(
                      team,
                      team.team.presence_state == "present" ? "" : "present"
                    );
                  }}
                ></Checkbox>
              {:else}
                <ButtonGroup>
                  <Button
                    disabled={team.org_state != "Verified" ||
                      team.team.presence_state == "present"}
                    on:click={() =>
                      set_team_presence_state_verify(team, "present")}
                  >
                    bestätigt
                  </Button>
                  <Button
                    disabled={team.org_state != "Verified" ||
                      team.team.presence_state == ""}
                    on:click={() => set_team_presence_state_verify(team, "")}
                  >
                    unbestätigt
                  </Button>
                  <Button
                    disabled={team.org_state != "Verified" ||
                      team.team.presence_state == "absent"}
                    on:click={() =>
                      set_team_presence_state_verify(team, "absent")}
                  >
                    abmelden
                  </Button>
                </ButtonGroup>
              {/if}
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </div>
  <Button
    class="m-10"
    on:click={() => {
      also_clear_changes_modal = true;
    }}>Download SwissChess Mannschaftsliste...</Button
  >
  <Label>Urkundendruck - Schulename:</Label>
  <Textarea value={slug_org_decode}></Textarea>
  <Label>Urkundendruck - Altersgruppe (Achtung, Attributfeld!):</Label>
  <Textarea value={slug_group_decode}></Textarea>

  <Modal bind:open={verify_change} size="xs" autoclose>
    <div class="text-center">
      <QuestionCircleSolid
        class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
      />
      <div class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
        <div>{verify_team?.org.name}</div>
        <div>{group_names.get(verify_team?.team.group_id || "")}</div>
        <div>{verify_team?.team.name}</div>
        <div>
          {verify_team_new_state == "present"
            ? "bestätigen?"
            : verify_team_new_state == "absent"
              ? "abmelden?"
              : verify_team_new_state == ""
                ? "ent-bestätigen?"
                : ""}
        </div>
      </div>
      <Button
        color="red"
        class="me-2"
        on:click={() => {
          if (verify_team) {
            set_team_presence_state(verify_team, verify_team_new_state || "");
          }
        }}>Yes</Button
      >
      <Button color="alternative">No (Abort)</Button>
    </div>
  </Modal>

  <Modal bind:open={also_clear_changes_modal} size="xs" autoclose>
    <div class="text-center">
      <QuestionCircleSolid
        class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
      />
      <div class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
        <div>Swiss-Chess Export</div>
        <div>
          Sollen die angezeigten Teams nach dem Export als exportiert markiert
          werden?
        </div>
      </div>
      <Button
        color="red"
        class="me-2"
        on:click={() => {
          also_clear_changes_modal = false;

          download_swisschess(true);
        }}>Yes</Button
      >
      <Button
        color="alternative"
        on:click={() => {
          also_clear_changes_modal = false;

          download_swisschess(false);
        }}>No</Button
      >
    </div>
  </Modal>
</div>

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
