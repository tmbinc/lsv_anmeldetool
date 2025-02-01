<script lang="ts">
  import { Alert, Button } from "flowbite-svelte";
  import { authLogin, authLogout, whoAmI } from "../../../api/api";
  import { onMount } from "svelte";
  import type { ApiResponse } from "@cocreators-ee/apity";

  let email = $state("");
  let password = $state("");
  let login_ok = $state(false);
  let login_failed = $state(false);
  let user_email: string | null = $state(null);

  onMount(async () => {
    const request = whoAmI({}).resp.subscribe((resp) => {
      if (resp?.ok) {
        user_email = resp.data.email;
      } else {
        user_email = "";
      }
    });
  });

  async function login() {
    const resp = await authLogin({ email: email, password: password }).result;
    if (resp.ok) {
      console.log(resp);
      login_ok = true;
      login_failed = false;
    } else {
      login_failed = true;
    }
  }

  async function logout() {
    const resp = await authLogout({});
    login_ok = false;
    user_email = "";
  }
</script>

{#if login_ok}
  Logged in
{:else}
  {#if login_failed}
    <Alert>Login failed.</Alert>
  {/if}

  {#if user_email != null}
    {#if user_email}
      <h1>Logged in as {user_email}</h1>
      <Button on:click={() => logout()}>Logout</Button>
    {:else}
      Aktuell nicht eingeloggt.
      <h1>Für den Administrationsbereich bitte einloggen.</h1>
      <div>
        (Für die Verwaltung von Mannschaften ist ein einloggen nicht
        erforderlich; bitte folgen Sie dem Link aus der Email.)
      </div>
      <div>
        <input type="email" name="email" bind:value={email} />
        <input type="password" name="password" bind:value={password} />
        <Button on:click={() => login()}>Login</Button>
      </div>
    {/if}
  {/if}
{/if}
