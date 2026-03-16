<script>
  import { page } from "$app/state";
  import img_verein from "$lib/images/Logo_Final_Schachverein.png";

  let mobileOpen = $state(false);

  const links = [
    { href: "/admin/events",  label: "Events"      },
    { href: "/admin/orgs",    label: "Schulen"     },
    { href: "/admin/invites", label: "Einladungen" },
    { href: "/admin/login",   label: "Login"       },
  ];

  const current = $derived(page.url.pathname);
</script>

<nav class="border-b border-gray-200 bg-white shadow-sm">
  <div class="mx-auto flex max-w-7xl items-center justify-between gap-4 px-4 py-2">

    <!-- Brand -->
    <a href="/admin" class="flex shrink-0 items-center gap-2">
      <img src={img_verein} class="h-7" alt="LSV1873 Logo" />
      <span class="hidden text-sm font-semibold text-gray-700 sm:inline">Admin</span>
    </a>

    <!-- Desktop links -->
    <div class="hidden items-center gap-1 sm:flex">
      {#each links as link}
        <a
          href={link.href}
          class="rounded-md px-3 py-1.5 text-sm font-medium transition-colors
            {current.startsWith(link.href)
              ? 'bg-blue-50 text-blue-700'
              : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'}"
        >{link.label}</a>
      {/each}
    </div>

    <!-- Mobile hamburger -->
    <button
      onclick={() => (mobileOpen = !mobileOpen)}
      class="flex items-center justify-center rounded-md p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-700 sm:hidden"
      aria-label="Menü öffnen"
    >
      {#if mobileOpen}
        <!-- X icon -->
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
        </svg>
      {:else}
        <!-- Hamburger icon -->
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      {/if}
    </button>
  </div>

  <!-- Mobile menu -->
  {#if mobileOpen}
    <div class="border-t border-gray-100 px-4 py-2 sm:hidden">
      {#each links as link}
        <a
          href={link.href}
          onclick={() => (mobileOpen = false)}
          class="block rounded-md px-3 py-2 text-sm font-medium transition-colors
            {current.startsWith(link.href)
              ? 'bg-blue-50 text-blue-700'
              : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'}"
        >{link.label}</a>
      {/each}
    </div>
  {/if}
</nav>

<slot />
