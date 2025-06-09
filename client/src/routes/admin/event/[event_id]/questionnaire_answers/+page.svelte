<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import {
    getQuestionnaireAnswersForEvent,
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
  let data: EventQuestionnaireAnswers | undefined = $state(undefined);
  let email_addresses: Set<string> | undefined = $state(undefined);

  onMount(async () => {
    const request = getQuestionnaireAnswersForEvent({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      data = resp.data;
      data.questions.sort((a, b) => a.sort - b.sort);
      email_addresses = new Set(
        data.orgs.map(
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

  {#if data}
    <Table hoverable={true} striped={true} shadow>
      <TableHead>
        <TableHeadCell></TableHeadCell>
        {#each data.questions as question}
          <TableHeadCell>
            {question.question_data}
          </TableHeadCell>
        {/each}
      </TableHead>
      <TableBody>
        {#each data.orgs as org}
          <TableBodyRow
            ><TableBodyCell>{org.org.name}</TableBodyCell>

            {#each data.questions as question}
              <TableBodyCell>
                {#each org.answers as answer}
                  {#if question.id == answer.question_id}
                    {answer.question_answer}
                  {/if}
                {/each}
              </TableBodyCell>
            {/each}
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if}
  <pre>
    {#if email_addresses}
      {#each email_addresses as email}{email}
      {/each}
    {/if}
  </pre>
</main>
