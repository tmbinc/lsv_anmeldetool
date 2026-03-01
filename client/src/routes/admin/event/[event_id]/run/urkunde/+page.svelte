<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    getResults,
    type EventOrg,
    type Group,
    type ResultEntry,
    type TimetableRow,
  } from "../../../../../../api/api";
  import { page } from "$app/state";
  import { generate } from "@pdfme/generator";
  import { table, text, barcodes, image } from "@pdfme/schemas";
  import { Button } from "flowbite-svelte";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  let event_id = page.params.event_id || "";

  let start_times: string[] = $state([]);
  let timetable = $state(new Map<string, TimetableRow[]>());
  let groupnames = $state(new Map<string, string>());

  const event_orgs: EventOrg[] = $state([]);
  let groups_results: Group[] = $state([]);

  let results: ResultEntry[] = $state([]);
  let orgs: Set<string> = new Set();
  let evtSource: EventSource | null = null;

  onMount(async () => {
    let results_request = getResults({ event: event_id });

    results_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        results = resp.data.results;
        groups_results = resp.data.groups;
        event_name = resp.data.event_name;
        results.sort((a, b) => (a.rank || 0) - (b.rank || 0));

        for (let result of results) {
          if (result.team) {
            orgs.add(result.team);
          }
        }

        groupnames = new Map(
          groups_results.map((group) => [group.id, group.name]),
        );
      }
    });

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = function (event) {
      console.log(event);
      var dataobj = JSON.parse(event.data);
      console.log(dataobj);
      if (dataobj.kind == "results") {
        results_request.reload();
      }
    };
  });

  onDestroy(() => {
    evtSource?.close();
  });

  type EntryWithName = {
    name: string;
    flags: string;
    groups: string[];
  };

  function roundForGroup(group: Group) {
    return results.find((p) => p.group == group.id)?.round || null;
  }

  async function generatePDF() {
    let template = {
      schemas: [
        [
          {
            type: "text",
            position: { x: 120.13, y: 20 },
            content: "INVOICE",
            width: 69.87,
            height: 22.68,
            rotate: 0,
            alignment: "right",
            verticalAlignment: "middle",
            fontSize: 40,
            lineHeight: 1,
            characterSpacing: 0,
            fontColor: "#000000",
            backgroundColor: "",
            opacity: 1,
            readOnly: true,
            fontName: "",
            name: "head",
          },
        ],
      ],
      basePdf: {
        width: 210,
        height: 297,
        padding: [20, 20, 20, 20],
        staticSchema: [],
      },
      pdfmeVersion: "5.0.0",
    };

    const inputs = [
      {
        mytable: [
          ["Alice", "New York", "Alice is a freelance web designer"],
          ["Bob", "Paris", "Bob is a freelance illustrator"],
          ["Charlie", "London", "Charlie is a freelance photographer"],
        ],
      },
    ];

    const plugins = { Table: table, Text: text };

    try {
      const pdf = await generate({ template, inputs, plugins });
      const blob = new Blob([pdf.buffer], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);
      window.open(url);
    } catch (error) {
      console.error("Error generating PDF:", error);
    }
  }
</script>

<main class="h-screen w-screen overflow-hidden bg-white text-black">
  {#each groups_results.filter((f) => !f.replacement) as group, group_index (group.id)}
    <Button onclick={generatePDF}>{group.name}</Button>
  {/each}
</main>
