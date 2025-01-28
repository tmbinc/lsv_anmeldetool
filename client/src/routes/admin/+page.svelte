<script>
  import { Button } from "flowbite-svelte";
  import { authLogin } from "../../api/api";

  let email = $state("");
  let password = $state("");
  let login_ok = $state(false);
  let login_failed = $state(false);

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
</script>

<div>Admin Page</div>

{#if login_ok}
  Logged in
  <Button href="/admin/orgs">Edit orgs...</Button>
  <Button href="/admin/teams">Edit teams...</Button>
  <Button href="/admin/events">Edit events...</Button>
{:else}
  {#if login_failed}
    Login failed.
  {/if}
  <div>
    <input type="email" name="email" bind:value={email} />
    <input type="password" name="password" bind:value={password} />
    <Button on:click={() => login()}>Login</Button>
  </div>
{/if}
