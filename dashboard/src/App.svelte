<script lang="ts">
  import IngestionGate from "./pages/IngestionGate.svelte";
  import LedgerView from "./pages/LedgerView.svelte";
  import Legend from "./lib/Legend.svelte";

  let activePage = $state<"ingestion" | "ledger">("ingestion");
</script>

<div class="app-shell">
  <header class="top-bar">
    <span class="brand">BICAMERAL</span>
    <Legend />
  </header>

  <div class="layout">
    <nav class="sidebar">
      <button
        class:active={activePage === "ingestion"}
        onclick={() => (activePage = "ingestion")}
      >
        <span class="nav-icon">◊</span>
        <span>Ingestion Gate</span>
      </button>
      <button
        class:active={activePage === "ledger"}
        onclick={() => (activePage = "ledger")}
      >
        <span class="nav-icon">☰</span>
        <span>Ledger View</span>
      </button>
    </nav>

    <main>
      {#if activePage === "ingestion"}
        <IngestionGate />
      {:else}
        <LedgerView />
      {/if}
    </main>
  </div>
</div>

<style>
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI",
      Roboto, sans-serif;
    background: #fafafa;
    color: #1a1a2e;
    -webkit-font-smoothing: antialiased;
  }

  :global(code) {
    font-family: "JetBrains Mono", monospace;
    font-size: 0.85em;
  }

  .app-shell {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .top-bar {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 0.65rem 1.25rem;
    background: #fff;
    border-bottom: 1px solid #e5e5e5;
  }

  .brand {
    font-size: 0.85rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: #1a1a2e;
  }

  .layout {
    display: flex;
    flex: 1;
  }

  .sidebar {
    width: 180px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 0.75rem 0.5rem;
    background: #fff;
    border-right: 1px solid #e5e5e5;
  }

  .sidebar button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 6px;
    background: none;
    color: #6b7280;
    font-size: 0.82rem;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.12s;
    text-align: left;
  }

  .sidebar button:hover {
    background: #f3f4f6;
    color: #1a1a2e;
  }

  .sidebar button.active {
    background: #eef2ff;
    color: #4338ca;
    font-weight: 500;
  }

  .nav-icon {
    font-size: 1rem;
    width: 1.2rem;
    text-align: center;
    flex-shrink: 0;
  }

  main {
    flex: 1;
    padding: 1.5rem 2rem;
    overflow: auto;
  }
</style>
