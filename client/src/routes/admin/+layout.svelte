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
      <NavLi href="/admin/invites">Invites</NavLi>
      <NavLi href="/admin/login">Login</NavLi>
    </NavUl>
  </Navbar>
</nav>

<slot />
