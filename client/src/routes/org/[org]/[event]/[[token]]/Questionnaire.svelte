<script lang="ts">
  let props = $props();
  import { onMount } from "svelte";

  import {
    Button,
    ButtonGroup,
    Checkbox,
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    Textarea,
    type SelectOptionType,
  } from "flowbite-svelte";
  import {
    getQuestionnaireAnswerForOrgEvent,
    getQuestionnaireForOrgEvent,
    updateQuestionnaireAnswer,
    type Questionnaire,
    type QuestionnaireAnswer,
  } from "../../../../../api/api";
  import { ExclamationCircleOutline } from "flowbite-svelte-icons";

  type QuestionnaireAnswered = Questionnaire & {
    answer?: string;
    items: SelectOptionType<string>[];
  };

  let questionnaire: QuestionnaireAnswered[] = $state([]);
  let changed: string[] = $state([]);
  let responses = new Map<string, QuestionnaireAnswer>();

  onMount(async () => {
    const resp_answer = await getQuestionnaireAnswerForOrgEvent({
      org: props.org_id,
      event: props.event_id,
    }).result;

    if (resp_answer.ok) {
      responses = new Map(
        resp_answer.data.map((answer) => [answer.question_id, answer])
      );
    }

    const resp = await getQuestionnaireForOrgEvent({
      event: props.event_id,
      org: props.org_id,
    }).result;
    if (resp.ok) {
      questionnaire = resp.data.map((question) => {
        if (!responses.has(question.id)) {
          responses.set(question.id, {
            event_id: props.event_id,
            org_id: props.org_id,
            question_id: question.id,
            question_answer: "",
          });
        }
        return {
          ...question,
          answer: responses.get(question.id)?.question_answer,
          checkbox: [],
          items: question.question_data.split("\n").map((option, index) => ({
            name: option,
            value: index.toString(),
          })),
        };
      });
    }
  });

  function change(id: string) {
    if (!changed.includes(id)) {
      changed.push(id);
    }
  }

  async function save() {
    for (const q of questionnaire) {
      let x = responses.get(q.id);
      if (x) {
        x.question_answer = q.answer || "";
      }
    }
    for (let c of changed) {
      const resp = responses.get(c);
      if (resp) {
        if ((await updateQuestionnaireAnswer({ ...resp }).result).ok) {
          changed = changed.filter((question) => question != resp.question_id);
        }
      }
    }
  }
</script>

<div class="mb-6">
  <Table>
    <TableBody>
      {#each questionnaire as question}
        <TableBodyRow>
          <TableBodyCell>
            {question.question_text}
          </TableBodyCell>
          <TableBodyCell>
            {#if question.question_type == "Freeform"}
              <Textarea
                bind:value={question.answer}
                on:input={() => change(question.id)}
              />
            {:else if question.question_type == "Checkbox"}
              <Checkbox
                bind:value={question.answer}
                on:change={() => change(question.id)}
                >{question.question_data}</Checkbox
              >
            {:else if question.question_type == "MultipleChoice"}
              <Select
                items={question.items}
                bind:value={question.answer}
                on:change={() => change(question.id)}
              />
            {/if}
          </TableBodyCell>
        </TableBodyRow>
      {/each}
      <TableBodyRow>
        <TableBodyCell>
          <Button
            color="green"
            disabled={changed.length == 0}
            onclick={() => {
              save();
            }}
            >{#if changed.length == 0}Gespeichert.{:else}Speichern...{/if}</Button
          ></TableBodyCell
        >
        <TableBodyCell></TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>
</div>
