<script>
  import {
    Navbar,
    NavBrand,
    NavHamburger,
    NavUl,
    NavLi,
  } from "flowbite-svelte";
  import { page } from "$app/state";

  const event_id = page.params.event_id;
</script>

<nav>
  <Navbar let:hidden let:toggle>
    <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
      <NavLi href="/admin/event/{event_id}/">Turnier-Administration</NavLi>

      <NavLi href="/admin/event/{event_id}/run/registration/"
        >Registrierung</NavLi
      >
      <NavLi href="/admin/event/{event_id}/run/timetable/">Zeitplanung</NavLi>
      <NavLi href="/event/{event_id}/paarvis/">paarvis</NavLi>
      <NavLi href="/event/{event_id}/timetable_pairing/"
        >timetable_pairing</NavLi
      >
      <NavLi href="/event/{event_id}/timetable/">timetable</NavLi>
      <NavLi href="/admin/event/{event_id}/run/results/"
        >Turnierleitungsfunktionen</NavLi
      >
      <NavLi href="/admin/event/{event_id}/run/ceremony/">Siegerehrung</NavLi>
    </NavUl>
  </Navbar>
</nav>

<slot />
