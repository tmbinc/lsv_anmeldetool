<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { createOrgSelfReg, getEvent, type Event } from "../../../api/api";
  import {
    Alert,
    Button,
    FloatingLabelInput,
    Helper,
    Modal,
  } from "flowbite-svelte";
  import {
    CheckCircleOutline,
    ExclamationCircleOutline,
  } from "flowbite-svelte-icons";
  import Loading from "../../Loading.svelte";
  import LoadError from "../../LoadError.svelte";

  let event: Event | null = $state(null);
  let loading = $state(true);
  let submitting = $state(false);
  let submitted = $state(false);
  let failed_submit = $state(false);
  let failed_load = $state(false);
  let contact_name = $state("");
  let contact_email = $state("");
  let org_name = $state("");
  let verify_failed: string | null = $state(null);
  let verify_failed_open = $state(false);

  const event_id = page.params.id;

  const is_valid_email = (email: string) => {
    return String(email)
      .toLowerCase()
      .match(
        /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|.(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
      );
  };

  onMount(async () => {
    const request = getEvent({ event: event_id });
    const resp = await request.result;
    if (resp.ok) {
      event = resp.data;
    } else {
      failed_load = true;
    }
    loading = false;
  });

  async function submit_reg() {
    if (org_name.length < 3) {
      verify_failed = "org_name_too_short";
      verify_failed_open = true;
    } else if (contact_name.length < 3) {
      verify_failed = "contact_name_too_short";
      verify_failed_open = true;
    } else if (!is_valid_email(contact_email)) {
      verify_failed = "email_invalid";
      verify_failed_open = true;
    } else {
      verify_failed = null;
      submitting = true;

      const result = await createOrgSelfReg({
        name: org_name,
        event_id: event_id,
        contact_email: contact_email,
        contact_name: contact_name,
      }).result;

      if (result.ok) {
        submitted = true;
      } else {
        failed_submit = true;
      }
      submitting = false;
    }
  }
</script>

<div class="px-10 pt-6 flex flex-col gap-4">
  {#if loading}
    <Loading text="Lade Turnierdetails..." />
  {:else if submitting}
    <Loading text="Speichere Anmeldung..." />
  {:else if failed_load}
    <LoadError text="Laden fehlgeschlagen!" />
  {:else if submitted}
    <div class="w-full mt-24 items-center justify-center gap-4">
      <div class="inline">
        <CheckCircleOutline
          color="green"
          class="mx-auto mb-4 w-12 h-12 inline"
        />
        <div class="text-2xl font-semibold text-gray-900 inline">
          Anmeldung erfolgreich gespeichert!
        </div>
      </div>
      <p class="mb-10">
        Sie bekommen in den nächsten Tagen eine Email an {contact_email} mit weiteren
        Informationen.
      </p>
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

      {#if failed_submit}
        <div class="w-full items-center justify-center">
          <h1 class="text-2xl font-semibold text-orange-900">
            Fehler beim Speichern der Anmeldung! Bitte noch einmal probieren
            oder per E-Mail an <a href="mailto:anmeldungen@lsv1873.de"
              >anmeldungen@lsv1873.de</a
            > senden!
          </h1>
        </div>
      {/if}

      {#if new Date(event?.begin || 0) < new Date()}
        <Alert>Dieses Turnier liegt in der Vergangenheit.</Alert>
      {:else if new Date(event?.public_reg_until || 0) < new Date()}
        <Alert
          >Die Anmeldefrist für dieses Turnier ist bereit abgelaufen. Eine
          Anmeldung über dieses Formular kann trotzdem vorgenommen werden, aber
          wir können eine Teilnahme nicht garantieren.
        </Alert>
      {/if}

      Abfolge:
      <ol class="ps-5 mt-2 space-y-1 list-decimal list-inside">
        <li>
          Jede Schule meldet über dieses Formular grundsätzlich das Interesse an
          einer Teilnahme an.
        </li>
        <li>
          Wir versenden dann einen Link an die angegebene E-Mail-Adresse. Unter
          diesem Link können dann die teilnehmenden Mannschaften {#if event?.public_reg_until}bis
            zum {new Date(
              event?.public_reg_until || 0
            ).toLocaleDateString()}{/if}
          gemeldet werden.
        </li>
        <li>
          Am Turniertag {#if event?.begin}({new Date(
              event?.begin || 0
            ).toLocaleDateString()}){/if} werden die Anmeldungen vor Ort bestätigt.
        </li>
      </ol>

      Zur Teilnahme bitte die folgenden Daten eingeben:
      <form>
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
              type="email"
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

  <Modal bind:open={verify_failed_open} size="xs" autoclose>
    <div class="text-center">
      <ExclamationCircleOutline
        class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200"
      />
      <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
        {#if verify_failed == "org_name_too_short"}
          Bitte den Namen der Schule eingeben!
        {:else if verify_failed == "contact_name_too_short"}
          Bitte Kontaktperson angeben!
        {:else if verify_failed == "email_invalid"}
          Bitte eine gültige Email angeben!
        {/if}
      </h3>
      <Button color="alternative">Schliessen</Button>
    </div>
  </Modal>
</div>
