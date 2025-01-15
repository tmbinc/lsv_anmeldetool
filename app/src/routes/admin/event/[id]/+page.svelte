<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { getEvent, type Event } from "../../../../api/api";

  let event: Event | null = $state(null);

  onMount(async () => {
    const request = getEvent({ event: page.params.id });
    const resp = await request.result;
    if (resp.ok) {
      event = resp.data;
    }
  });
</script>

<h1>Event Details</h1>

<h1>
  {event?.name}
</h1>
