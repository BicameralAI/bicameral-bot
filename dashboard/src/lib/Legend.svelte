<script lang="ts">
  /** Global legend for signoff/compliance state badges. */
</script>

<div class="legend">
  <span class="legend-group">
    <span class="legend-label">Signoff</span>
    <span class="badge proposed">proposed</span>
    <span class="badge approved">approved</span>
    <span class="badge rejected">rejected</span>
    <span class="badge collision">collision pending</span>
    <span class="badge superseded">superseded</span>
  </span>
  <span class="sep"></span>
  <span class="legend-group">
    <span class="legend-label">Compliance</span>
    <span class="badge reflected">reflected</span>
    <span class="badge partial">partial</span>
    <span class="badge drifted">drifted</span>
    <span class="badge pending">pending</span>
    <span class="badge ungrounded">ungrounded</span>
  </span>
</div>

<style>
  .legend {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.68rem;
    margin-left: auto;
    flex-wrap: wrap;
  }

  .legend-group {
    display: flex;
    align-items: center;
    gap: 0.3rem;
  }

  .legend-label {
    color: #9ca3af;
    font-weight: 500;
    margin-right: 0.15rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    font-size: 0.62rem;
  }

  .sep {
    width: 1px;
    height: 16px;
    background: #d1d5db;
  }

  .badge {
    padding: 0.12rem 0.4rem;
    border-radius: 4px;
    font-weight: 500;
    white-space: nowrap;
    border: 1px solid;
  }

  .proposed { background: #fef3c7; color: #92400e; border-color: #fcd34d; }
  .approved { background: #d1fae5; color: #065f46; border-color: #6ee7b7; }
  .rejected { background: #fee2e2; color: #991b1b; border-color: #fca5a5; }
  .collision { background: #f3e8ff; color: #6b21a8; border-color: #d8b4fe; }
  .superseded { background: #f3f4f6; color: #6b7280; border-color: #d1d5db; }

  .reflected { background: #d1fae5; color: #065f46; border-color: #6ee7b7; }
  .partial { background: #fef3c7; color: #92400e; border-color: #fcd34d; }
  .drifted { background: #fee2e2; color: #991b1b; border-color: #fca5a5; }
  .pending { background: #e0e7ff; color: #3730a3; border-color: #a5b4fc; }
  .ungrounded { background: #f3f4f6; color: #6b7280; border-color: #d1d5db; }
</style>
