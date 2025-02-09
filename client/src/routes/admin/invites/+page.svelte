<script lang="ts">
  import { onMount } from "svelte";
  import {
    getServerUrl,
    listInvites,
    sendInvite,
    type Invite,
  } from "../../../api/api";
  import {
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";
  import type { ApiRequest } from "@cocreators-ee/apity";
  import type { components } from "../../../api/api.d";
  import { goto, invalidateAll } from "$app/navigation";
  import FetchErrors from "../../FetchErrors.svelte";
  import { page } from "$app/state";
  import Loading from "../../Loading.svelte";

  let loading = false;
  let sending = $state(false);
  let fetch_errors: FetchErrors;
  let invites: Invite[] = $state([]);

  onMount(async () => {
    const request = listInvites({});
    const resp = await request.result;
    if (resp.ok) {
      invites = resp.data;
    } else {
      fetch_errors.check(resp);
    }
  });

  async function send_invite() {
    sending = true;
    const invite_res = await sendInvite({ send: true }).result;
    if (invite_res.ok) {
      invites.shift();
      sending = false;
    } else {
      alert("failed to send invite");
    }
  }
</script>

<main>
  <FetchErrors bind:this={fetch_errors} />
  {#if sending}
    <Loading text="Sending invite..." />
  {/if}

  <Table striped={true}>
    <TableBody>
      {#each invites as invite, index}
        <TableBodyRow>
          <TableBodyCell>
            {#if index == 0}
              <Button class="m-10" on:click={send_invite}
                >Send one more invite...</Button
              >
            {/if}
            <pre>
To: {invite.org.contact_name} &lt;{invite.org.contact_email}&gt;

Hallo {invite.org.name},

vielen Dank für die Registrierung bei dem Turnier "{invite.event.name}".

Zur Anmeldung der Mannschaften benutzen Sie bitte den folgenden Link:

<a href="{getServerUrl()}/org/{invite.org.id}/{invite.event.id}/{invite.secret}"
                >{getServerUrl()}/org/{invite.org.id}/{invite.event
                  .id}/{invite.secret}

Vielen Dank!
                </a></pre>
          </TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
</main>
