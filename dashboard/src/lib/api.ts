/** Thin API client for the Bicameral gateway dashboard endpoints. */

import type {
  IngestionGateItem,
  LedgerReviewItem,
  ReviewCommandPayload,
} from "./types";

const BASE = "";

export async function fetchIngestionGate(): Promise<IngestionGateItem[]> {
  const res = await fetch(`${BASE}/api/v1/dashboard/ingestion-gate`);
  if (!res.ok) throw new Error(`ingestion-gate: ${res.status}`);
  return res.json();
}

export async function fetchLedgerView(): Promise<LedgerReviewItem[]> {
  const res = await fetch(`${BASE}/api/v1/dashboard/ledger`);
  if (!res.ok) throw new Error(`ledger: ${res.status}`);
  return res.json();
}

export async function submitReviewCommand(
  payload: ReviewCommandPayload,
): Promise<void> {
  const res = await fetch(`${BASE}/api/v1/dashboard/command`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!res.ok) throw new Error(`command: ${res.status}`);
}
