<script lang="ts">
  import {
    Button,
    ButtonGroup,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";

  import {
    getGroupsForEvent,
    getTimetable,
    type Group,
    type TimetableRow,
  } from "../../../../../../api/api";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import FetchErrors from "../../../../../FetchErrors.svelte";

  let event_id = page.params.event_id;
  let fetch_errors: FetchErrors;
  type TimetableColumn = {
    group: Group;
    rows: TimetableRow[];
  };
  let timetable = $state(new Map<String, TimetableColumn>());
  let group_ids: String[] = $state([]);
  let max_round = $state(0);
  let timetable_names: String[] = $state([]);

  onMount(async () => {
    const resp_timetable = await getTimetable({
      event: event_id,
    }).result;
    if (resp_timetable.ok) {
      const new_timetable = new Map<String, TimetableColumn>(
        resp_timetable.data.groups.map((group) => [
          group.id,
          { group: group, rows: [] },
        ])
      );

      for (const row of resp_timetable.data.rows) {
        let group = new_timetable.get(row.group);
        if (group) {
          group.rows.push(row);
        }
      }

      let new_max_rounds = 0;
      for (const group of new_timetable.values()) {
        if (group.group.num_rounds > new_max_rounds) {
          new_max_rounds = group.group.num_rounds;
        }
        group.rows.sort((item) => item.row_index);
      }
      max_round = new_max_rounds;

      let new_timetable_names = [];
      new_timetable_names.push("Anmeldung");
      new_timetable_names.push("Eröffnung");
      for (let j = 1; j <= max_round; ++j) {
        new_timetable_names.push("Runde " + j + " Paarung");
        new_timetable_names.push("Runde " + j + " Start");
      }
      new_timetable_names.push("Siegerehrung");
      new_timetable_names.push("Ende");
      timetable_names = new_timetable_names;

      group_ids = Array.from(new_timetable.values())
        .toSorted((a, b) => a.group.name.localeCompare(b.group.name))
        .map((entry) => entry.group.id);

      timetable = new_timetable;
    } else {
      fetch_errors.check(resp_timetable);
    }
  });

  function setActive(group_id: String, active_index: number) {
    let group = timetable.get(group_id);
    if (group) {
      for (const row of group.rows) {
        if (active_index == row.row_index) {
          row.state = "active";
        } else {
          row.state = "";
        }
      }
    }
  }
</script>

<FetchErrors bind:this={fetch_errors} />

For each group, allow to set expected time for ("Registration", "Round #n
paired", "Round #n start", "Ceremony")

<Table>
  <TableHead>
    <TableHeadCell></TableHeadCell>
    {#each group_ids as group_id}
      <TableHeadCell>{timetable.get(group_id)?.group.slug}</TableHeadCell>
    {/each}
  </TableHead>
  <TableBody>
    {#each timetable_names as timetable_name}
      <TableBodyRow>
        <TableBodyCell>{timetable_name}</TableBodyCell>
        {#each group_ids as group_id}
          <TableBodyCell>
            <input
              class={timetable
                .get(group_id)
                ?.rows.find((n) => n.name == timetable_name)?.state == "active"
                ? "bg-green-500"
                : "bg-white"}
              value={timetable
                .get(group_id)
                ?.rows.find((n) => n.name == timetable_name)?.expected_time ||
                "9:00 Uhr"}
            />

            <ButtonGroup>
              <Button color="green">now</Button>
              <Button color="blue">&gt;&gt;</Button>
            </ButtonGroup>
          </TableBodyCell>
        {/each}
      </TableBodyRow>
    {/each}
  </TableBody>
</Table>
<Button>Update</Button>
