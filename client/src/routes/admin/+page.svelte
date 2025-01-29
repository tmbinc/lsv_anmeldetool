<script>
  import { Button, ButtonGroup } from "flowbite-svelte";
  import { authLogin } from "../../api/api";
</script>

<div>Admin Page</div>

<ButtonGroup>
  <Button href="/admin/orgs">Edit orgs...</Button>
  <Button href="/admin/teams">Edit teams...</Button>
  <Button href="/admin/events">Edit events...</Button>
</ButtonGroup>
