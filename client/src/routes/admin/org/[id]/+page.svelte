<script lang="ts">
  import { onMount } from "svelte";
  import {
    getEvent,
    getOrg,
    getOrgEvents,
    type Org,
    type OrgEvent,
  } from "../../../../api/api";
  import { page } from "$app/state";
  import {
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { goto } from "$app/navigation";
  import FetchErrors from "../../../FetchErrors.svelte";

  let org: Org | null = $state(null);
  let org_events: OrgEvent[] = $state([]);
  let org_id = page.params.id;
  let fetch_error: FetchErrors;

  onMount(async () => {
    const request = getOrg({ org: org_id });
    const resp = await request.result;
    if (resp.ok) {
      org = resp.data;
    } else {
      fetch_error.check(resp);
    }
    const events_request = getOrgEvents({ org: org_id });
    const events_resp = await events_request.result;
    if (events_resp.ok) {
      org_events = events_resp.data;
    } else {
      fetch_error.check(events_resp);
    }
  });
</script>

<main>
  <FetchErrors bind:this={fetch_error} />
  <h1>
    {org?.name}
  </h1>

  Currently enlisted for:
  <Table>
    <TableHead>
      <TableHeadCell>Event</TableHeadCell>
      <TableHeadCell>Status</TableHeadCell>
      <TableHeadCell>Details</TableHeadCell>
    </TableHead>
    <TableBody>
      {#each org_events as event}
        <TableBodyRow>
          <TableBodyCell>{event.event.name}</TableBodyCell>
          <TableBodyCell>{event.state}</TableBodyCell>
          <TableBodyCell>
            <Button href="/org/{org?.id}/{event.event.id}">Link</Button>
          </TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
</main>
