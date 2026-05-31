<script lang="ts">
  import IngestionGate from "./pages/IngestionGate.svelte";
  import LedgerView from "./pages/LedgerView.svelte";
  import Legend from "./lib/Legend.svelte";

  let activePage = $state<"ingestion" | "ledger">("ingestion");
</script>

<div class="app">
  <header>
    <h1>Bicameral</h1>
    <nav>
      <button
        class:active={activePage === "ingestion"}
        onclick={() => (activePage = "ingestion")}
      >
        Ingestion Gate
      </button>
      <button
        class:active={activePage === "ledger"}
        onclick={() => (activePage = "ledger")}
      >
        Ledger View
      </button>
    </nav>
    <Legend />
  </header>

  <main>
    {#if activePage === "ingestion"}
      <IngestionGate />
    {:else}
      <LedgerView />
    {/if}
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family:
      -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: #0f1117;
    color: #e0e0e8;
  }

  .app {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
  }

  header {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 1rem 0;
    border-bottom: 1px solid #2a2d3a;
  }

  h1 {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
    color: #a5b4fc;
  }

  nav {
    display: flex;
    gap: 0.25rem;
  }

  nav button {
    background: none;
    border: 1px solid transparent;
    border-radius: 6px;
    padding: 0.4rem 0.75rem;
    color: #9ca3af;
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.15s;
  }

  nav button:hover {
    color: #e0e0e8;
    background: #1e2030;
  }

  nav button.active {
    color: #a5b4fc;
    background: #1e2030;
    border-color: #3b3f54;
  }

  main {
    padding: 1.5rem 0;
  }
</style>
