<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import {
    createOrgSelfReg,
    getEvent,
    getEventOrgs,
    listOrgs,
    setEventOrgState,
    type Event,
    type EventOrg,
    type Org,
  } from "../../../api/api";
  import {
    Table,
    TableHead,
    TableHeadCell,
    TableBody,
    TableBodyRow,
    TableBodyCell,
    Button,
    FloatingLabelInput,
    Helper,
  } from "flowbite-svelte";
  import { EnvelopeSolid } from "flowbite-svelte-icons";
  import Loading from "../../Loading.svelte";

  let event: Event | null = $state(null);
  let loading = $state(true);
  let submitting = $state(false);
  let submitted = $state(false);
  let failed = $state(false);
  let contact_name = $state("");
  let contact_email = $state("");
  let org_name = $state("");

  const event_id = page.params.id;

  onMount(async () => {
    const request = getEvent({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      event = resp.data;
      loading = false;
    }
  });

  async function submit_reg() {
    submitting = true;
    const result = await createOrgSelfReg({
      name: org_name,
      event_id: event_id,
      contact_email: contact_email,
      contact_name: contact_name,
    }).result;

    if (result.ok) {
      submitted = true;
      submitting = false;
    } else {
      submitting = false;
      failed = true;
    }
  }
</script>

<div class="px-10 pt-6 flex flex-col gap-4">
  {#if loading}
    <Loading text="Lade Turnierdetails..." />
  {:else if submitting}
    <Loading text="Speichere Anmeldung..." />
  {:else if submitted}
    <div class="w-full mt-24 flex items-center justify-center gap-4">
      <h1 class="text-2xl font-semibold text-gray-900">
        Anmeldung erfolgreich gespeichert!
      </h1>
    </div>
  {:else}
    <h1
      class="mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white"
    >
      {event?.name}
    </h1>

    <div class="w-full gap-4">
      {@html event?.description}
    </div>

    {#if event?.public}
      <h2
        class="mb-4 text-xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white"
      >
        Teilnahme
      </h2>

      {#if failed}
        <div class="w-full items-center justify-center">
          <h1 class="text-2xl font-semibold text-orange-900">
            Fehler beim Speichern der Anmeldung! Bitte noch einmal probieren
            oder per E-Mail an <a href="mailto:anmeldungen@lsv1873.de"
              >anmeldungen@lsv1873.de</a
            > senden!
          </h1>
        </div>
      {/if}

      Abfolge:
      <ol class="ps-5 mt-2 space-y-1 list-decimal list-inside">
        <li>
          Jede Schule meldet über dieses Formular grundsätzlich ein Interesse an
          einer Teilnahme an.
        </li>
        <li>
          Wir versenden dann einen Link an die angegebene E-Mail-Adresse, der
          zur Bestätigung der Anmeldung dient. Unter diesem Link können dann die
          teilnehmenden Mannschaften {#if event?.public_reg_until}bis zum {event?.public_reg_until}{/if}
          mit den entsprechenden Altersgruppen gemeldet werden.
        </li>
        <li>
          Am Turniertag {#if event?.begin}({event?.begin}){/if} werden die Anmeldungen
          vor Ort bestätigt.
        </li>
      </ol>

      Zur Teilnahme bitte die folgenden Daten eingeben:
      <form method="POST" action="/event/{event?.id}/submitted">
        <div class="grid gap-6 items-end w-full">
          <FloatingLabelInput
            style="outlined"
            id="floating_outlined"
            name="floating_outlined"
            bind:value={org_name}
            type="text"
          >
            Name der Schule
          </FloatingLabelInput>
          <Helper class="pt-2">Bitte den Namen der Schule eintragen.</Helper>

          <div class="grid gap-6 mb-6 md:grid-cols-2">
            <FloatingLabelInput
              style="outlined"
              id="floating_outlined"
              name="floating_outlined"
              bind:value={contact_name}
              type="text"
            >
              Kontaktperson (Name)
            </FloatingLabelInput>

            <Helper class="pt-2"
              >Bitte den Namen einer Kontaktperson für diese Schule eintragen.
              Diese Person muss nicht am Turniertag anwesend sein, sollte aber
              für organisatorische Anfragen vor dem Turnier zur Verfügung
              stehen.</Helper
            >

            <FloatingLabelInput
              style="outlined"
              id="floating_outlined"
              name="floating_outlined"
              type="text"
              bind:value={contact_email}
            >
              Email-Adresse
            </FloatingLabelInput>
            <Helper class="pt-2"
              >Für die Anmeldung wird eine E-Mail an die angegebene Adresse
              verschickt.</Helper
            >
          </div>
          <Button on:click={submit_reg}>Absenden</Button>
        </div>
      </form>
    {:else}
      Bitte benutzen Sie den Link aus der E-Mail, um sich für dieses Turnier
      anzumelden.
    {/if}
  {/if}
</div>
