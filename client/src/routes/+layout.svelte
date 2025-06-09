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
  import { page } from "$app/stores";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
</svelte:head>

{#if !$page.url.pathname.includes("/run/ceremony")}
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
    </Navbar>
  </nav>

  <slot />

  <div></div>
  <Footer class="mx-10">
    <FooterLinkGroup
      ulClass="flex flex-wrap items-center mt-3 text-sm text-gray-500 dark:text-gray-400 sm:mt-0"
    >
      <FooterLink href="/">Über</FooterLink>
      <FooterLink href="/privacy">Datenschutz</FooterLink>
      <FooterLink href="mailto:anmeldung@lsv1873.de">Kontakt</FooterLink>
    </FooterLinkGroup>
  </Footer>

  <style lang="postcss">
    :global(html) {
      background-color: theme(colors.gray.50);
    }
  </style>
{:else}
  <slot />
{/if}
