<script>
  import {
    Navbar,
    NavBrand,
    NavHamburger,
    NavUl,
    NavLi,
  } from "flowbite-svelte";
</script>

<nav>
  <Navbar let:hidden let:toggle>
    <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
      <NavLi href="/admin/events">Events</NavLi>
      <NavLi href="/admin/orgs">Orgs</NavLi>
      <NavLi href="/admin/teams">Teams</NavLi>
    </NavUl>
  </Navbar>
</nav>

<slot />
