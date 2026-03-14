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
  } from "../../../../api/api";
  import { Button, Select, type SelectOptionType } from "flowbite-svelte";
  import { ArrowRightOutline } from "flowbite-svelte-icons";

  import Groups from "./Groups.svelte";
  import Questionnaire from "./Questionnaire.svelte";
  import FetchErrors from "../../../FetchErrors.svelte";

  let event: Event | null = $state(null);
  let event_changed = $state(false);
  let event_orgs: EventOrg[] = $state([]);
  let other_orgs: Org[] = $state([]);
  let fetch_errors: FetchErrors;

  const states: SelectOptionType<string>[] = [
    { name: "Not enlisted",    value: "NotEnlisted",  disabled: true },
    { name: "Self-registered", value: "Registered" },
    { name: "Invited",         value: "Invited" },
    { name: "Updated",         value: "Updated" },
    { name: "Submitted",       value: "Submitted" },
    { name: "Verified",        value: "Verified" },
  ];

  const stateBadge: Record<string, string> = {
    NotEnlisted: "bg-gray-100 text-gray-600",
    Registered:  "bg-red-100 text-red-700",
    Invited:     "bg-yellow-100 text-yellow-700",
    Updated:     "bg-green-100 text-green-700",
    Submitted:   "bg-blue-100 text-blue-700",
    Verified:    "bg-purple-100 text-purple-700",
  };

  const stateRow: Record<string, string> = {
    NotEnlisted: "bg-gray-50",
    Registered:  "bg-red-50",
    Invited:     "bg-yellow-50",
    Updated:     "bg-green-50",
    Submitted:   "bg-blue-50",
    Verified:    "bg-purple-50",
  };

  const event_id = page.params.event_id || "";

  onMount(async () => {
    const resp = await getEvent({ event: event_id }).result;
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
      other_orgs = resp_nonevent_orgs.data.sort((a, b) => a.name.localeCompare(b.name));
      other_orgs = other_orgs.filter(
        (o) => event_orgs.findIndex((eo) => eo.org.id === o.id) === -1,
      );
    } else {
      fetch_errors.check(resp_nonevent_orgs);
    }
    sort();
  });

  function sort() {
    event_orgs = event_orgs.sort((a, b) => a.org.name.localeCompare(b.org.name));
  }

  async function add(org: Org) {
    setEventOrgState({ event: event_id, org: org.id, state: "Registered" });
    event_orgs.push({ org, state: "Registered", teams: [] });
    other_orgs = other_orgs.filter((o) => o.id !== org.id);
    sort();
  }

  async function remove(org: Org) {
    setEventOrgState({ event: event_id, org: org.id, state: "NotEnlisted" });
    event_orgs = event_orgs.filter((eo) => eo.org.id !== org.id);
    other_orgs.push(org);
    sort();
  }

  async function update_event() {
    if (event) {
      const result = await updateEvent({ ...event, event: event.id }).result;
      if (result.ok) {
        event_changed = false;
      } else {
        alert("Speichern fehlgeschlagen: " + result.data);
      }
    }
  }

  async function update_org_state(event_org: EventOrg) {
    setEventOrgState({ event: event_id, org: event_org.org.id, state: event_org.state });
  }

  async function set_org_state(event_org: EventOrg, new_state: EventOrgState) {
    event_org.state = new_state;
    setEventOrgState({ event: event_id, org: event_org.org.id, state: event_org.state });
  }

  async function invite_org(event_org: EventOrg) {
    const result = await inviteOrgToEvent({ org: event_org.org.id, event: event_id }).result;
    if (result.ok) {
      event_org.state = "Invited";
    } else {
      alert("Einladung fehlgeschlagen: " + event_org.org.name);
    }
  }

  async function invite_all() {
    event_orgs.forEach((org) => {
      if (org.state === "Registered") invite_org(org);
    });
  }
</script>

<FetchErrors bind:this={fetch_errors} />

<div class="mx-auto max-w-5xl px-4 py-6 md:px-8">

  {#if event}
    <!-- Page header -->
    <div class="mb-6 flex items-center justify-between">
      <h1 class="text-2xl font-bold text-gray-900">{event.name}</h1>
      <div class="flex gap-2">
        <Button size="sm" color="alternative" href="/event/{event_id}/">Öffentliche Anmeldung</Button>
        <Button size="sm" href="/admin/event/{event_id}/run">
          Turniertag <ArrowRightOutline class="ml-1 h-4 w-4" />
        </Button>
      </div>
    </div>

    <!-- Event settings -->
    <section class="mb-6 rounded-xl border border-gray-200 bg-white shadow-sm">
      <div class="border-b border-gray-100 px-5 py-3">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-gray-500">Turnier-Einstellungen</h2>
      </div>
      <div class="px-5 py-4">
        <div class="grid gap-4 sm:grid-cols-2">
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700" for="event_name">Name</label>
            <input
              id="event_name"
              type="text"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
              oninput={() => (event_changed = true)}
              bind:value={event.name}
              placeholder="Turniername"
            />
          </div>
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700" for="event_begin">
              Startzeit <span class="font-normal text-gray-400">(z.B. 2024-03-15T09:00:00.000)</span>
            </label>
            <input
              id="event_begin"
              type="text"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
              oninput={() => (event_changed = true)}
              bind:value={event.begin}
              placeholder="2024-03-15T09:00:00.000"
            />
          </div>
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700" for="event_reg_start">
              Anmeldung ab <span class="font-normal text-gray-400">(z.B. 2024-01-01T00:00:00.000)</span>
            </label>
            <input
              id="event_reg_start"
              type="text"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
              oninput={() => (event_changed = true)}
              bind:value={event.registration_start_date}
              placeholder="2024-01-01T00:00:00.000"
            />
          </div>
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700" for="event_reg_until">
              Anmeldung bis
            </label>
            <input
              id="event_reg_until"
              type="text"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
              oninput={() => (event_changed = true)}
              bind:value={event.public_reg_until}
              placeholder="2024-03-01T00:00:00.000"
            />
          </div>
        </div>

        <!-- Toggles -->
        <div class="mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {#each [
            { id: "public",               label: "Öffentlich sichtbar",     key: "public" as keyof Event },
            { id: "reg_active",           label: "Anmeldung aktiv",         key: "registration_active" as keyof Event },
            { id: "self_reg",             label: "Selbstanmeldung erlaubt", key: "self_registration_allowed" as keyof Event },
            { id: "set_present",          label: "Anwesenheit setzbar",     key: "allow_set_present" as keyof Event },
            { id: "user_changes",         label: "Benutzeränderungen",      key: "allow_user_changes" as keyof Event },
            { id: "results_public",       label: "Ergebnisse öffentlich",   key: "results_are_public" as keyof Event },
          ] as toggle}
            <label class="flex cursor-pointer items-center gap-3 rounded-lg border border-gray-200 px-3 py-2 hover:bg-gray-50">
              <input
                type="checkbox"
                class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-400"
                bind:checked={event[toggle.key] as boolean}
                onchange={() => (event_changed = true)}
              />
              <span class="text-sm text-gray-700">{toggle.label}</span>
            </label>
          {/each}
        </div>

        <!-- Description -->
        <div class="mt-4">
          <label class="mb-1 block text-sm font-medium text-gray-700" for="event_description">Beschreibung</label>
          <textarea
            id="event_description"
            rows={6}
            class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
            oninput={() => (event_changed = true)}
            bind:value={event.description}
            placeholder="Turnierbeschreibung"
          ></textarea>
        </div>

        <div class="mt-3 flex justify-end">
          <Button size="sm" disabled={!event_changed} onclick={update_event}>Speichern</Button>
        </div>
      </div>
    </section>

    <!-- Groups -->
    <section class="mb-6 rounded-xl border border-gray-200 bg-white shadow-sm">
      <div class="border-b border-gray-100 px-5 py-3">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-gray-500">Altersgruppen</h2>
      </div>
      <div class="px-5 py-4">
        <Groups {event_id} />
      </div>
    </section>

    <!-- Questionnaire -->
    <section class="mb-6 rounded-xl border border-gray-200 bg-white shadow-sm">
      <div class="border-b border-gray-100 px-5 py-3">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-gray-500">Fragebogen</h2>
      </div>
      <div class="px-5 py-4">
        <Questionnaire {event_id} />
      </div>
    </section>

    <!-- Enrolled orgs -->
    <section class="mb-6 rounded-xl border border-gray-200 bg-white shadow-sm">
      <div class="flex items-center justify-between border-b border-gray-100 px-5 py-3">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-gray-500">Teilnehmende Schulen</h2>
        <Button size="xs" color="alternative" onclick={invite_all}>Alle einladen</Button>
      </div>
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-gray-100 text-left text-xs font-semibold uppercase tracking-wide text-gray-400">
              <th class="px-5 py-3">Schule</th>
              <th class="px-3 py-3">Status</th>
              <th class="px-3 py-3">Status setzen</th>
              <th class="px-3 py-3"></th>
            </tr>
          </thead>
          <tbody>
            {#each event_orgs as event_org}
              <tr class="border-b border-gray-100 {stateRow[event_org.state] ?? 'bg-gray-50'}">
                <td class="px-5 py-2.5">
                  <a href="/admin/org/{event_org.org.id}" class="font-medium text-gray-900 hover:text-blue-600">
                    {event_org.org.name}
                  </a>
                </td>
                <td class="px-3 py-2.5">
                  <span class="rounded-full px-2 py-0.5 text-xs font-medium {stateBadge[event_org.state] ?? 'bg-gray-100 text-gray-600'}">
                    {event_org.state}
                  </span>
                </td>
                <td class="px-3 py-2.5">
                  <select
                    class="rounded border border-gray-300 bg-white px-2 py-1 text-xs focus:outline-none focus:ring-2 focus:ring-blue-400"
                    onchange={() => update_org_state(event_org)}
                    bind:value={event_org.state}
                  >
                    {#each states as s}
                      <option value={s.value} disabled={s.disabled}>{s.name}</option>
                    {/each}
                  </select>
                </td>
                <td class="px-3 py-2.5">
                  <div class="flex items-center gap-1">
                    {#if event_org.state === "Registered"}
                      <Button size="xs" color="green" onclick={() => invite_org(event_org)}>Einladen</Button>
                    {:else if event_org.state === "Updated"}
                      <Button size="xs" color="green" onclick={() => set_org_state(event_org, "Submitted")}>Submit</Button>
                    {:else if event_org.state === "Submitted"}
                      <Button size="xs" color="green" onclick={() => set_org_state(event_org, "Verified")}>Verify</Button>
                    {/if}
                    <Button size="xs" color="alternative" href="/org/{event_org.org.id}/{event_id}">Teams</Button>
                    <Button size="xs" color="red" onclick={() => remove(event_org.org)}>Entfernen</Button>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </section>

    <!-- Unassigned orgs -->
    <section class="mb-6 rounded-xl border border-gray-200 bg-white shadow-sm">
      <div class="border-b border-gray-100 px-5 py-3">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-gray-500">Nicht zugeordnete Schulen</h2>
      </div>
      {#if other_orgs.length === 0}
        <p class="px-5 py-4 text-sm text-gray-400">Keine nicht zugeordneten Schulen.</p>
      {:else}
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <tbody>
              {#each other_orgs as org}
                <tr class="border-b border-gray-50 hover:bg-gray-50">
                  <td class="px-5 py-2.5 font-medium text-gray-700">{org.name}</td>
                  <td class="px-3 py-2.5 text-gray-400">nicht zugeordnet</td>
                  <td class="px-3 py-2.5">
                    <Button size="xs" onclick={() => add(org)}>Hinzufügen</Button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </section>

  {/if}
</div>
