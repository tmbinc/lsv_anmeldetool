<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import {
    getEvent,
    getEventOrgs,
    inviteOrgToEvent,
    listOrgs,
    setEventOrgState,
    updateEvent,
    type Event,
    type EventOrg,
    type EventOrgState,
    type Org,
    type Team,
  } from "../../../../api/api";
  import {
    Table,
    TableHead,
    TableHeadCell,
    TableBody,
    TableBodyRow,
    TableBodyCell,
    Button,
    Label,
    Input,
    A,
    Checkbox,
    Textarea,
    Select,
    type SelectOptionType,
    ButtonGroup,
    Modal,
  } from "flowbite-svelte";

  import Groups from "./Groups.svelte";
  import Questionnaire from "./Questionnaire.svelte";
  import FetchErrors from "../../../FetchErrors.svelte";
  import { ExclamationCircleOutline } from "flowbite-svelte-icons";

  let event: Event | null = $state(null);
  let event_changed = $state(false);
  let event_orgs: EventOrg[] = $state([]);
  let other_orgs: Org[] = $state([]);
  let fetch_errors: FetchErrors;

  const states: SelectOptionType<string>[] = [
    { name: "Not enlisted", value: "NotEnlisted", disabled: true },
    { name: "Self-registered", value: "Registered" },
    { name: "Invited", value: "Invited" },
    { name: "Updated", value: "Updated" },
    { name: "Submitted", value: "Submitted" },
    { name: "Verified", value: "Verified" },
  ];

  const event_id = page.params.event_id;

  onMount(async () => {
    const request = getEvent({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      event = resp.data;
    } else {
      fetch_errors.check(resp);
    }

    const resp_event_orgs = await getEventOrgs({ event: event_id }).result;
    if (resp_event_orgs.ok) {
      event_orgs = resp_event_orgs.data;
    } else {
      fetch_errors.check(resp_event_orgs);
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
    } else {
      fetch_errors.check(resp_nonevent_orgs);
    }
    sort();
  });

  function sort() {
    event_orgs = event_orgs.sort((eventa, eventb) =>
      eventa.org.name.localeCompare(eventb.org.name)
    );
  }

  async function add(org: Org) {
    setEventOrgState({ event: event_id, org: org.id, state: "Registered" });
    event_orgs.push({ org: org, state: "Registered", teams: [] });
    other_orgs = other_orgs.filter((other_org) => other_org.id != org.id);
    sort();
  }

  async function remove(org: Org) {
    setEventOrgState({ event: event_id, org: org.id, state: "NotEnlisted" });
    event_orgs = event_orgs.filter((event_org) => event_org.org.id != org.id);
    other_orgs.push(org);
    sort();
  }

  async function update_event() {
    if (event) {
      let result = await updateEvent({ ...event, event: event.id }).result;
      if (result.ok) {
        event_changed = false;
      } else {
        alert("warning - failed to save: " + result.data);
      }
    }
  }

  async function update_org_state(event_org: EventOrg) {
    setEventOrgState({
      event: event_id,
      org: event_org.org.id,
      state: event_org.state,
    });
  }

  async function set_org_state(event_org: EventOrg, new_state: EventOrgState) {
    event_org.state = new_state;
    setEventOrgState({
      event: event_id,
      org: event_org.org.id,
      state: event_org.state,
    });
  }
  async function invite_org(event_org: EventOrg) {
    let result = await inviteOrgToEvent({
      org: event_org.org.id,
      event: event_id,
    }).result;
    if (result.ok) {
      event_org.state = "Invited";
    } else {
      alert("failed to invite org: " + event_org.org.name);
    }
  }
  async function invite_all() {
    event_orgs.forEach((org) => {
      if (org.state == "Registered") {
        invite_org(org);
      }
    });
  }
</script>

<main class="px-2 md:px-10 pt-6 flex flex-col gap-4">
  <FetchErrors bind:this={fetch_errors} />
  <div>
    <Button color="yellow" href="/admin/event/{event_id}/run"
      >Turniertag...</Button
    >
  </div>
  {#if event}
    <div class="grid gap-6 mb-6 md:grid-cols-2">
      <div>
        <Label for="event_name">Event Name</Label>
        <Input
          id="event_name"
          type="text"
          on:input={() => (event_changed = true)}
          bind:value={event.name}
          placeholder="Event Name"
          required
        />
      </div>
      <div>
        <Label for="event_begin"
          >Start Time -- FIXME: format _must be_ <pre>2022-01-19T18:15:36.283</pre></Label
        >
        <Input
          id="event_begin"
          type="text"
          on:input={() => (event_changed = true)}
          bind:value={event.begin}
          placeholder="Event Start Time"
          required
        />
      </div>
      <div>
        <Label for="event_public_reg_until">Registration End Time</Label>
        <Input
          id="event_public_reg_until"
          type="text"
          on:input={() => (event_changed = true)}
          bind:value={event.public_reg_until}
          placeholder="Registration End Time"
          required
        />
      </div>
      <div>
        <Label for="event_public">Public (visible on home page)</Label>
        <Checkbox
          id="event_public"
          bind:checked={event.public}
          on:change={() => (event_changed = true)}
        ></Checkbox>
      </div>
      <div>
        <Label for="event_self_registration_allowed"
          >Self-registration allowed</Label
        >
        <Checkbox
          id="event_self_registration_allowed"
          bind:checked={event.self_registration_allowed}
          on:change={() => (event_changed = true)}
        ></Checkbox>
      </div>
      <div>
        <Label for="event_set_present_ok">Set presence allowed</Label>
        <Checkbox
          id="event_set_present_ok"
          bind:checked={event.allow_set_present}
          on:change={() => (event_changed = true)}
        ></Checkbox>
      </div>
      <div>
        <Label for="event_set_allow_user_changes">Allow User Changes</Label>
        <Checkbox
          id="event_set_allow_user_changes"
          bind:checked={event.allow_user_changes}
          on:change={() => (event_changed = true)}
        ></Checkbox>
      </div>
      <div>
        <Label for="event_set_results_are_public"
          >Results are published? (Otherwise, results are restricted to admin
          users e.g. during ceremony.)</Label
        >
        <Checkbox
          id="event_set_results_are_public"
          bind:checked={event.results_are_public}
          on:change={() => (event_changed = true)}
        ></Checkbox>
      </div>
    </div>
    <div class="mb-6">
      <div>
        <Label for="event_description">Event Description</Label>
        <Textarea
          id="event_description"
          rows={10}
          on:input={() => (event_changed = true)}
          bind:value={event.description}
          placeholder="Event Description"
          required
        />
      </div>
      <Button disabled={!event_changed} on:click={update_event}>Save</Button>
    </div>

    <Button href="/event/{event_id}/">Public Self-Registration Link</Button>

    <Groups {event_id} />
    <Questionnaire {event_id} />

    <div class="table-div-class">
      <Table class="text-sm" shadow>
        <TableHead class="th-class">
          <TableHeadCell>Name</TableHeadCell>
          <TableHeadCell>State</TableHeadCell>
          <TableHeadCell>Edit</TableHeadCell>
        </TableHead>
        <TableBody>
          {#if event_orgs != null}
            {#each event_orgs as event_org}
              <TableBodyRow
                class={"tr-class " +
                  {
                    NotEnlisted: "bg-red-300",
                    Registered: "bg-red-300",
                    Invited: "bg-yellow-300",
                    Updated: "bg-green-300",
                    Submitted: "bg-blue-300",
                    Verified: "bg-purple-300",
                  }[event_org.state] || "bg-red-300"}
              >
                <TableBodyCell class="td-class"
                  ><a href="/admin/org/{event_org.org.id}"
                    >{event_org.org.name}</a
                  ></TableBodyCell
                >
                <TableBodyCell class="td-class">
                  <Select
                    class="mt-2"
                    items={states}
                    onchange={() => update_org_state(event_org)}
                    bind:value={event_org.state}
                  />
                </TableBodyCell>
                <TableBodyCell class="td-class">
                  <ButtonGroup>
                    {#if event_org.state == "Registered"}
                      <Button
                        color="green"
                        on:click={() => invite_org(event_org)}>Invite</Button
                      >
                    {:else if event_org.state == "Invited"}
                      <Button color="green" disabled={true}>Invite</Button>
                    {:else if event_org.state == "Updated"}
                      <Button
                        color="green"
                        on:click={() => set_org_state(event_org, "Submitted")}
                        >Submit</Button
                      >
                    {:else if event_org.state == "Submitted"}
                      <Button
                        color="green"
                        on:click={() => set_org_state(event_org, "Verified")}
                        >Verify</Button
                      >
                    {:else if event_org.state == "Verified"}
                      <Button color="green" disabled={true}>Verify</Button>
                    {/if}

                    <Button color="red" onclick={() => remove(event_org.org)}
                      >Remove</Button
                    >
                    <Button
                      color="blue"
                      href="/org/{event_org.org.id}/{event_id}">Teams</Button
                    ></ButtonGroup
                  >
                </TableBodyCell>
              </TableBodyRow>
            {/each}
          {/if}
        </TableBody>
      </Table>
    </div>
    <h2>Unassigned:</h2>
    <div class="table-div-class">
      <Table class="text-sm">
        <TableBody>
          {#if other_orgs != null}
            {#each other_orgs as org}
              <TableBodyRow class="tr-class">
                <TableBodyCell class="td-class">{org.name}</TableBodyCell>
                <TableBodyCell class="td-class"
                  ><i>not assigned</i></TableBodyCell
                >
                <TableBodyCell class="td-class"
                  ><Button onclick={() => add(org)}>Add</Button></TableBodyCell
                >
              </TableBodyRow>
            {/each}
          {/if}
        </TableBody>
      </Table>
    </div>

    <Button on:click={() => invite_all()}
      >Invite all Self-Registered orgs...</Button
    >
  {/if}
</main>

<style lang="postcss">
  :global(.td-class) {
    @apply px-4 py-3;
  }
  :global(.tr-class) {
    @apply flex flex-col mb-4 sm:table-row;
  }
  :global(.th-class) {
    @apply flex flex-col mb-4 sm:table-header-group;
  }
  :global(.table-div-class) {
    @apply flex sm:justify-normal justify-center ml-4 sm:ml-0;
  }
</style>
