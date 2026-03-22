# ADR 0001: Keep AI evidence and report output inside the local desktop boundary

- Status: Accepted
- Date: 2026-03-21

## Context

IncidentReview now includes a broader AI-assisted review flow than the original scaffold:

- `qir_ai` persists evidence sources, chunks, indexes, and draft artifacts locally.
- `src-tauri` exposes the thin command layer that bridges the desktop UI to the deterministic core and the optional AI features.
- The UI now carries explicit sections for AI guidance, dashboard review, and report-output proof on the public baseline.

These changes affect the architecture boundary because they decide where evidence storage lives, how AI guidance is allowed to operate, and how report-output work is exposed to the desktop app.

## Decision

We will keep the IncidentReview architecture inside a strict local desktop boundary:

1. Deterministic incident ingest, validation, metrics, and reporting remain in the Rust core.
2. Optional AI features remain additive and evidence-backed, with all evidence, chunks, index state, and draft artifacts stored locally.
3. The Tauri layer remains a thin adapter that exposes approved commands to the UI without moving business logic into the frontend.
4. Report-output and AI theme synthesis proof stay part of the local desktop workflow rather than depending on a hosted service.
5. Public repo documentation must continue to describe the system as local-first and evidence-first.

## Consequences

### Positive

- The public baseline matches the operating promise of the product: local-first, audited, and optional-AI rather than cloud-required.
- AI drafting and theme synthesis stay reviewable because the evidence and artifacts live with the workspace.
- The UI, Tauri bridge, and Rust crates remain easier to reason about because the logic boundary is explicit.

### Tradeoffs

- Desktop verification remains important because more of the product value now depends on the local Tauri + Rust + UI path working together.
- Documentation drift becomes more expensive when AI schemas or Tauri commands change, because the generated reference docs must stay current.

## Follow-up

- Keep adding tests when AI evidence storage, theme synthesis, or report-output commands change.
- Regenerate and commit reference docs whenever the Tauri or schema surface changes.
- Revisit this ADR only if IncidentReview introduces a non-local service boundary or moves report generation outside the desktop app.
