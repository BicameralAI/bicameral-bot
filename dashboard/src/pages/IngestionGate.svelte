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
      case "fresh": return "#4ade80";
      case "stale": return "#fbbf24";
      case "offline": return "#f87171";
      default: return "#6b7280";
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
  <h2>Ingestion Gate</h2>
  <p class="subtitle">
    Source-derived candidates awaiting governed promotion. Decisions are created
    only through batch ingestion — not manual entry.
  </p>

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
      <div class="source-list">
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
      </div>

      {#if selectedIdx !== null && items[selectedIdx]}
        {@const sel = items[selectedIdx]}
        <div class="detail-pane">
          <div class="detail-header">
            <h3>{sel.source_title}</h3>
            <div class="snapshot-info">
              <span>URI: <code>{sel.source.uri}</code></span>
              <span>Snapshot: <code>{sel.snapshot.snapshot_addr}</code></span>
              <span>Ref: <code>{sel.snapshot.snapshot_ref}</code></span>
              <span>Captured: {sel.snapshot.captured_at}</span>
            </div>
          </div>

          {#if sel.evidence.length > 0}
            <div class="evidence-section">
              <h4>Source evidence</h4>
              {#each sel.evidence as ev}
                <div class="evidence-item">
                  <span class="pointer-type">{ev.pointer_type}</span>
                  <code class="pointer-val">{ev.pointer_value}</code>
                  <blockquote>{ev.excerpt}</blockquote>
                </div>
              {/each}
            </div>
          {/if}

          <div class="candidates-section">
            <h4>Candidates</h4>
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
                    <span class="badge collision">conflict hint</span>
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
  h2 { margin: 0 0 0.25rem; font-size: 1.1rem; }
  .subtitle { color: #6b7280; font-size: 0.8rem; margin: 0 0 1rem; }

  .empty {
    text-align: center;
    padding: 3rem 1rem;
    color: #6b7280;
  }
  .empty-icon { font-size: 2rem; margin-bottom: 0.5rem; }
  .empty-title { font-size: 1rem; font-weight: 600; color: #9ca3af; }
  .empty-body { font-size: 0.8rem; max-width: 440px; margin: 0.5rem auto 0; line-height: 1.5; }

  .error { color: #f87171; padding: 1rem; background: #2a0a0a; border-radius: 6px; }

  .gate-layout { display: flex; gap: 1rem; min-height: 400px; }
  .source-list { width: 280px; flex-shrink: 0; display: flex; flex-direction: column; gap: 0.35rem; }

  .source-card {
    display: flex; flex-direction: column; gap: 0.15rem;
    padding: 0.6rem 0.75rem;
    background: #1a1d2e;
    border: 1px solid #2a2d3a;
    border-radius: 6px;
    cursor: pointer;
    text-align: left;
    color: #e0e0e8;
    font-size: 0.8rem;
    transition: border-color 0.15s;
  }
  .source-card:hover { border-color: #4a4f6a; }
  .source-card.selected { border-color: #a5b4fc; background: #1e2030; }

  .source-type { color: #6b7280; font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.04em; }
  .source-title { font-weight: 500; }
  .freshness { font-size: 0.7rem; }
  .candidate-count { color: #9ca3af; font-size: 0.7rem; }

  .detail-pane { flex: 1; background: #1a1d2e; border: 1px solid #2a2d3a; border-radius: 6px; padding: 1rem; overflow: auto; }
  .detail-header h3 { margin: 0 0 0.5rem; font-size: 1rem; }
  .snapshot-info { display: flex; flex-direction: column; gap: 0.15rem; font-size: 0.75rem; color: #9ca3af; }
  .snapshot-info code { color: #a5b4fc; font-size: 0.72rem; }

  .evidence-section, .candidates-section { margin-top: 1rem; }
  h4 { margin: 0 0 0.5rem; font-size: 0.85rem; color: #9ca3af; }

  .evidence-item { margin-bottom: 0.5rem; font-size: 0.8rem; }
  .pointer-type { color: #6b7280; font-size: 0.7rem; margin-right: 0.25rem; }
  .pointer-val { font-size: 0.72rem; color: #a5b4fc; }
  blockquote { margin: 0.25rem 0 0 0; padding: 0.4rem 0.6rem; border-left: 2px solid #3b3f54; color: #d1d5db; font-size: 0.78rem; }

  .candidate-row {
    display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid #2a2d3a;
    font-size: 0.8rem;
  }
  .candidate-row.rejected { opacity: 0.5; text-decoration: line-through; }
  .candidate-summary { flex: 1; min-width: 200px; }
  .feature-hint { color: #6b7280; font-size: 0.72rem; }

  .badge {
    padding: 0.1rem 0.35rem;
    border-radius: 3px;
    font-size: 0.68rem;
    font-weight: 500;
  }
  .collision { background: #3b0764; color: #c084fc; }
  .review-state { background: #1e293b; color: #94a3b8; }

  .btn-reject {
    padding: 0.2rem 0.5rem; border: 1px solid #7f1d1d; background: #2a0a0a;
    color: #f87171; border-radius: 4px; font-size: 0.72rem; cursor: pointer;
  }
  .btn-reject:hover { background: #450a0a; }

  .ingest-action { margin-top: 0.75rem; display: flex; align-items: center; gap: 0.75rem; }
  .btn-ingest {
    padding: 0.4rem 1rem; border: 1px solid #166534; background: #052e16;
    color: #4ade80; border-radius: 6px; font-size: 0.8rem; font-weight: 500; cursor: pointer;
  }
  .btn-ingest:hover { background: #064e3b; }
  .ingest-hint { color: #6b7280; font-size: 0.72rem; }

  .empty-inline { color: #6b7280; font-size: 0.8rem; }
</style>
