<script lang="ts">
  import { Alert } from "flowbite-svelte";

  let props = $props();
</script>

<div class="w-full mt-24 flex items-center justify-center gap-4">
  <div
    class="animate-bounce rounded-full h-8 w-8 border-t-2 border-b-2 border-red-600"
  ></div>

  <Alert>{props.text}</Alert>
</div>
