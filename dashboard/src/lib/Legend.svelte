<script lang="ts">
  /** Global legend for signoff/compliance state badges. */
</script>

<div class="legend">
  <span class="legend-title">State legend</span>
  <span class="badge proposed">proposed</span>
  <span class="badge approved">approved</span>
  <span class="badge rejected">rejected</span>
  <span class="badge collision">collision pending</span>
  <span class="badge superseded">superseded</span>
  <span class="sep">|</span>
  <span class="badge reflected">reflected</span>
  <span class="badge partial">partial</span>
  <span class="badge drifted">drifted</span>
  <span class="badge pending">pending</span>
  <span class="badge ungrounded">ungrounded</span>
</div>

<style>
  .legend {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.7rem;
    margin-left: auto;
    flex-wrap: wrap;
  }

  .legend-title {
    color: #6b7280;
    margin-right: 0.25rem;
    font-weight: 500;
  }

  .sep {
    color: #3b3f54;
    margin: 0 0.15rem;
  }

  .badge {
    padding: 0.15rem 0.4rem;
    border-radius: 4px;
    font-weight: 500;
    white-space: nowrap;
  }

  .proposed {
    background: #422006;
    color: #fbbf24;
    border: 1px solid #78350f;
  }

  .approved {
    background: #052e16;
    color: #4ade80;
    border: 1px solid #166534;
  }

  .rejected {
    background: #2a0a0a;
    color: #f87171;
    border: 1px solid #7f1d1d;
  }

  .collision {
    background: #3b0764;
    color: #c084fc;
    border: 1px solid #6b21a8;
  }

  .superseded {
    background: #1e1e2e;
    color: #9ca3af;
    border: 1px solid #374151;
  }

  .reflected {
    background: #052e16;
    color: #4ade80;
    border: 1px solid #166534;
  }

  .partial {
    background: #422006;
    color: #fbbf24;
    border: 1px solid #78350f;
  }

  .drifted {
    background: #2a0a0a;
    color: #f87171;
    border: 1px solid #7f1d1d;
  }

  .pending {
    background: #1e293b;
    color: #94a3b8;
    border: 1px solid #334155;
  }

  .ungrounded {
    background: #1e1e2e;
    color: #6b7280;
    border: 1px solid #374151;
  }
</style>
