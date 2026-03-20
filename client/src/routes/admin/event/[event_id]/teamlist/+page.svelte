<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import {
    getEventOrgs,
    getGroupsForEvent,
    type EventOrg,
    type Group,
  } from "../../../../../api/api";
  import FetchErrors from "../../../../FetchErrors.svelte";

  const event_id = page.params.event_id || "";
  let fetch_errors: FetchErrors;

  let event_orgs: EventOrg[] = $state([]);
  let groups: Group[] = $state([]);
  let groupnames = $state(new Map<string, string>());
  let loading = $state(true);

  let show_changed = $state(false);
  let show_state   = $state(false);
  let sort_by_group = $state(false);
  let show_delimiters = $state(false);

  onMount(async () => {
    const resp = await getEventOrgs({ event: event_id }).result;
    if (resp.ok) {
      event_orgs = resp.data.filter((e) => e.teams.length > 0);
    } else {
      fetch_errors.check(resp);
    }

    const resp_groups = await getGroupsForEvent({ event: event_id }).result;
    if (resp_groups.ok) {
      groups = resp_groups.data;
      groupnames = new Map(resp_groups.data.map((g) => [g.id, g.name]));
    }

    loading = false;
  });

  // Orgs sorted by name; within each org, teams grouped by group (sorted by
  // group name), teams sorted alphabetically within each group.
  const sortedOrgs = $derived.by(() =>
    [...event_orgs]
      .sort((a, b) => a.org.name.localeCompare(b.org.name))
      .map((event_org) => {
        const byGroup = new Map<string, typeof event_org.teams>();
        for (const team of event_org.teams) {
          const gid = team.group_id ?? "";
          if (!byGroup.has(gid)) byGroup.set(gid, []);
          byGroup.get(gid)!.push(team);
        }
        const groupedTeams = [...byGroup.entries()]
          .map(([gid, teams]) => ({
            groupName: groupnames.get(gid) ?? "–",
            teams: [...teams].sort((a, b) => a.name.localeCompare(b.name)),
          }))
          .sort((a, b) => a.groupName.localeCompare(b.groupName));
        return { event_org, groupedTeams };
      })
  );

  // Groups sorted by name; within each group, orgs sorted by name; within each
  // org, teams sorted alphabetically — filtered to only teams in that group.
  const sortedByGroup = $derived.by(() =>
    [...groups]
      .sort((a, b) => a.name.localeCompare(b.name))
      .flatMap((group) => {
        const orgs = event_orgs
          .map((eo) => ({
            event_org: eo,
            teams: [...eo.teams]
              .filter((t) => t.group_id === group.id)
              .sort((a, b) => a.name.localeCompare(b.name)),
          }))
          .filter(({ teams }) => teams.length > 0)
          .sort((a, b) => a.event_org.org.name.localeCompare(b.event_org.org.name));
        return orgs.length > 0 ? [{ group, orgs }] : [];
      })
  );

  // Unique "Name <email>" strings for every org that has teams.
  const emailList = $derived.by(() => {
    const seen = new Set<string>();
    for (const { event_org } of sortedOrgs) {
      const { contact_name, contact_email } = event_org.org;
      if (contact_email) {
        const entry = contact_name ? `${contact_name} <${contact_email}>` : contact_email;
        seen.add(entry);
      }
    }
    return [...seen].sort();
  });

  function presenceIcon(presence: string | null | undefined): { symbol: string; class: string } {
    if (presence === "present") return { symbol: "✓", class: "rounded px-1 py-0.5 text-[10px] font-bold leading-none bg-green-500 text-white" };
    if (presence === "absent")  return { symbol: "✗", class: "rounded px-1 py-0.5 text-[10px] font-bold leading-none bg-red-500 text-white" };
    return { symbol: "?", class: "rounded px-1 py-0.5 text-[10px] font-bold leading-none bg-amber-400 text-amber-900" };
  }
</script>

<FetchErrors bind:this={fetch_errors} />

<main class="mx-auto max-w-3xl px-6 py-8">

  <!-- Options -->
  <div class="print:hidden mb-8 flex flex-wrap gap-6 rounded-xl border border-gray-200 bg-gray-50 px-5 py-3.5">
    <label class="flex cursor-pointer items-center gap-2 text-sm text-gray-700 select-none">
      <input type="checkbox" bind:checked={show_changed} class="h-4 w-4 rounded border-gray-300 accent-amber-500" />
      Geänderte Teams markieren
    </label>
    <label class="flex cursor-pointer items-center gap-2 text-sm text-gray-700 select-none">
      <input type="checkbox" bind:checked={show_state} class="h-4 w-4 rounded border-gray-300 accent-blue-600" />
      Anwesenheitsstatus anzeigen
    </label>
    <label class="flex cursor-pointer items-center gap-2 text-sm text-gray-700 select-none">
      <input type="checkbox" bind:checked={sort_by_group} class="h-4 w-4 rounded border-gray-300 accent-purple-600" />
      Nach Gruppen sortieren
    </label>
    <label class="flex cursor-pointer items-center gap-2 text-sm text-gray-700 select-none">
      <input type="checkbox" bind:checked={show_delimiters} class="h-4 w-4 rounded border-gray-300 accent-gray-500" />
      Trennlinien zwischen Teams
    </label>
    <button
      onclick={() => window.print()}
      class="ml-auto rounded-lg bg-gray-800 px-3 py-1.5 text-sm font-medium text-white hover:bg-gray-700"
    >
      Drucken
    </button>
  </div>

  {#if loading}
    <p class="text-sm text-gray-400">Lade…</p>

  {:else if sortedOrgs.length === 0}
    <p class="text-sm text-gray-400">Keine angemeldeten Teams gefunden.</p>

  {:else if sort_by_group}
    <!-- ── Per-group sections ───────────────────────────────────────── -->
    {#each sortedByGroup as { group, orgs }, gi}
      <section class="{gi > 0 ? 'mt-7 print:mt-0 print:break-before-page' : ''}">
        <!-- Group heading -->
        <h2 class="mb-4 border-b-4 pb-1.5 text-lg font-bold text-gray-900" style="border-color: {group.color};">
          {group.name}
        </h2>

        <div class="space-y-5 pl-2">
          {#each orgs as { event_org, teams }}
            <div>
              <p class="mb-1 text-xs font-semibold uppercase tracking-widest text-gray-500">
                {event_org.org.name}
              </p>
              <ol class="{show_delimiters ? '' : 'space-y-px'} pl-3">
                {#each teams as team, j}
                  {@const absent = show_state && team.presence_state === "absent"}
                  {@const changed = show_changed && team.changed_since_export}
                  <li class="flex items-center gap-x-2 text-sm leading-snug
                    {show_delimiters ? 'border border-gray-300 px-2 py-1.5 -mx-2 ' + (j > 0 ? '-mt-px' : '') : ''}
                    {changed ? 'bg-amber-50' : ''}">
                    <span class="w-5 shrink-0 text-right tabular-nums text-gray-400">{j + 1}.</span>
                    <span class="flex-1 font-medium {absent ? 'line-through text-gray-400' : 'text-gray-900'}">{team.name}</span>
                    {#if changed}
                      <span class="rounded bg-amber-400 px-1.5 py-0.5 text-[10px] font-bold uppercase leading-none text-amber-900">neu</span>
                    {/if}
                    {#if show_state}
                      {@const icon = presenceIcon(team.presence_state)}
                      <span class="{icon.class}">{icon.symbol}</span>
                    {/if}
                    {#if show_delimiters}
                      <span class="ml-auto shrink-0 h-5 w-5 border border-gray-400"></span>
                    {/if}
                  </li>
                {/each}
              </ol>
            </div>
          {/each}
        </div>
      </section>
    {/each}

  {:else}
    <!-- ── Per-org sections ─────────────────────────────────────────── -->
    {#each sortedOrgs as { event_org, groupedTeams }, i}
      {#if i > 0}
        <hr class="my-7 border-gray-200" />
      {/if}

      <section>
        <!-- Org heading -->
        <h2 class="mb-3 border-b-2 border-gray-800 pb-1.5 text-base font-bold text-gray-900">
          {event_org.org.name}
        </h2>

        <!-- Groups & teams -->
        <div class="space-y-3 pl-2">
          {#each groupedTeams as { groupName, teams }}
            <div>
              <p class="mb-1 text-xs font-semibold uppercase tracking-widest text-gray-400">
                {groupName}
              </p>
              <ol class="{show_delimiters ? '' : 'space-y-px'} pl-3">
                {#each teams as team, j}
                  {@const absent = show_state && team.presence_state === "absent"}
                  {@const changed = show_changed && team.changed_since_export}
                  <li class="flex items-center gap-x-2 text-sm leading-snug
                    {show_delimiters ? 'border border-gray-300 px-2 py-1.5 -mx-2 ' + (j > 0 ? '-mt-px' : '') : ''}
                    {changed ? 'bg-amber-50' : ''}">
                    <span class="w-5 shrink-0 text-right tabular-nums text-gray-400">{j + 1}.</span>
                    <span class="flex-1 font-medium {absent ? 'line-through text-gray-400' : 'text-gray-900'}">{team.name}</span>
                    {#if changed}
                      <span class="rounded bg-amber-400 px-1.5 py-0.5 text-[10px] font-bold uppercase leading-none text-amber-900">neu</span>
                    {/if}
                    {#if show_state}
                      {@const icon = presenceIcon(team.presence_state)}
                      <span class="{icon.class}">{icon.symbol}</span>
                    {/if}
                    {#if show_delimiters}
                      <span class="ml-auto shrink-0 h-5 w-5 border border-gray-400"></span>
                    {/if}
                  </li>
                {/each}
              </ol>
            </div>
          {/each}
        </div>
      </section>
    {/each}

    <!-- ── Email list ──────────────────────────────────────────────── -->
    {#if emailList.length > 0}
      <hr class="my-8 border-gray-200 print:hidden" />
      <section class="print:hidden">
        <h2 class="mb-2 text-xs font-semibold uppercase tracking-widest text-gray-400">
          E-Mail-Empfänger
        </h2>
        <p class="mb-2 text-xs text-gray-400">Für das „An:"-Feld kopieren:</p>
        <textarea
          readonly
          rows={Math.min(emailList.length, 12)}
          class="w-full rounded-lg border border-gray-200 bg-gray-50 px-3 py-2 font-mono text-xs text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-300"
          value={emailList.join(",\n")}
        ></textarea>
      </section>
    {/if}

  {/if}

</main>
