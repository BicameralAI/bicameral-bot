<script lang="ts">
  /**
   * Ingestion Gate — source/candidate intake page.
   *
   * Shows existing DecisionCandidates for selected source snapshots.
   * Does NOT provide a manual Decision creation form.
   * Ingest batch-promotes remaining candidates for a selected snapshot.
   * Reject emits durable reject_candidate review events.
   */
  import { fetchIngestionGate, submitReviewCommand } from "../lib/api";
  import type { IngestionGateItem, DecisionCandidatePreview } from "../lib/types";

  let items = $state<IngestionGateItem[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let selectedIdx = $state<number | null>(null);

  async function load() {
    loading = true;
    error = null;
    try {
      items = await fetchIngestionGate();
    } catch (e: any) {
      error = e.message;
      items = [];
    } finally {
      loading = false;
    }
  }

  load();

  function freshnessColor(f: string) {
    switch (f) {
      case "fresh": return "#059669";
      case "stale": return "#d97706";
      case "offline": return "#dc2626";
      default: return "#9ca3af";
    }
  }

  async function handleIngest(item: IngestionGateItem) {
    const remaining = item.candidates.filter(c => c.review_state !== "rejected");
    if (remaining.length === 0) return;
    try {
      await submitReviewCommand({
        target_id: item.snapshot.snapshot_addr,
        command: "ingest_batch",
        reason: `Batch promote ${remaining.length} candidate(s) from ${item.source.uri}`,
      });
      await load();
    } catch (e: any) {
      error = e.message;
    }
  }

  async function handleReject(candidate: DecisionCandidatePreview) {
    if (!candidate.id) return;
    try {
      await submitReviewCommand({
        target_id: candidate.id,
        command: "reject_candidate",
      });
      await load();
    } catch (e: any) {
      error = e.message;
    }
  }
</script>

<section>
  <header class="page-header">
    <h1>Ingestion Gate</h1>
    <p class="subtitle">
      Source-derived candidates awaiting governed promotion. Decisions are created
      only through batch ingestion — not manual entry.
    </p>
  </header>

  {#if loading}
    <div class="empty">Loading sources…</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if items.length === 0}
    <div class="empty">
      <div class="empty-icon">⊘</div>
      <div class="empty-title">No source snapshots</div>
      <div class="empty-body">
        Connect a source or run an extractor to produce DecisionCandidates.
        The Ingestion Gate shows candidates that already exist for a selected
        source snapshot — it is not a manual creation form.
      </div>
    </div>
  {:else}
    <div class="gate-layout">
      <aside class="source-list">
        {#each items as item, idx}
          <button
            class="source-card"
            class:selected={selectedIdx === idx}
            onclick={() => (selectedIdx = idx)}
          >
            <span class="source-type">{item.source.source_type}</span>
            <span class="source-title">{item.source_title}</span>
            <span
              class="freshness"
              style="color: {freshnessColor(item.source_freshness)}"
            >
              {item.source_freshness}
            </span>
            <span class="candidate-count">
              {item.candidates.length} candidate{item.candidates.length !== 1 ? "s" : ""}
            </span>
          </button>
        {/each}
      </aside>

      {#if selectedIdx !== null && items[selectedIdx]}
        {@const sel = items[selectedIdx]}
        <div class="detail-pane">
          <div class="detail-header">
            <h2>{sel.source_title}</h2>
            <dl class="snapshot-meta">
              <dt>URI</dt><dd><code>{sel.source.uri}</code></dd>
              <dt>Snapshot</dt><dd><code>{sel.snapshot.snapshot_addr}</code></dd>
              <dt>Ref</dt><dd><code>{sel.snapshot.snapshot_ref}</code></dd>
              <dt>Captured</dt><dd>{sel.snapshot.captured_at}</dd>
            </dl>
          </div>

          {#if sel.evidence.length > 0}
            <div class="section">
              <h3>Source evidence</h3>
              {#each sel.evidence as ev}
                <div class="evidence-item">
                  <span class="pointer-type">{ev.pointer_type}</span>
                  <code class="pointer-val">{ev.pointer_value}</code>
                  <blockquote>{ev.excerpt}</blockquote>
                </div>
              {/each}
            </div>
          {/if}

          <div class="section">
            <h3>Candidates</h3>
            {#if sel.candidates.length === 0}
              <p class="empty-inline">No candidates extracted for this snapshot.</p>
            {:else}
              {#each sel.candidates as c}
                <div class="candidate-row" class:rejected={c.review_state === "rejected"}>
                  <div class="candidate-summary">{c.summary}</div>
                  {#if c.feature_hint}
                    <span class="feature-hint">{c.feature_hint}</span>
                  {/if}
                  {#if c.conflict_hint}
                    <span class="badge collision">conflict</span>
                  {/if}
                  {#if c.review_state}
                    <span class="badge review-state">{c.review_state}</span>
                  {/if}
                  {#if c.review_state !== "rejected"}
                    <button class="btn-reject" onclick={() => handleReject(c)}>
                      Reject
                    </button>
                  {/if}
                </div>
              {/each}
              <div class="ingest-action">
                <button class="btn-ingest" onclick={() => handleIngest(sel)}>
                  Ingest remaining candidates
                </button>
                <span class="ingest-hint">
                  Batch-promotes remaining candidates through governed review.
                </span>
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  {/if}
</section>

<style>
  section { max-width: 1100px; }

  .page-header { margin-bottom: 1.25rem; }
  h1 { font-size: 1.35rem; font-weight: 600; margin: 0 0 0.25rem; color: #1a1a2e; }
  .subtitle { color: #6b7280; font-size: 0.8rem; margin: 0; }

  .empty {
    text-align: center;
    padding: 3rem 1rem;
    color: #9ca3af;
  }
  .empty-icon { font-size: 2rem; margin-bottom: 0.5rem; }
  .empty-title { font-size: 1rem; font-weight: 600; color: #6b7280; }
  .empty-body { font-size: 0.82rem; max-width: 440px; margin: 0.5rem auto 0; line-height: 1.5; }

  .error { color: #dc2626; padding: 1rem; background: #fef2f2; border: 1px solid #fecaca; border-radius: 8px; }

  .gate-layout { display: flex; gap: 1.25rem; min-height: 400px; }

  .source-list {
    width: 280px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    overflow: auto;
  }

  .source-card {
    display: flex; flex-direction: column; gap: 0.15rem;
    padding: 0.6rem 0.75rem;
    background: #fff;
    border: 1px solid #e5e5e5;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    color: #1a1a2e;
    font-size: 0.82rem;
    font-family: inherit;
    transition: all 0.12s;
  }
  .source-card:hover { border-color: #a5b4fc; background: #fafafe; }
  .source-card.selected { border-color: #4338ca; background: #eef2ff; }

  .source-type { color: #9ca3af; font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.04em; }
  .source-title { font-weight: 500; }
  .freshness { font-size: 0.72rem; }
  .candidate-count { color: #6b7280; font-size: 0.72rem; }

  .detail-pane {
    flex: 1;
    background: #fff;
    border: 1px solid #e5e5e5;
    border-radius: 8px;
    padding: 1.25rem;
    overflow: auto;
  }

  .detail-header h2 { margin: 0 0 0.75rem; font-size: 1.1rem; font-weight: 600; color: #1a1a2e; }

  .snapshot-meta {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.15rem 0.75rem;
    font-size: 0.78rem;
    color: #6b7280;
  }
  .snapshot-meta dt { font-weight: 500; color: #9ca3af; }
  .snapshot-meta dd { margin: 0; }
  .snapshot-meta code { color: #4338ca; font-size: 0.75rem; }

  .section { margin-top: 1.25rem; }
  h3 { margin: 0 0 0.5rem; font-size: 0.88rem; font-weight: 600; color: #6b7280; text-transform: uppercase; letter-spacing: 0.03em; }

  .evidence-item { margin-bottom: 0.5rem; font-size: 0.82rem; }
  .pointer-type { color: #9ca3af; font-size: 0.72rem; margin-right: 0.25rem; }
  .pointer-val { font-size: 0.75rem; color: #4338ca; }
  blockquote {
    margin: 0.25rem 0 0 0;
    padding: 0.4rem 0.75rem;
    border-left: 3px solid #e5e5e5;
    color: #374151;
    font-size: 0.82rem;
    font-style: italic;
    background: #f9fafb;
    border-radius: 0 6px 6px 0;
  }

  .candidate-row {
    display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;
    padding: 0.5rem 0.6rem;
    border-bottom: 1px solid #f3f4f6;
    font-size: 0.82rem;
  }
  .candidate-row.rejected { opacity: 0.4; text-decoration: line-through; }
  .candidate-summary { flex: 1; min-width: 200px; color: #1a1a2e; }
  .feature-hint { color: #9ca3af; font-size: 0.72rem; }

  .badge {
    padding: 0.12rem 0.4rem;
    border-radius: 4px;
    font-size: 0.68rem;
    font-weight: 500;
    border: 1px solid;
  }
  .collision { background: #f3e8ff; color: #6b21a8; border-color: #d8b4fe; }
  .review-state { background: #f3f4f6; color: #6b7280; border-color: #d1d5db; }

  .btn-reject {
    padding: 0.2rem 0.5rem;
    border: 1px solid #fca5a5;
    background: #fef2f2;
    color: #dc2626;
    border-radius: 4px;
    font-size: 0.72rem;
    font-family: inherit;
    cursor: pointer;
    transition: background 0.12s;
  }
  .btn-reject:hover { background: #fee2e2; }

  .ingest-action { margin-top: 0.75rem; display: flex; align-items: center; gap: 0.75rem; }
  .btn-ingest {
    padding: 0.4rem 1rem;
    border: 1px solid #6ee7b7;
    background: #d1fae5;
    color: #065f46;
    border-radius: 6px;
    font-size: 0.82rem;
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
    transition: background 0.12s;
  }
  .btn-ingest:hover { background: #a7f3d0; }
  .ingest-hint { color: #9ca3af; font-size: 0.72rem; }

  .empty-inline { color: #9ca3af; font-size: 0.82rem; }
</style>
