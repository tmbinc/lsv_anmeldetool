<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import {
    getEvent,
    getEventOrgs,
    listOrgs,
    setEventOrgState,
    type Event,
    type EventOrg,
    type Org,
  } from "../../../../api/api";
  import {
    Table,
    TableHead,
    TableHeadCell,
    TableBody,
    TableBodyRow,
    TableBodyCell,
    Button,
  } from "flowbite-svelte";

  import Groups from "./Groups.svelte";

  let event: Event | null = $state(null);
  let event_orgs: EventOrg[] = $state([]);
  let other_orgs: Org[] = $state([]);

  const event_id = page.params.id;

  onMount(async () => {
    const request = getEvent({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      event = resp.data;
    }

    const resp_event_orgs = await getEventOrgs({ event: event_id }).result;
    if (resp_event_orgs.ok) {
      event_orgs = resp_event_orgs.data;
    }

    const resp_nonevent_orgs = await listOrgs({}).result;
    if (resp_nonevent_orgs.ok) {
      other_orgs = resp_nonevent_orgs.data;

      other_orgs.sort((eventa, eventb) =>
        eventa.name.localeCompare(eventb.name)
      );

      // Ooops O(n^2)
      other_orgs = other_orgs.filter(
        (other_org) =>
          event_orgs.findIndex(
            (event_orgs) => event_orgs.org.id == other_org.id
          ) == -1
      );
    }
    sort();
  });

  function sort() {
    event_orgs = event_orgs.sort((eventa, eventb) =>
      eventa.org.name.localeCompare(eventb.org.name)
    );
  }

  async function add(org: Org) {
    setEventOrgState({ event: event_id, org: org.id, state: "Created" });
    event_orgs.push({ org: org, state: "Created", teams: [] });
    other_orgs = other_orgs.filter((other_org) => other_org.id != org.id);
    sort();
  }

  async function remove(org: Org) {
    setEventOrgState({ event: event_id, org: org.id, state: "NotEnlisted" });
    event_orgs = event_orgs.filter((event_org) => event_org.org.id != org.id);
    other_orgs.push(org);
    sort();
  }
</script>

<main>
  <h1>{event?.name}</h1>
  <Groups {event_id} />

  <Table>
    <TableHead>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>State</TableHeadCell>
      <TableHeadCell></TableHeadCell>
    </TableHead>
    <TableBody>
      {#if event_orgs != null}
        {#each event_orgs as event_org}
          <TableBodyRow>
            <TableBodyCell
              ><a href="/admin/org/{event_org.org.id}">{event_org.org.name}</a
              ></TableBodyCell
            >
            <TableBodyCell>{event_org.state}</TableBodyCell>
            <TableBodyCell
              ><Button onclick={() => remove(event_org.org)}>Remove</Button
              ></TableBodyCell
            >
          </TableBodyRow>
        {/each}
      {/if}
    </TableBody>
  </Table>
  <h2>Unassigned:</h2>
  <Table>
    <TableBody>
      {#if other_orgs != null}
        {#each other_orgs as org}
          <TableBodyRow>
            <TableBodyCell>{org.name}</TableBodyCell>
            <TableBodyCell><i>not assigned</i></TableBodyCell>
            <TableBodyCell
              ><Button onclick={() => add(org)}>Add</Button></TableBodyCell
            >
          </TableBodyRow>
        {/each}
      {/if}
    </TableBody>
  </Table>
</main>
