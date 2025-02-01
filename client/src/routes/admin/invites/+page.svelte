<script lang="ts">
  import { onMount } from "svelte";
  import { getServerUrl, listInvites, type Invite } from "../../../api/api";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";
  import type { ApiRequest } from "@cocreators-ee/apity";
  import type { components } from "../../../api/api.d";
  import { goto } from "$app/navigation";
  import FetchErrors from "../../FetchErrors.svelte";

  let loading = false;
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
</script>

<main>
  <FetchErrors bind:this={fetch_errors} />

  <Table>
    <TableBody>
      {#each invites as invite}
        <TableBodyRow>
          <TableBodyCell>
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
