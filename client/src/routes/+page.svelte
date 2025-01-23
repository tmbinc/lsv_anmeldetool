<script>
  import { onMount } from "svelte";
  import { listOrgs, getOrg } from "../api/api";
  import {
    Footer,
    FooterCopyright,
    FooterLinkGroup,
    FooterLink,
    Button,
  } from "flowbite-svelte";
</script>

<main>Hello!</main>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
