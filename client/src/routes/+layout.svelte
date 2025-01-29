<script lang="ts">
  import {
    Footer,
    FooterLink,
    FooterLinkGroup,
    Navbar,
    NavBrand,
    NavHamburger,
    NavLi,
    NavUl,
  } from "flowbite-svelte";
  import "../app.css";
  import { onMount } from "svelte";
  import { whoAmI } from "../api/api";
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
</script>

<nav>
  <Navbar let:hidden let:toggle>
    <NavBrand href="/">
      <img
        src="https://lsv1873.de/images/Joa/Logo_Final_Schachverein.png"
        class="mr-3 h-6 sm:h-9"
        alt="LSV1873 Logo"
      />
      <span
        class="self-center whitespace-nowrap text-xl font-semibold dark:text-white"
      >
        Turnier-Anmeldungen
      </span>
    </NavBrand>
    <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
      <NavLi href="/admin/login">
        {#if user_email != null}
          {#if user_email}
            Logged in as {user_email}
          {:else}
            Not logged in
          {/if}
        {/if}
      </NavLi>
      <NavLi href="/">Home</NavLi>
      <NavLi href="/admin">Admin</NavLi>
    </NavUl>
  </Navbar>
</nav>

<slot />

<div></div>
<Footer>
  <FooterLinkGroup
    ulClass="flex flex-wrap items-center mt-3 text-sm text-gray-500 dark:text-gray-400 sm:mt-0"
  >
    <FooterLink href="/">Über</FooterLink>
    <FooterLink href="/privacy">Datenschutz</FooterLink>
    <FooterLink href="mailto:Felix@Dom.ke">Kontakt</FooterLink>
  </FooterLinkGroup>
</Footer>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
