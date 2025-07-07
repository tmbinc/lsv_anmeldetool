<script lang="ts">
  import { Button, ButtonGroup } from "flowbite-svelte";
  import {
    getGroupsForEvent,
    getTimetable,
    setTimetable,
    type Group,
    type TimetableEntry,
    type TimetableRow,
  } from "../../../../../../api/api";
  import { DateInput } from "date-picker-svelte";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import FetchErrors from "../../../../../FetchErrors.svelte";
  import TimePicker from "date-picker-svelte/TimePicker.svelte";
  import { BellActiveAltSolid } from "flowbite-svelte-icons";

  let event_id = page.params.event_id;
  let fetch_errors: FetchErrors;
  type TimetableColumn = {
    group: Group;
    rows: TimetableRow[];
  };
  let timetable: TimetableColumn[] = $state([]);
  let max_round = $state(0);
  let timetable_names: string[] = $state([]);
  let base_date = $state(new Date());

  onMount(async () => {
    const resp_timetable = await getTimetable({
      event: event_id,
    }).result;
    if (resp_timetable.ok) {
      timetable = resp_timetable.data.groups.map((group) => ({
        group: group,
        rows: [],
      }));

      for (const row of resp_timetable.data.rows) {
        let group = timetable.find((f) => f.group.id == row.group);
        if (group) {
          group.rows.push(row);
        }
      }

      let new_max_rounds = 0;
      for (const group of timetable.values()) {
        if (group.group.num_rounds > new_max_rounds) {
          new_max_rounds = group.group.num_rounds;
        }
        group.rows.sort((item) => item.row_index);
      }
      max_round = new_max_rounds;

      timetable.sort((a, b) => a.group.name.localeCompare(b.group.name));
    } else {
      fetch_errors.check(resp_timetable);
    }
  });

  function setActive(group_id: string, active_index: number) {
    let group = timetable.find((group) => group.group.id == group_id);
    if (group) {
      for (const row of group.rows) {
        if (active_index == row.row_index) {
          row.state = "active";
        } else if (active_index + 1 == row.row_index) {
          row.state = "next";
        } else {
          row.state = "";
        }
      }
    }
  }

  function resetToDefault() {
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

    for (const column of timetable.values()) {
      column.rows = new_timetable_names
        .entries()
        .map((i) => ({
          expected_time: base_date.toISOString().slice(0, -1),
          group: column.group.id,
          last_update: "",
          name: i[1],
          row_index: i[0],
          state: "",
          flags: "",
        }))
        .toArray();
    }
  }

  function newTimetableEntry(group_id: string) {
    let group = timetable.find((group) => group.group.id == group_id);
    if (group) {
      group.rows.push({
        expected_time: base_date.toISOString().slice(0, -1),
        group: group_id,
        last_update: "",
        name: "new",
        row_index: group.rows.length,
        state: "",
        flags: "",
      });
    }
  }

  // Swap with next
  function bump_down(group: TimetableColumn, row_index: number) {
    const index = group.rows.findIndex((row) => row.row_index == row_index);
    if (index >= 0 && index + 1 < group.rows.length) {
      let r1 = group.rows[index].row_index;
      let r2 = group.rows[index + 1].row_index;
      group.rows[index].row_index = r2;
      group.rows[index + 1].row_index = r1;
    }
    group.rows.sort((a, b) => a.row_index - b.row_index);
  }

  // Swap with previous
  function bump_up(group: TimetableColumn, row_index: number) {
    const index = group.rows.findIndex((row) => row.row_index == row_index);
    if (index >= 1 && index < group.rows.length) {
      let r1 = group.rows[index - 1].row_index;
      let r2 = group.rows[index].row_index;
      group.rows[index - 1].row_index = r2;
      group.rows[index].row_index = r1;
    }
    group.rows.sort((a, b) => a.row_index - b.row_index);
  }

  async function upload() {
    let res: TimetableEntry[] = [];

    for (const t of timetable) {
      for (const r of t.rows) {
        res.push({
          event: event_id,
          expected_time: r.expected_time,
          group_id: t.group.id,
          last_update: new Date(Date.now()).toISOString().slice(0, -1),
          name: r.name,
          row_index: r.row_index,
          state: r.state,
          flags: r.flags,
        });
      }
    }

    const upload_res = await setTimetable({ event: event_id, timetable: res })
      .result;
    if (!upload_res.ok) {
      alert("upload of timetable failed");
    } else {
      alert("upload of timetable ok");
    }
  }

  function copyToOthers(timetable_group: TimetableColumn) {
    timetable.forEach((group) => {
      if (group != timetable_group) {
        group.rows = timetable_group.rows.map((n) => ({ ...n }));
      }
    });
  }

  function toggleFlags(entry: TimetableRow, flags: string) {
    if (entry.flags.includes(flags)) {
      entry.flags = entry.flags.replaceAll(flags, "");
    } else {
      entry.flags += flags;
    }
  }
</script>

<FetchErrors bind:this={fetch_errors} />
<div class="flex flex-row">
  <div>
    Base Date
    <DateInput bind:value={base_date} />
  </div>
  <Button on:click={() => resetToDefault()}>Reset To Default</Button>
  <Button href="/event/{event_id}/timetable">Public Link</Button>
</div>

<div class="flex flex-row">
  {#each timetable as timetable_group}
    <div class="flex flex-col m-5">
      {timetable_group.group.name}
      <Button on:click={() => copyToOthers(timetable_group)}>Copy...</Button>
      {#each timetable_group.rows as row}
        <div
          class={"shadow-sm flex flex-row " +
            (row.state == "active"
              ? "bg-green-400"
              : row.state == "next"
                ? "bg-orange-400"
                : "")}
        >
          <input bind:value={row.name} />
          <TimePicker
            browseDate={new Date(Date.parse(row.expected_time + "Z"))}
            timePrecision={"minute"}
            setTime={(d) => {
              row.expected_time = d.toISOString().slice(0, -1);
              return d;
            }}
          />
          <ButtonGroup>
            <Button on:click={() => bump_up(timetable_group, row.row_index)}
              >↑</Button
            >
            <Button on:click={() => bump_down(timetable_group, row.row_index)}
              >↓</Button
            >
            <Button
              on:click={() =>
                setActive(timetable_group.group.id, row.row_index)}
              ><BellActiveAltSolid /></Button
            >
            <Button
              checked={row.flags.includes("p")}
              on:click={() => toggleFlags(row, "p")}>Show Pairing Link</Button
            >
            <Button
              checked={row.flags.includes("!")}
              on:click={() => toggleFlags(row, "!")}>Include in Overview</Button
            >
          </ButtonGroup>
        </div>
      {/each}
      <div>
        <Button on:click={() => newTimetableEntry(timetable_group.group.id)}
          >+</Button
        >
      </div>
    </div>
  {/each}
</div>

<Button on:click={() => upload()}>Upload</Button>
