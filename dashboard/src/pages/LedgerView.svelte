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
  <h2>Ledger View</h2>
  <p class="subtitle">
    Canonical Decision Ledger state. Decisions enter only through governed
    candidate promotion. Demotion actions emit review commands.
  </p>

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
      <div class="ledger-list">
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
      </div>

      {#if selectedId}
        {@const sel = selectedItem()}
        {#if sel}
          <div class="detail-pane">
            {#if isDecision(sel)}
              <h3>{sel.summary}</h3>
              <div class="detail-meta">
                <span>ID: <code>{sel.id}</code></span>
                <span>Feature: {sel.feature}</span>
                {#if sel.parent_id}<span>Parent: <code>{sel.parent_id}</code></span>{/if}
                <div class="axes">
                  <span>Signoff: <span class="badge {signoffClass(sel.signoff)}">{sel.signoff.replace("_", " ")}</span></span>
                  <span>Compliance: <span class="badge {complianceClass(sel.compliance)}">{sel.compliance}</span></span>
                </div>
                {#if sel.discovered}
                  <span class="discovered-tag">agent-discovered</span>
                {/if}
                {#if sel.conflicts_with && sel.conflicts_with.length > 0}
                  <div class="conflicts">
                    Conflicts with: {sel.conflicts_with.join(", ")}
                  </div>
                {/if}
              </div>

              {#if sel.sources.length > 0}
                <div class="detail-section">
                  <h4>Source evidence</h4>
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
                <div class="detail-section">
                  <h4>Code regions</h4>
                  {#each sel.regions as r}
                    <div class="region">
                      <code>{r.file}:{r.start_line}-{r.end_line}</code>
                    </div>
                  {/each}
                </div>
              {/if}

              <div class="detail-section">
                <h4>Allowed commands</h4>
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
              <!-- LedgerCandidate detail -->
              <h3>{sel.summary}</h3>
              <div class="detail-meta">
                <span>ID: <code>{sel.id}</code></span>
                <span>Review state: <span class="badge badge-review-state">{sel.review_state}</span></span>
                {#if sel.feature_hint}<span>Feature hint: {sel.feature_hint}</span>{/if}
              </div>

              {#if sel.sources.length > 0}
                <div class="detail-section">
                  <h4>Source evidence</h4>
                  {#each sel.sources as ev}
                    <div class="evidence-item">
                      <div class="ev-meta"><code>{ev.source_uri}</code></div>
                      <blockquote>{ev.excerpt}</blockquote>
                    </div>
                  {/each}
                </div>
              {/if}

              <div class="detail-section">
                <h4>Allowed commands</h4>
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

  .ledger-layout { display: flex; gap: 1rem; min-height: 400px; }
  .ledger-list { width: 340px; flex-shrink: 0; display: flex; flex-direction: column; gap: 0.25rem; overflow: auto; }

  .ledger-row {
    display: flex; flex-direction: column; gap: 0.2rem;
    padding: 0.5rem 0.6rem;
    background: #1a1d2e;
    border: 1px solid #2a2d3a;
    border-radius: 6px;
    cursor: pointer;
    text-align: left;
    color: #e0e0e8;
    font-size: 0.8rem;
    transition: border-color 0.15s;
  }
  .ledger-row:hover { border-color: #4a4f6a; }
  .ledger-row.selected { border-color: #a5b4fc; background: #1e2030; }

  .ledger-row.is-proposed { border-left: 3px solid #fbbf24; }
  .ledger-row.is-collision { border-left: 3px solid #c084fc; }

  .row-top { display: flex; gap: 0.3rem; }
  .row-summary { font-weight: 500; }
  .row-feature { color: #6b7280; font-size: 0.72rem; }

  .badge {
    padding: 0.1rem 0.35rem;
    border-radius: 3px;
    font-size: 0.68rem;
    font-weight: 500;
  }

  .badge-proposed { background: #422006; color: #fbbf24; }
  .badge-approved { background: #052e16; color: #4ade80; }
  .badge-rejected { background: #2a0a0a; color: #f87171; }
  .badge-collision { background: #3b0764; color: #c084fc; }
  .badge-superseded { background: #1e1e2e; color: #9ca3af; }

  .badge-reflected { background: #052e16; color: #4ade80; }
  .badge-partial { background: #422006; color: #fbbf24; }
  .badge-drifted { background: #2a0a0a; color: #f87171; }
  .badge-pending { background: #1e293b; color: #94a3b8; }
  .badge-ungrounded { background: #1e1e2e; color: #6b7280; }

  .badge-candidate { background: #1e293b; color: #60a5fa; }
  .badge-review-state { background: #1e293b; color: #94a3b8; }

  .detail-pane { flex: 1; background: #1a1d2e; border: 1px solid #2a2d3a; border-radius: 6px; padding: 1rem; overflow: auto; }
  .detail-pane h3 { margin: 0 0 0.5rem; font-size: 1rem; }
  .detail-meta { display: flex; flex-direction: column; gap: 0.2rem; font-size: 0.78rem; color: #9ca3af; }
  .detail-meta code { color: #a5b4fc; font-size: 0.72rem; }

  .axes { display: flex; gap: 1rem; margin-top: 0.25rem; }

  .discovered-tag {
    display: inline-block;
    padding: 0.1rem 0.35rem;
    background: #1e293b;
    color: #60a5fa;
    border-radius: 3px;
    font-size: 0.68rem;
    width: fit-content;
  }

  .conflicts { color: #c084fc; font-size: 0.75rem; margin-top: 0.25rem; }

  .detail-section { margin-top: 1rem; }
  h4 { margin: 0 0 0.4rem; font-size: 0.85rem; color: #9ca3af; }

  .evidence-item { margin-bottom: 0.5rem; font-size: 0.78rem; }
  .ev-meta { display: flex; gap: 0.5rem; align-items: center; }
  .ev-meta code { color: #a5b4fc; font-size: 0.72rem; }
  .pointer-type { color: #6b7280; font-size: 0.7rem; }
  blockquote { margin: 0.2rem 0 0 0; padding: 0.35rem 0.5rem; border-left: 2px solid #3b3f54; color: #d1d5db; font-size: 0.75rem; }

  .region { font-size: 0.78rem; margin-bottom: 0.2rem; }
  .region code { color: #a5b4fc; }

  .command-bar { display: flex; flex-wrap: wrap; gap: 0.35rem; }
  .btn-command {
    padding: 0.3rem 0.6rem; border: 1px solid #3b3f54; background: #1e2030;
    color: #e0e0e8; border-radius: 4px; font-size: 0.75rem; cursor: pointer;
    transition: background 0.15s;
  }
  .btn-command:hover { background: #2a2d3a; }
</style>
