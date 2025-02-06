<script lang="ts">
  import { onMount } from "svelte";

  import {
    Button,
    ButtonGroup,
    Checkbox,
    Input,
    Radio,
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

  let { active_changes = $bindable(false), event_id, org_id } = $props();

  type QuestionnaireAnswered = Questionnaire & {
    answer?: string;
    items: SelectOptionType<string>[];
  };

  let questionnaire: QuestionnaireAnswered[] = $state([]);
  let changed: string[] = $state([]);
  let responses = new Map<string, QuestionnaireAnswer>();

  onMount(async () => {
    const resp_answer = await getQuestionnaireAnswerForOrgEvent({
      org: org_id,
      event: event_id,
    }).result;

    if (resp_answer.ok) {
      responses = new Map(
        resp_answer.data.map((answer) => [answer.question_id, answer])
      );
    }

    const resp = await getQuestionnaireForOrgEvent({
      event: event_id,
      org: org_id,
    }).result;
    if (resp.ok) {
      questionnaire = resp.data
        .sort((q0, q1) => q0.sort - q1.sort)
        .map((question) => {
          if (!responses.has(question.id)) {
            responses.set(question.id, {
              event_id: event_id,
              org_id: org_id,
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
    active_changes = changed.length > 0;
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
    active_changes = changed.length > 0;
  }
</script>

<div>
  {#each questionnaire as question}
    <div>
      {@html question.question_text}
    </div>
    {#if question.question_type == "Freeform"}
      <Textarea
        class="w-xl"
        rows={5}
        bind:value={question.answer}
        on:input={() => change(question.id)}
      />
    {:else if question.question_type == "Checkbox"}
      <Checkbox
        bind:value={question.answer}
        on:change={() => change(question.id)}>{question.question_data}</Checkbox
      >
    {:else if question.question_type == "MultipleChoice"}
      <Select
        items={question.items}
        bind:value={question.answer}
        on:change={() => change(question.id)}
      />
    {:else if question.question_type == "Radio"}
      {#each question.items as item}
        <Radio
          value={item.value}
          bind:group={question.answer}
          on:change={() => change(question.id)}>{item.name}</Radio
        >
      {/each}
    {:else if question.question_type == "SingleInput"}
      <div class="inline-block">
        {question.question_data.split("__")[0]}
        <Input
          class="inline-block w-16"
          bind:value={question.answer}
          on:change={() => change(question.id)}
        />
        {question.question_data.split("__")[1]}
      </div>
    {/if}
  {/each}
  {#if questionnaire.length > 0}
    <Button
      color="green"
      disabled={changed.length == 0}
      onclick={() => {
        save();
      }}
      >{#if changed.length == 0}Gespeichert.{:else}Speichern...{/if}</Button
    >
  {/if}
</div>
