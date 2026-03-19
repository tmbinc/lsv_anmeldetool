<script lang="ts">
  import { onMount } from "svelte";
  import QRCode from "qrcode";
  import { browser } from "$app/environment";

  let { url }: { url: string } = $props();

  let dataUrl = $state("");

  $effect(() => {
    if (!browser) return;
    QRCode.toDataURL(url, { margin: 1, width: 128 }).then((d) => (dataUrl = d));
  });
</script>

{#if dataUrl}
  <div class="pointer-events-none fixed right-4 bottom-4 z-50">
    <img src={dataUrl} alt="QR-Code" class="h-24 w-24 rounded shadow-md" />
  </div>
{/if}
