<script lang="ts">
  /**
   * Ledger View — canonical decision state and review surface.
   *
   * Shows Decision Ledger state with both signoff and compliance axes.
   * Proposed Decisions are visually distinct (collision/sensitivity pending).
   * Demotion actions emit governed review commands, not direct storage writes.
   * Does NOT expose manual Decision creation.
   */
  import { fetchLedgerView, submitReviewCommand } from "../lib/api";
  import type {
    LedgerReviewItem,
    LedgerDecision,
    LedgerCandidate,
    DecisionCommandKind,
  } from "../lib/types";

  let items = $state<LedgerReviewItem[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let selectedId = $state<string | null>(null);

  async function load() {
    loading = true;
    error = null;
    try {
      items = await fetchLedgerView();
    } catch (e: any) {
      error = e.message;
      items = [];
    } finally {
      loading = false;
    }
  }

  load();

  function isDecision(item: LedgerReviewItem): item is LedgerDecision {
    return item.kind === "decision";
  }

  function signoffClass(s: string) {
    switch (s) {
      case "proposed": return "badge-proposed";
      case "approved": return "badge-approved";
      case "rejected": return "badge-rejected";
      case "collision_pending": return "badge-collision";
      case "superseded": return "badge-superseded";
      default: return "";
    }
  }

  function complianceClass(c: string) {
    switch (c) {
      case "reflected": return "badge-reflected";
      case "partial": return "badge-partial";
      case "drifted": return "badge-drifted";
      case "pending": return "badge-pending";
      case "ungrounded": return "badge-ungrounded";
      default: return "";
    }
  }

  async function handleCommand(targetId: string, command: DecisionCommandKind | "reject_candidate" | "accept_candidate" | "demote_decision") {
    try {
      await submitReviewCommand({ target_id: targetId, command });
      await load();
    } catch (e: any) {
      error = e.message;
    }
  }

  function selectedItem(): LedgerReviewItem | undefined {
    return items.find(i => i.id === selectedId);
  }
</script>

<section>
  <header class="page-header">
    <h1>Ledger View</h1>
    <p class="subtitle">
      Canonical Decision Ledger state. Decisions enter only through governed
      candidate promotion. Demotion actions emit review commands.
    </p>
  </header>

  {#if loading}
    <div class="empty">Loading ledger…</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if items.length === 0}
    <div class="empty">
      <div class="empty-icon">⊘</div>
      <div class="empty-title">Ledger is empty</div>
      <div class="empty-body">
        No Decisions or queued candidates. Use the Ingestion Gate to promote
        source-derived candidates into the Decision Ledger through governed
        review.
      </div>
    </div>
  {:else}
    <div class="ledger-layout">
      <aside class="ledger-list">
        {#each items as item}
          <button
            class="ledger-row"
            class:selected={selectedId === item.id}
            class:is-proposed={isDecision(item) && item.signoff === "proposed"}
            class:is-collision={isDecision(item) && item.signoff === "collision_pending"}
            onclick={() => (selectedId = item.id)}
          >
            <div class="row-top">
              {#if isDecision(item)}
                <span class="badge {signoffClass(item.signoff)}">
                  {item.signoff.replace("_", " ")}
                </span>
                <span class="badge {complianceClass(item.compliance)}">
                  {item.compliance}
                </span>
              {:else}
                <span class="badge badge-candidate">candidate</span>
                <span class="badge badge-review-state">{item.review_state}</span>
              {/if}
            </div>
            <div class="row-summary">{item.summary}</div>
            {#if isDecision(item)}
              <div class="row-feature">{item.feature}</div>
            {:else if item.feature_hint}
              <div class="row-feature">{item.feature_hint}</div>
            {/if}
          </button>
        {/each}
      </aside>

      {#if selectedId}
        {@const sel = selectedItem()}
        {#if sel}
          <div class="detail-pane">
            {#if isDecision(sel)}
              <h2>{sel.summary}</h2>
              <dl class="detail-meta">
                <dt>ID</dt><dd><code>{sel.id}</code></dd>
                <dt>Feature</dt><dd>{sel.feature}</dd>
                {#if sel.parent_id}<dt>Parent</dt><dd><code>{sel.parent_id}</code></dd>{/if}
                <dt>Signoff</dt><dd><span class="badge {signoffClass(sel.signoff)}">{sel.signoff.replace("_", " ")}</span></dd>
                <dt>Compliance</dt><dd><span class="badge {complianceClass(sel.compliance)}">{sel.compliance}</span></dd>
              </dl>
              {#if sel.discovered}
                <span class="discovered-tag">agent-discovered</span>
              {/if}
              {#if sel.conflicts_with && sel.conflicts_with.length > 0}
                <div class="conflicts">
                  <h3>Conflicts</h3>
                  <p>Overlaps with: {sel.conflicts_with.join(", ")}</p>
                </div>
              {/if}

              {#if sel.sources.length > 0}
                <div class="section">
                  <h3>Sources</h3>
                  {#each sel.sources as ev}
                    <div class="evidence-item">
                      <div class="ev-meta">
                        <code>{ev.source_uri}</code>
                        <span class="pointer-type">{ev.pointer_type}: {ev.pointer_value}</span>
                      </div>
                      <blockquote>{ev.excerpt}</blockquote>
                    </div>
                  {/each}
                </div>
              {/if}

              {#if sel.regions && sel.regions.length > 0}
                <div class="section">
                  <h3>Implementation</h3>
                  {#each sel.regions as r}
                    <div class="region">
                      <code>{r.file}:{r.start_line}-{r.end_line}</code>
                    </div>
                  {/each}
                </div>
              {:else}
                <div class="section">
                  <h3>Implementation</h3>
                  <div class="no-regions">no code regions bound yet</div>
                </div>
              {/if}

              <div class="section">
                <h3>Actions</h3>
                <div class="command-bar">
                  {#each sel.allowed_commands as cmd}
                    <button
                      class="btn-command"
                      onclick={() => handleCommand(sel.id, cmd)}
                    >
                      {cmd.replace(/_/g, " ")}
                    </button>
                  {/each}
                </div>
              </div>
            {:else}
              <h2>{sel.summary}</h2>
              <dl class="detail-meta">
                <dt>ID</dt><dd><code>{sel.id}</code></dd>
                <dt>Review state</dt><dd><span class="badge badge-review-state">{sel.review_state}</span></dd>
                {#if sel.feature_hint}<dt>Feature hint</dt><dd>{sel.feature_hint}</dd>{/if}
              </dl>

              {#if sel.sources.length > 0}
                <div class="section">
                  <h3>Sources</h3>
                  {#each sel.sources as ev}
                    <div class="evidence-item">
                      <div class="ev-meta"><code>{ev.source_uri}</code></div>
                      <blockquote>{ev.excerpt}</blockquote>
                    </div>
                  {/each}
                </div>
              {/if}

              <div class="section">
                <h3>Actions</h3>
                <div class="command-bar">
                  {#each sel.allowed_commands as cmd}
                    <button
                      class="btn-command"
                      onclick={() => handleCommand(sel.id, cmd)}
                    >
                      {cmd.replace(/_/g, " ")}
                    </button>
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        {/if}
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

  .ledger-layout { display: flex; gap: 1.25rem; min-height: 400px; }

  .ledger-list {
    width: 340px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    overflow: auto;
  }

  .ledger-row {
    display: flex; flex-direction: column; gap: 0.2rem;
    padding: 0.5rem 0.65rem;
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
  .ledger-row:hover { border-color: #a5b4fc; }
  .ledger-row.selected { border-color: #4338ca; background: #eef2ff; }

  .ledger-row.is-proposed { border-left: 3px solid #f59e0b; }
  .ledger-row.is-collision { border-left: 3px solid #a855f7; }

  .row-top { display: flex; gap: 0.3rem; }
  .row-summary { font-weight: 500; }
  .row-feature { color: #9ca3af; font-size: 0.72rem; }

  .badge {
    padding: 0.12rem 0.4rem;
    border-radius: 4px;
    font-size: 0.68rem;
    font-weight: 500;
    border: 1px solid;
  }

  .badge-proposed { background: #fef3c7; color: #92400e; border-color: #fcd34d; }
  .badge-approved { background: #d1fae5; color: #065f46; border-color: #6ee7b7; }
  .badge-rejected { background: #fee2e2; color: #991b1b; border-color: #fca5a5; }
  .badge-collision { background: #f3e8ff; color: #6b21a8; border-color: #d8b4fe; }
  .badge-superseded { background: #f3f4f6; color: #6b7280; border-color: #d1d5db; }

  .badge-reflected { background: #d1fae5; color: #065f46; border-color: #6ee7b7; }
  .badge-partial { background: #fef3c7; color: #92400e; border-color: #fcd34d; }
  .badge-drifted { background: #fee2e2; color: #991b1b; border-color: #fca5a5; }
  .badge-pending { background: #e0e7ff; color: #3730a3; border-color: #a5b4fc; }
  .badge-ungrounded { background: #f3f4f6; color: #6b7280; border-color: #d1d5db; }

  .badge-candidate { background: #dbeafe; color: #1e40af; border-color: #93c5fd; }
  .badge-review-state { background: #f3f4f6; color: #6b7280; border-color: #d1d5db; }

  .detail-pane {
    flex: 1;
    background: #fff;
    border: 1px solid #e5e5e5;
    border-radius: 8px;
    padding: 1.25rem;
    overflow: auto;
  }
  .detail-pane h2 { margin: 0 0 0.75rem; font-size: 1.15rem; font-weight: 600; color: #1a1a2e; }

  .detail-meta {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.2rem 0.75rem;
    font-size: 0.82rem;
    color: #6b7280;
  }
  .detail-meta dt { font-weight: 500; color: #9ca3af; }
  .detail-meta dd { margin: 0; }
  .detail-meta code { color: #4338ca; font-size: 0.75rem; }

  .discovered-tag {
    display: inline-block;
    margin-top: 0.5rem;
    padding: 0.12rem 0.4rem;
    background: #dbeafe;
    color: #1e40af;
    border: 1px solid #93c5fd;
    border-radius: 4px;
    font-size: 0.68rem;
    font-weight: 500;
  }

  .conflicts {
    margin-top: 1rem;
    padding: 0.75rem 1rem;
    background: #fef3c7;
    border: 1px solid #fcd34d;
    border-radius: 8px;
    color: #92400e;
    font-size: 0.82rem;
  }
  .conflicts h3 { color: #92400e; font-size: 0.82rem; margin-bottom: 0.25rem; text-transform: none; letter-spacing: 0; }
  .conflicts p { margin: 0; }

  .section { margin-top: 1.25rem; }
  h3 { margin: 0 0 0.5rem; font-size: 0.82rem; font-weight: 600; color: #9ca3af; text-transform: uppercase; letter-spacing: 0.03em; }

  .no-regions {
    padding: 0.5rem 0.75rem;
    background: #f9fafb;
    border: 1px solid #e5e5e5;
    border-radius: 6px;
    color: #9ca3af;
    font-size: 0.82rem;
    font-style: italic;
  }

  .evidence-item { margin-bottom: 0.5rem; font-size: 0.82rem; }
  .ev-meta { display: flex; gap: 0.5rem; align-items: center; }
  .ev-meta code { color: #4338ca; font-size: 0.75rem; }
  .pointer-type { color: #9ca3af; font-size: 0.72rem; }
  blockquote {
    margin: 0.2rem 0 0 0;
    padding: 0.4rem 0.75rem;
    border-left: 3px solid #e5e5e5;
    color: #374151;
    font-size: 0.82rem;
    font-style: italic;
    background: #f9fafb;
    border-radius: 0 6px 6px 0;
  }

  .region { font-size: 0.82rem; margin-bottom: 0.2rem; }
  .region code { color: #4338ca; }

  .command-bar { display: flex; flex-wrap: wrap; gap: 0.35rem; }
  .btn-command {
    padding: 0.35rem 0.65rem;
    border: 1px solid #e5e5e5;
    background: #fff;
    color: #1a1a2e;
    border-radius: 6px;
    font-size: 0.78rem;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.12s;
  }
  .btn-command:hover { background: #f3f4f6; border-color: #a5b4fc; }
</style>
