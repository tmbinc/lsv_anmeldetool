<script lang="ts">
  let props = $props();
</script>

<div class="w-full mt-24 flex items-center justify-center gap-4">
  <div
    class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-blue-600"
  ></div>

  <h1 class="text-2xl font-semibold text-gray-900">{props.text}</h1>
</div>
