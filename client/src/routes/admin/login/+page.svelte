<script lang="ts">
  import { authLogin, authLogout, whoAmI } from "../../../api/api";
  import { onMount } from "svelte";

  let email = $state("");
  let password = $state("");
  let login_ok = $state(false);
  let login_failed = $state(false);
  let user_email: string | null = $state(null);

  onMount(async () => {
    whoAmI({}).resp.subscribe((resp) => {
      user_email = resp?.ok ? resp.data.email : "";
    });
  });

  async function login() {
    const resp = await authLogin({ email, password }).result;
    if (resp.ok) {
      login_ok = true;
      login_failed = false;
    } else {
      login_failed = true;
    }
  }

  async function logout() {
    await authLogout({});
    login_ok = false;
    user_email = "";
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Enter") login();
  }
</script>

<div class="flex min-h-screen items-center justify-center bg-gray-50 px-4">
  <div class="w-full max-w-sm">

    <div class="mb-8 text-center">
      <h1 class="text-2xl font-bold text-gray-900">Administration</h1>
      <p class="mt-1 text-sm text-gray-500">Lübecker Schachverein von 1873</p>
    </div>

    <div class="rounded-xl border border-gray-200 bg-white p-8 shadow-sm">

      {#if login_ok}
        <div class="text-center">
          <div class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-green-100 text-green-600 text-2xl">✓</div>
          <p class="font-medium text-gray-900">Erfolgreich eingeloggt.</p>
          <p class="mt-1 text-sm text-gray-500">Sie können zurücknavigieren.</p>
        </div>

      {:else if user_email === null}
        <!-- loading -->

      {:else if user_email}
        <div class="text-center">
          <p class="text-sm text-gray-500">Eingeloggt als</p>
          <p class="mt-1 font-medium text-gray-900">{user_email}</p>
          <button
            onclick={logout}
            class="mt-6 w-full rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 transition-colors"
          >
            Ausloggen
          </button>
        </div>

      {:else}
        <div class="mb-6">
          <p class="text-sm text-gray-600">
            Für den Administrationsbereich bitte einloggen. Für die Verwaltung
            von Mannschaften folgen Sie dem Link aus der E-Mail.
          </p>
        </div>

        {#if login_failed}
          <div class="mb-4 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
            E-Mail oder Passwort falsch.
          </div>
        {/if}

        <div class="flex flex-col gap-4">
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700" for="email">E-Mail</label>
            <input
              id="email"
              type="email"
              bind:value={email}
              {onkeydown}
              placeholder="admin@example.com"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
            />
          </div>
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700" for="password">Passwort</label>
            <input
              id="password"
              type="password"
              bind:value={password}
              {onkeydown}
              placeholder="••••••••"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
            />
          </div>
          <button
            onclick={login}
            class="w-full rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 transition-colors"
          >
            Einloggen
          </button>
        </div>
      {/if}

    </div>
  </div>
</div>
