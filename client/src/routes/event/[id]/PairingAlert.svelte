<script lang="ts">
  import { onDestroy } from "svelte";

  // `signal` is incremented by the parent's SSE handler each time a "pairing"
  // event arrives — no separate SSE connection needed here.
  let { event_id, signal }: { event_id: string; signal: number } = $props();

  let visible = $state(false);
  let flashing = $state(false);
  let progress = $state(100);

  let dismissTimer: ReturnType<typeof setTimeout> | null = null;
  let progressInterval: ReturnType<typeof setInterval> | null = null;

  const DURATION = 20_000;

  function clearTimers() {
    if (dismissTimer)     clearTimeout(dismissTimer);
    if (progressInterval) clearInterval(progressInterval);
    dismissTimer = progressInterval = null;
  }

  function trigger() {
    clearTimers();

    flashing = true;
    setTimeout(() => (flashing = false), 900);

    visible = true;
    progress = 100;
    const t0 = Date.now();
    progressInterval = setInterval(() => {
      progress = Math.max(0, 100 - ((Date.now() - t0) / DURATION) * 100);
    }, 100);
    dismissTimer = setTimeout(() => { visible = false; clearTimers(); }, DURATION);
  }

  function dismiss() {
    visible = false;
    clearTimers();
  }

  // Fire whenever the parent increments the signal (skip the initial 0).
  $effect(() => {
    if (signal > 0) trigger();
  });

  onDestroy(clearTimers);
</script>

<!-- Flash overlay: briefly covers the viewport, pointer-events disabled so nothing is blocked -->
{#if flashing}
  <div class="pointer-events-none fixed inset-0 z-40 flash-overlay"></div>
{/if}

<!-- Floating pill: sits at bottom, never covers the main content column -->
{#if visible}
  <div class="fixed bottom-5 left-1/2 z-50 -translate-x-1/2 overflow-hidden rounded-full border border-green-200 bg-white shadow-lg shadow-green-100/60"
       role="status">
    <div class="flex items-center gap-2.5 px-4 py-2.5">
      <span class="h-2 w-2 shrink-0 animate-pulse rounded-full bg-green-500"></span>
      <span class="whitespace-nowrap text-sm font-medium text-gray-800">Neue Paarungen verfügbar</span>
      <a
        href="/event/{event_id}/pairings"
        onclick={dismiss}
        class="whitespace-nowrap text-sm font-semibold text-green-700 hover:text-green-900"
      >Ansehen →</a>
      <button
        onclick={dismiss}
        class="ml-0.5 text-lg leading-none text-gray-300 hover:text-gray-500"
        aria-label="Schließen"
      >×</button>
    </div>
    <!-- Progress bar shows remaining display time -->
    <div class="h-0.5 bg-gray-100">
      <div class="h-full bg-green-400" style="width: {progress}%; transition: width 0.1s linear;"></div>
    </div>
  </div>
{/if}

<style>
  .flash-overlay {
    background: #bbf7d0; /* green-200 */
    animation: flash-fade 0.9s ease-out forwards;
  }
  @keyframes flash-fade {
    0%   { opacity: 0;   }
    20%  { opacity: 0.5; }
    100% { opacity: 0;   }
  }
</style>
