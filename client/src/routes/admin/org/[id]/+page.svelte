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
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";

  let org: Org | null = $state(null);
  let org_events: OrgEvent[] = $state([]);
  let org_id = page.params.id;

  onMount(async () => {
    const request = getOrg({ org: org_id });
    const resp = await request.result;
    if (resp.ok) {
      org = resp.data;
    }
    const events_request = getOrgEvents({ org: org_id });
    const events_resp = await events_request.result;
    if (events_resp.ok) {
      org_events = events_resp.data;
    }
  });
</script>

<main>
  <h1>
    {org?.name}
  </h1>

  Currently enlisted for:
  <Table>
    <TableBody>
      {#each org_events as event}
        <TableBodyRow>
          <TableBodyCell
            >{event.event.name}

            Current teams:

            <Table>
              <TableHead>
                <TableHeadCell>Team Name</TableHeadCell>
                <TableHeadCell>Assigned Group</TableHeadCell>
              </TableHead>
              <TableBody>
                <TableBodyRow>
                  <TableBodyCell>team 1</TableBodyCell>
                  <TableBodyCell>team 1</TableBodyCell>
                </TableBodyRow>
              </TableBody>
            </Table>

            <a href="/org/{org?.id}/{event.event.id}">Link</a>
          </TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
</main>
