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

<div class="m-10 shadow bg-white rounded-xl">
  {#if login_ok}
    <div class="flex-row text-center">
      <div class="m-5">Logged in. Please navigate yourself.</div>
    </div>
  {:else}
    {#if login_failed}
      <Alert>Login failed.</Alert>
    {/if}

    {#if user_email != null}
      {#if user_email}
        <div class="flex-row text-center">
          <div class="m-5">Logged in as {user_email}</div>
          <Button class="m-5" on:click={() => logout()}>Logout</Button>
        </div>
      {:else}
        <div class="flex-row">
          <div class="m-5 text-center">Aktuell nicht eingeloggt.</div>
          <div class="m-5 font-bold">
            Für den Administrationsbereich bitte einloggen.
          </div>
          <div class="m-5">
            (Für die Verwaltung von Mannschaften ist ein einloggen nicht
            erforderlich; bitte folgen Sie dem Link aus der Email.)
          </div>
          <div
            class="flex flex-wrap items-center mt-3 text-sm text-gray-500 dark:text-gray-400 sm:mt-0"
          >
            <input
              class="m-3"
              type="email"
              name="email"
              bind:value={email}
              placeholder="username"
            />
            <input
              class="m-3"
              type="password"
              name="password"
              placeholder="password"
              bind:value={password}
            />
            <Button class="m-3" on:click={() => login()}>Login</Button>
          </div>
        </div>
      {/if}
    {/if}
  {/if}
</div>
