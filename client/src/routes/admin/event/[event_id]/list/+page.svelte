<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import {
    getEventOrgs,
    getQuestionnaireAnswersForEvent,
    type EventOrg,
    type EventQuestionnaireAnswers,
  } from "../../../../../api/api";
  import FetchErrors from "../../../../FetchErrors.svelte";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  const event_id = page.params.event_id;

  let fetch_errors: FetchErrors;
  let event_orgs: EventOrg[] = $state([]);
  let email_addresses: Set<string> | undefined = $state(undefined);
  let possible_states = [
    "Registered",
    "Invited",
    "Updated",
    "Submitted",
    "Verified",
  ];

  onMount(async () => {
    const request = getEventOrgs({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      event_orgs = resp.data;
      email_addresses = new Set(
        event_orgs.map(
          (org) => org.org.contact_name + " <" + org.org.contact_email + ">"
        )
      );
    } else {
      fetch_errors.check(resp);
    }
  });
</script>

<main class="px-2 md:px-10 pt-6 flex flex-col gap-4">
  <FetchErrors bind:this={fetch_errors} />

  {#each possible_states as state}
    <div>{state}</div>
    <Table hoverable={true} striped={true} shadow>
      <TableHead>
        <TableHeadCell></TableHeadCell>
        <TableHeadCell></TableHeadCell>
      </TableHead>
      <TableBody>
        {#each event_orgs as org}
          {#if org.state == state}
            <TableBodyRow
              ><TableBodyCell>{org.org.name}</TableBodyCell>
              <TableBodyCell
                >{org.org.contact_name} &lt;{org.org
                  .contact_email}&gt;</TableBodyCell
              >
              <TableBodyCell>
                {org.teams.length} Teams
              </TableBodyCell>
            </TableBodyRow>
          {/if}
        {/each}
      </TableBody>
    </Table>
  {/each}

  <pre>
    {#if email_addresses}
      {#each email_addresses as email}{email}
      {/each}
    {/if}
  </pre>
</main>
