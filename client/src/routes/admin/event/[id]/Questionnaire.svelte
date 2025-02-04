<script lang="ts">
  let props = $props();
  import { onMount } from "svelte";

  import {
    Button,
    ButtonGroup,
    Input,
    Modal,
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Textarea,
    type SelectOptionType,
  } from "flowbite-svelte";
  import {
    createQuestionnaire,
    deleteQuestionnaire,
    getQuestionnaireForEvent,
    updateQuestionnaire,
    type Questionnaire,
  } from "../../../../api/api";
  import { ExclamationCircleOutline } from "flowbite-svelte-icons";

  let questionnaire: Questionnaire[] = $state([]);
  let edit_mode = $state(false);
  let changed: string[] = $state([]);
  let verify_delete_question = $state(false);
  let question_to_delete: string | null = $state(null);

  const states: SelectOptionType<string>[] = [
    { name: "Freeform (Text)", value: "Freeform" },
    { name: "Checkbox", value: "Checkbox" },
    { name: "Multiple Choice", value: "MultipleChoice" },
  ];

  onMount(async () => {
    const resp = await getQuestionnaireForEvent({ event: props.event_id })
      .result;
    if (resp.ok) {
      questionnaire = resp.data;
    }
  });

  async function new_question() {
    const res = await createQuestionnaire({
      event: props.event_id,
    }).result;
    if (res.ok) {
      questionnaire.push(res.data);
    }
  }

  function change(id: string) {
    if (!changed.includes(id)) {
      changed.push(id);
    }
  }

  async function update(id: string) {
    let question_to_update = questionnaire.find(
      (questionnaire) => questionnaire.id == id
    );
    if (question_to_update) {
      if (
        !(await updateQuestionnaire({ ...question_to_update, id: id }).result)
          .ok
      ) {
        alert("failed to save");
      } else {
        changed = changed.filter((item) => item != id);
      }
    }
  }

  async function delete_question(id: string | null) {
    if (id) {
      deleteQuestionnaire({ id: id });

      changed = changed.filter((item) => item != id);
      questionnaire = questionnaire.filter(
        (questionnaire) => questionnaire.id != id
      );
      question_to_delete = null;
    }
  }
</script>

<div class="mb-6">
  <h1>Questionnaire</h1>
  <Table>
    <TableHead>
      <TableHeadCell>Text</TableHeadCell>
      <TableHeadCell>Type</TableHeadCell>
      <TableHeadCell>Data</TableHeadCell>
      <TableHeadCell>Edit</TableHeadCell>
    </TableHead>
    <TableBody>
      {#each questionnaire as question}
        <TableBodyRow>
          <TableBodyCell>
            <Textarea
              disabled={!edit_mode}
              bind:value={question.question_text}
              oninput={() => change(question.id)}
            /></TableBodyCell
          >
          <TableBodyCell>
            <Select
              class="mt-2"
              disabled={!edit_mode}
              items={states}
              onchange={() => change(question.id)}
              bind:value={question.question_type}
            />
          </TableBodyCell>
          <TableBodyCell>
            <Textarea
              disabled={!edit_mode}
              bind:value={question.question_data}
              oninput={() => change(question.id)}
            /></TableBodyCell
          >

          <TableBodyCell>
            {#if edit_mode}
              <ButtonGroup>
                <Button
                  on:click={() => update(question.id)}
                  color="green"
                  disabled={!changed.includes(question.id)}>Save</Button
                >

                <Button
                  color="red"
                  on:click={() => {
                    question_to_delete = question.id;
                    verify_delete_question = true;
                  }}
                >
                  Remove</Button
                >
              </ButtonGroup>
            {/if}
          </TableBodyCell>
        </TableBodyRow>
      {/each}
      <TableBodyRow>
        <TableBodyCell
          ><Button disabled={!edit_mode} on:click={() => new_question()}
            >New...</Button
          >
          <Button
            disabled={edit_mode}
            onclick={() => {
              edit_mode = true;
            }}>Edit</Button
          ></TableBodyCell
        >
        <TableBodyCell></TableBodyCell>
      </TableBodyRow>
    </TableBody>
  </Table>

  <Modal bind:open={verify_delete_question} size="xs" autoclose>
    <div class="text-center">
      <ExclamationCircleOutline
        class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
      />
      <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
        Really remove this question?
      </h3>
      <Button
        color="red"
        class="me-2"
        on:click={() => delete_question(question_to_delete)}>Yes</Button
      >
      <Button color="alternative">No (Abort)</Button>
    </div>
  </Modal>
</div>
