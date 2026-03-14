<script lang="ts">
  import { Navbar, NavBrand } from "flowbite-svelte";
  import "../app.css";
  import { page } from "$app/stores";
  import img_verein from "$lib/images/Logo_Final_Schachverein.png";
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link
    rel="icon"
    href="https://lsv1873.de/wp-content/uploads/2025/12/cropped-cropped-Icon-2-1-32x32.png"
  />
</svelte:head>

{#if !$page.url.pathname.includes("/run/ceremony") && !$page.url.pathname.includes("/timetable_pairing") && !$page.url.pathname.includes("/paarvis")}
  <nav>
    <Navbar let:hidden let:toggle>
      <NavBrand href="/">
        <img src={img_verein} class="mr-3 h-6 sm:h-9" alt="LSV1873 Logo" />
        <span
          class="self-center whitespace-nowrap text-xl font-semibold dark:text-white"
        >
          Turnier-Anmeldungen
        </span>
      </NavBrand>
    </Navbar>
  </nav>

  <slot />

  <footer class="mt-16 border-t border-gray-200 bg-white">
    <div
      class="mx-auto max-w-5xl px-6 py-6 flex flex-col items-center gap-3 sm:flex-row sm:justify-between"
    >
      <span class="text-sm text-gray-400"
        ><a href="https://www.lsv1873.de/">Lübecker Schachverein von 1873</a
        ></span
      >
      <div class="flex gap-5 text-sm">
        <a
          href="/privacy"
          class="text-gray-400 hover:text-gray-700 transition-colors"
          >Datenschutz</a
        >
        <a
          href="mailto:anmeldung@lsv1873.de"
          class="text-gray-400 hover:text-gray-700 transition-colors">Kontakt</a
        >
      </div>
    </div>
  </footer>

  <style lang="postcss">
    :global(html) {
      background-color: theme(colors.gray.50);
    }
  </style>
{:else}
  <slot />
{/if}
