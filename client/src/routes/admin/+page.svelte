<script>
  import { Button, ButtonGroup } from "flowbite-svelte";
  import { authLogin } from "../../api/api";
</script>

<main class="m-10 flex-row">
  <div class="text-center mb-5">Admin Page</div>

  <div class="text-center">
    <ButtonGroup>
      <Button href="/admin/orgs">Edit orgs...</Button>
      <Button href="/admin/teams">Edit teams...</Button>
      <Button href="/admin/events">Edit events...</Button>
    </ButtonGroup>
  </div>
</main>
