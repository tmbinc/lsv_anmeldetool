<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { getResults, type Group, type ResultEntry } from "../../../../api/api";
  import PairingAlert from "../PairingAlert.svelte";
  import { page } from "$app/state";
  import { browser } from "$app/environment";

  let loading = $state(true);
  let failed_load = $state(false);
  let event_name = $state("");
  const event_id = page.params.id || "";
  let groups: Group[] = $state([]);
  let results: ResultEntry[] = $state([]);
  let evtSource: EventSource | null = null;
  let pairingSignal = $state(0);

  let org_selected = $state(browser ? (localStorage.getItem("org_selected") ?? "*") : "*");
  let orgs: string[] = $state([]);

  function saveOrg() {
    if (browser) localStorage.setItem("org_selected", org_selected);
  }

  // ── Matrix easter egg ──────────────────────────────────────────────────────
  let matrix = $state(false);
  let canvasEl = $state<HTMLCanvasElement | null>(null);

  // Add/remove matrix class on <html> for scrollbar theming
  $effect(() => {
    if (!browser) return;
    if (matrix) document.documentElement.classList.add("mx-mode");
    else         document.documentElement.classList.remove("mx-mode");
    return () => document.documentElement.classList.remove("mx-mode");
  });

  // Digital rain animation
  $effect(() => {
    if (!matrix || !canvasEl) return;
    const canvas = canvasEl;
    const ctx = canvas.getContext("2d")!;

    const CHARS =
      "アァカサタナハマヤャラワガザダバパイィキシチニヒミリヰギジヂビピウゥクスツヌフムユュルグズブヅプエェケセテネヘメレヱゲゼデベペオォコソトノホモヨョロヲゴゾドボポヴッン" +
      "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ" +
      "<>[]{}=+-*/\\|;:";
    const chars = CHARS.split("");
    const FONT_SIZE = 14;

    let cols: number;
    let drops: number[];

    function resize() {
      canvas.width  = window.innerWidth;
      canvas.height = window.innerHeight;
      cols  = Math.floor(canvas.width / FONT_SIZE);
      // Preserve existing drops, fill new columns with random negative offsets
      const old = drops ?? [];
      drops = Array.from({ length: cols }, (_, i) =>
        old[i] ?? Math.floor(Math.random() * -50)
      );
    }
    resize();
    window.addEventListener("resize", resize);

    ctx.font = `${FONT_SIZE}px monospace`;

    let animId: number;
    function draw() {
      // Fade trail
      ctx.fillStyle = "rgba(0,0,0,0.05)";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      ctx.font = `${FONT_SIZE}px monospace`;
      for (let i = 0; i < drops.length; i++) {
        const y = drops[i] * FONT_SIZE;
        if (y < 0) { drops[i]++; continue; }

        const ch = chars[Math.floor(Math.random() * chars.length)];
        const r  = Math.random();
        // bright white tip → bright green → medium green trail
        ctx.fillStyle = r > 0.97 ? "#ffffff" : r > 0.85 ? "#ccffcc" : "#00ff41";
        ctx.fillText(ch, i * FONT_SIZE, y);

        if (y > canvas.height && Math.random() > 0.975) drops[i] = 0;
        drops[i] += 0.5;
      }
      animId = requestAnimationFrame(draw);
    }
    draw();

    return () => {
      cancelAnimationFrame(animId);
      window.removeEventListener("resize", resize);
    };
  });

  // ── Theme helpers ──────────────────────────────────────────────────────────
  // A compact derived theme-token object keeps the template readable.
  const t = $derived(matrix ? {
    page:          "bg-transparent text-green-400 font-mono",
    header:        "bg-black/90 border-green-900 backdrop-blur-sm",
    title:         "text-green-400 font-mono tracking-widest matrix-glitch",
    select:        "w-full rounded border border-green-800 bg-black/80 px-2 py-1.5 text-sm text-green-400 font-mono focus:outline-none focus:ring-1 focus:ring-green-500",
    pill:          "flex shrink-0 items-center gap-1.5 whitespace-nowrap rounded border border-green-800 bg-black/70 px-3 py-1 text-xs font-mono text-green-400 transition-all hover:bg-green-950 hover:border-green-600",
    pillDot:       "h-2 w-2 shrink-0 rounded-full bg-green-500",
    pillMerged:    "text-green-700",
    card:          "bg-black/80 border-green-900 backdrop-blur-sm",
    groupHdr:      "bg-green-950 border-b border-green-800",
    groupHdrName:  "font-bold font-mono text-green-300",
    groupHdrRound: "text-sm font-mono text-green-600",
    colLabel:      "bg-black/60 border-green-900 text-green-800 font-mono",
    rowNormal:     "",
    rowAlt:        "bg-green-950/20",
    rowHL:         "bg-green-900/50",
    rowDim:        "opacity-30",
    rank:          "text-green-600 font-mono",
    rankHL:        "text-green-300 font-mono",
    team:          "text-green-400 font-mono",
    teamHL:        "text-green-200 font-mono",
    org:           "text-green-800 font-mono",
    orgHL:         "text-green-500 font-mono",
    pts:           "text-green-500 font-mono",
    ptsHL:         "text-green-300 font-mono",
    tie:           "text-green-800 font-mono",
    tieHL:         "text-green-600 font-mono",
    rowBorder:     "#00ff41",
  } : {
    page:          "bg-gray-50",
    header:        "bg-white border-gray-200",
    title:         "text-gray-900",
    select:        "w-full rounded-lg border border-gray-200 bg-white px-2 py-1.5 text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-400",
    pill:          "flex shrink-0 items-center gap-1.5 whitespace-nowrap rounded-full border border-gray-200 bg-white px-3 py-1 text-xs font-semibold text-gray-700 shadow-sm transition-all hover:bg-gray-50 active:bg-gray-100",
    pillDot:       "h-2 w-2 shrink-0 rounded-full",
    pillMerged:    "text-gray-400",
    card:          "bg-white border-gray-200",
    groupHdr:      "",   // uses group.color inline style
    groupHdrName:  "font-bold text-gray-900",
    groupHdrRound: "text-sm font-medium text-gray-800 opacity-70",
    colLabel:      "bg-gray-50 border-gray-100 text-gray-400",
    rowNormal:     "",
    rowAlt:        "bg-gray-50/60",
    rowHL:         "bg-amber-50",
    rowDim:        "opacity-40",
    rank:          "text-gray-400",
    rankHL:        "text-gray-700",
    team:          "text-gray-900",
    teamHL:        "text-gray-900",
    org:           "text-gray-400",
    orgHL:         "text-amber-700",
    pts:           "text-gray-700",
    ptsHL:         "text-gray-800",
    tie:           "text-gray-400",
    tieHL:         "text-gray-500",
    rowBorder:     "",   // uses group.color
  });

  // ── Data ───────────────────────────────────────────────────────────────────
  onMount(async () => {
    const results_request = getResults({ event: event_id });

    results_request.resp.subscribe((resp) => {
      if (resp?.ok) {
        results = resp.data.results.sort((a, b) => (a.rank ?? 0) - (b.rank ?? 0));
        groups  = resp.data.groups;
        event_name = resp.data.event_name;

        const seen = new Set<string>();
        for (const r of results) if (r.team_org) seen.add(r.team_org);
        orgs = [...seen].sort((a, b) => a.localeCompare(b));

        loading = false;
      } else if (resp) {
        failed_load = true;
        loading = false;
      }
    });

    evtSource = new EventSource("/api/v1/event/" + event_id + "/sse");
    evtSource.onmessage = (ev) => {
      const data = JSON.parse(ev.data);
      if (data.kind == "results") results_request.reload();
      if (data.kind == "pairing") pairingSignal++;
    };
  });

  onDestroy(() => evtSource?.close());

  function roundForGroup(g: Group): number | null {
    return results.find((r) => r.group == g.id)?.round ?? null;
  }

  function fmt(v: number | null | undefined): string {
    if (v == null) return "–";
    return Number.isInteger(v) ? String(v) : v.toFixed(1);
  }

  const visibleGroups = $derived(groups.filter((g) => !g.replacement));
</script>

<PairingAlert {event_id} signal={pairingSignal} />

<!-- ── Matrix rain canvas (below everything) ──────────────────────────── -->
{#if matrix}
  <canvas
    bind:this={canvasEl}
    class="pointer-events-none fixed inset-0 z-0"
  ></canvas>
  <!-- Scanline overlay -->
  <div class="pointer-events-none fixed inset-0 z-10 scanlines"></div>
{/if}

<!-- ── Page ───────────────────────────────────────────────────────────── -->
<div class="relative z-20 min-h-screen {t.page}">

  <!-- Sticky header -->
  <header class="sticky top-0 z-30 border-b shadow-sm {t.header}">
    <!-- Nav row -->
    <div class="flex items-center gap-3 px-4 pt-2 pb-1 text-xs {matrix ? 'text-green-700 font-mono' : 'text-gray-400'}">
      <a href="/event/{event_id}/timetable"
         class="transition-colors {matrix ? 'hover:text-green-400' : 'hover:text-gray-600'}">
        {matrix ? "< ZEITPLAN" : "← Zeitplan"}
      </a>
      <span class="opacity-40">|</span>
      <a href="/event/{event_id}/pairings"
         class="transition-colors {matrix ? 'hover:text-green-400' : 'hover:text-gray-600'}">
        {matrix ? "PAARUNGEN >" : "Paarungen →"}
      </a>
    </div>

    <!-- Title -->
    <div class="px-4 pt-1 pb-2">
      <h1 class="text-lg font-bold {t.title}">
        {matrix ? "> " : ""}{event_name || "Ergebnisse"}{matrix ? "_" : ""}
      </h1>
    </div>

    <!-- Org selector -->
    {#if orgs.length > 0}
      <div class="px-4 pb-2">
        <select bind:value={org_selected} onchange={saveOrg} class={t.select}>
          <option value="*">{matrix ? "[ ALLE SCHULEN ]" : "Alle Schulen"}</option>
          {#each orgs as org}
            <option value={org}>{matrix ? "> " : ""}{org}</option>
          {/each}
        </select>
      </div>
    {/if}

    <!-- Group jump pills -->
    {#if visibleGroups.length > 1}
      <div class="flex flex-wrap gap-2 px-4 pb-2.5">
        {#each visibleGroups as group (group.id)}
          {@const merged = groups.filter((g) => g.replacement === group.id)}
          <button
            onclick={() => document.getElementById(`group-${group.id}`)?.scrollIntoView({ behavior: "smooth", block: "start" })}
            class={t.pill}
          >
            <span
              class={t.pillDot}
              style={matrix ? "" : `background-color: ${group.color};`}
            ></span>
            {group.name}{#if merged.length > 0}<span class="ml-0.5 font-normal {t.pillMerged}">({merged.map((g) => g.name).join(", ")})</span>{/if}
          </button>
        {/each}
      </div>
    {/if}
  </header>

  <!-- Content -->
  <main class="mx-auto max-w-2xl px-4 py-6 space-y-5">

    {#if loading}
      <div class="flex items-center justify-center py-24 text-sm {matrix ? 'text-green-700 font-mono' : 'text-gray-400'}">
        {matrix ? "> LOADING MATRIX DATA..." : "Lade…"}
      </div>

    {:else if failed_load}
      <div class="rounded-xl border px-4 py-3 text-sm {matrix ? 'border-red-900 bg-black/80 text-red-500 font-mono' : 'border-red-200 bg-red-50 text-red-600'}">
        {matrix ? "> ERROR: COULD NOT LOAD DATA" : "Ergebnisse konnten nicht geladen werden."}
      </div>

    {:else if visibleGroups.length === 0}
      <div class="flex items-center justify-center py-24 text-sm {matrix ? 'text-green-900 font-mono' : 'text-gray-400'}">
        {matrix ? "> NO DATA FOUND" : "Noch keine Ergebnisse verfügbar."}
      </div>

    {:else}
      {#each visibleGroups as group (group.id)}
        {@const groupResults = results.filter((r) => r.group === group.id)}
        {@const round = roundForGroup(group)}

        <div
          id="group-{group.id}"
          class="scroll-mt-44 overflow-hidden rounded-xl border shadow-sm {t.card}"
          class:matrix-card={matrix}
        >
          <!-- Group header -->
          <div
            class="flex items-baseline justify-between px-4 py-3 {t.groupHdr}"
            style={matrix ? "" : `background-color: ${group.color};`}
          >
            <span class={t.groupHdrName}>
              {matrix ? `// ${group.name}` : group.name}
            </span>
            {#if round}
              <span class={t.groupHdrRound}>
                {matrix ? `RND_${round}` : `Stand: Runde ${round}`}
              </span>
            {:else}
              <span class="{t.groupHdrRound} opacity-50">
                {matrix ? "NO_DATA" : "Keine Ergebnisse"}
              </span>
            {/if}
          </div>

          {#if groupResults.length === 0}
            <p class="px-4 py-4 text-sm {matrix ? 'text-green-900 font-mono' : 'text-gray-400'}">
              {matrix ? "> NULL" : "Noch keine Ergebnisse."}
            </p>
          {:else}

            <!-- Column labels -->
            <div
              class="pts-grid grid border-b px-2 py-1.5 text-xs font-semibold uppercase {t.colLabel}"
            >
              <div class="text-center">#</div>
              <div class="px-1">{matrix ? "TEAM_ID" : "Mannschaft"}</div>
              <div class="text-right">{matrix ? "M" : "M"}</div>
              <div class="text-right">{matrix ? "B" : "B"}</div>
              <div class="text-right">{matrix ? "Bh" : "Bh"}</div>
            </div>

            {#each groupResults as result, i}
              {@const hl  = org_selected !== "*" && result.team_org === org_selected}
              {@const dim = org_selected !== "*" && result.team_org !== org_selected}
              <div
                class="pts-grid grid items-center border-b px-2 py-2 last:border-0
                  {hl ? t.rowHL : dim ? t.rowNormal : i % 2 === 1 ? t.rowAlt : t.rowNormal}
                  {dim ? t.rowDim : ''}"
                style="border-left: 3px solid {matrix ? (hl ? '#00ff41' : '#003300') : group.color};
                       border-bottom-color: {matrix ? '#0a2a0a' : ''};"
              >
                <!-- Rank -->
                <div class="text-center tabular-nums text-sm font-black {hl ? t.rankHL : t.rank}">
                  {result.rank ?? "–"}
                </div>

                <!-- Team + org -->
                <div class="min-w-0 px-1">
                  <div class="truncate leading-tight font-semibold {hl ? t.teamHL : t.team}">
                    {result.team ?? "–"}
                  </div>
                  {#if result.team_org}
                    <div class="truncate text-xs leading-tight {hl ? t.orgHL : t.org}">
                      {result.team_org}
                    </div>
                  {/if}
                </div>

                <!-- Points -->
                <div class="text-right tabular-nums text-sm font-semibold {hl ? t.ptsHL : t.pts}">
                  {fmt(result.points_team)}
                </div>
                <div class="text-right tabular-nums text-sm {hl ? t.ptsHL : t.pts}">
                  {fmt(result.points_player)}
                </div>
                <div class="text-right tabular-nums text-sm {hl ? t.tieHL : t.tie}">
                  {fmt(result.tie)}
                </div>
              </div>
            {/each}

          {/if}
        </div>
      {/each}
    {/if}

  </main>
</div>

<!-- ── π button (barely visible easter egg trigger) ───────────────────── -->
<button
  onclick={() => (matrix = !matrix)}
  class="fixed bottom-4 left-4 z-50 select-none rounded-full px-2 py-1 text-xs transition-all duration-300
    {matrix
      ? 'border border-green-700 bg-black/80 font-mono text-green-500 opacity-70 shadow-[0_0_8px_#00ff41] hover:opacity-100'
      : 'text-gray-300 opacity-10 hover:opacity-30'}"
  title="π"
>π</button>

<style>
  /* Responsive points grid: rank | name | M | B | Bh */
  .pts-grid {
    grid-template-columns: 1.75rem 1fr 2rem 2rem 2rem;
  }
  @media (min-width: 360px) {
    .pts-grid {
      grid-template-columns: 2rem 1fr 2.5rem 2.5rem 2.5rem;
    }
  }
  @media (min-width: 480px) {
    .pts-grid {
      grid-template-columns: 2.25rem 1fr 3rem 3rem 3rem;
    }
  }

  /* Scanline CRT effect */
  .scanlines {
    background: repeating-linear-gradient(
      to bottom,
      transparent 0px,
      transparent 2px,
      rgba(0, 0, 0, 0.15) 2px,
      rgba(0, 0, 0, 0.15) 4px
    );
  }

  /* Title glitch animation in matrix mode */
  :global(.matrix-glitch) {
    animation: glitch 4s infinite;
  }
  @keyframes glitch {
    0%, 88%, 100% { transform: none; opacity: 1; text-shadow: 0 0 8px #00ff41; }
    89%  { transform: translateX(3px) skewX(-1deg); opacity: 0.85; }
    90%  { transform: translateX(-3px) skewX(1deg); opacity: 0.9; }
    91%  { transform: translateX(2px); opacity: 0.85; }
    92%  { transform: none; opacity: 1; }
  }

  /* Card glow in matrix mode */
  :global(.matrix-card) {
    box-shadow: 0 0 12px rgba(0, 255, 65, 0.08), inset 0 0 30px rgba(0, 255, 65, 0.02);
  }

  /* Green scrollbar when matrix mode is active */
  :global(.mx-mode) {
    scrollbar-color: #00ff41 #000000;
  }
  :global(.mx-mode::-webkit-scrollbar) {
    width: 8px;
    background: #000;
  }
  :global(.mx-mode::-webkit-scrollbar-thumb) {
    background: #003a00;
    border-radius: 4px;
  }
  :global(.mx-mode::-webkit-scrollbar-thumb:hover) {
    background: #00ff41;
  }
</style>
