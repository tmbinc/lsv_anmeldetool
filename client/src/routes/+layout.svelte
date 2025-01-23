<script>
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
    <FooterLink href="/">About</FooterLink>
    <FooterLink href="/">Privacy Policy</FooterLink>
    <FooterLink href="/">Contact</FooterLink>
  </FooterLinkGroup>
</Footer>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
