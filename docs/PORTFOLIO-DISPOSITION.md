# IncidentReview — Portfolio Disposition

**Status:** Active (Tauri 2 + Rust + TypeScript, local-LLM-dependent
optional) — local-first macOS desktop app for engineering teams
running **Quarterly Incident Reviews** on `origin/main`. Imports
Jira incident CSVs + Slack transcripts, computes MTTD/MTTA/MTTR +
severity distributions with deterministic logic, renders ECharts
dashboards, optionally uses **Ollama local AI** (with evidence-
citation requirement — must cite evidence chunks or mark UNKNOWN)
for executive summary / theme synthesis / recommendations. **33rd
signing cluster candidate, but Active state** (no v1.0 release
closeout cadence yet). **Third local-LLM-dependent sub-pattern
member** (after thought-trails + ApplyKit). Heavy CI tooling:
husky + Lighthouse + stylelint + .perf-baselines.

> Disposition uses strict `origin/main` verification.
> **Local-LLM-dependent sub-pattern reaches 3 members.**

---

## Verification posture

Only `origin` (`saagpatel/IncidentReview`). Clean.

`origin/main`:

- Tip: `efcfb5d` Merge pull request #22 from saagpatel/codex/chore/sync-local-changes
- Recent commits are docs / schema-refresh / scaffolding cadence:
  - `efcfb5d` Merge codex/chore/sync-local-changes
  - `83fa3ee` MIT license
  - `c1424e0` codex bootstrap merge
  - Multiple `docs(schemas):` refresh commits (Jira, Sanitized,
    Slack, Workspace, Evidence, Incident, Ai, App, Backup,
    Citation, Dashboard)
  - `78cdb4f` perf baselines + AGENTS.md update
- Earlier substantive feat commits (per prior session triage):
  - `2c2818c feat(review): wire theme and report output flows`
  - `8aed0f7 feat(dashboards): complete incident drill-down UI`
- Repo tree (`origin/main`):
  - `Cargo.toml` + `Cargo.lock` + `.cargo/` (Rust)
  - Tauri 2 (implied from Rust + frontend setup)
  - `.husky/` (git hooks)
  - `.lighthouserc.json` + `.lighthouserc.production.json`
  - `.perf-baselines/` + `.perf-results/`
  - `.stylelintrc.cjs`, `.eslintrc.cjs`
  - `HINSITE.md` (operator artifact — unusual file name)
- **No v1.0.0 version bump** visible
- **No .dmg distribution build deps** commit visible
- Default branch: `main`

---

## Current state in one paragraph

IncidentReview is a Tauri 2 + Rust + TypeScript local-first macOS
desktop app built for **engineering teams running Quarterly
Incident Reviews**. The data flow: import **Jira incident CSVs**
(with mapping UI + saved column mappings) and **Slack transcripts**
(file or paste; structured timeline extraction) → compute
deterministic metrics (per-incident MTTD / IT awareness lag /
MTTA / time-to-mitigation / MTTR; quarter rollups; weighted "pain"
scores) → render ECharts dashboards (quarter-at-a-glance KPI
deltas, timeline + trend, severity/impact distributions, vendor
heat maps, repeat offender analysis). **Optional Ollama local AI**
adds evidence-citation-required narrative drafting (executive
summary + theme synthesis + recommendations) — `127.0.0.1` only,
no external calls. Recent canonical state is operator-side
schema refresh + codex bootstrap merge; substantive features
(theme, report flows, drill-down UI) shipped in earlier merges.
Active because no v1.0 release closeout cadence yet.

---

## Why "Active" — third local-LLM-dependent sub-pattern member

Joins the local-LLM-dependent sub-pattern inside the signing
cluster:

| Member                 | Cluster slot           | Local LLM use                                                                    |
| ---------------------- | ---------------------- | -------------------------------------------------------------------------------- |
| thought-trails (R15.5) | Signing #29 RF         | **Required** — main feature is chain-of-thought visualization                    |
| ApplyKit (R17.3)       | Signing #32 Active     | **Required** — Truth Gate generation uses local LLM                              |
| **IncidentReview**     | **Signing #33 Active** | **Optional** — AI drafting is opt-in; core metrics + dashboards work without LLM |

Distinct sub-pattern variant: **optional local-LLM-dependent**.
The product core (deterministic metrics + dashboards) doesn't
require the LLM; AI drafting is value-add. This is a meaningful
distinction because:

- **Installation friction is lower** (no required Ollama install
  for baseline use)
- **Privacy posture is stronger** (no LLM data flow for users
  who skip AI drafting)
- **Audit trail discipline higher** (AI outputs MUST cite evidence
  chunks or mark UNKNOWN — same Truth-Gate pattern as ApplyKit)

State is Active because:

- No v1.0 version bump visible
- No .dmg distribution build deps
- Recent commits are docs/schema-refresh, not release closeout
- Heavy CI tooling (husky + Lighthouse + stylelint + perf
  baselines) suggests in-development sophistication, not
  ship-readiness

---

## New observation: heavy frontend QA tooling

IncidentReview is the **first portfolio repo with this combination**:

- `.husky/` (git hooks)
- `.lighthouserc.json` + `.lighthouserc.production.json` (web
  performance auditing — unusual for a Tauri desktop app)
- `.stylelintrc.cjs` (CSS linting)
- `.perf-baselines/` + `.perf-results/` (perf regression detection)

This tooling stack signals **operator investment in
production-grade frontend quality** for what is ostensibly a
desktop app. The Lighthouse production config (`.production.json`
suffix) is particularly unusual — Tauri apps don't deploy to a
production URL. May be lint-only or testing-only usage. Worth
verifying during reactivation.

---

## Cluster taxonomy update

| Cluster                     | Count                       | Notes                                                                                   |
| --------------------------- | --------------------------- | --------------------------------------------------------------------------------------- |
| **Signing (Apple desktop)** | **33** (provisional Active) | 30 RF + 1 IRS-local-pending + 3 Active (JobMarketHeatmap, ApplyKit, **IncidentReview**) |

Signing cluster Active state: 3 members. Cluster lattice
established across iOS / PyPI / signing.

---

## Unblock trigger (operator)

1. **Decide v1.0 release closeout cadence** — apply standard
   Tauri 2 v1.0 signature (CSP + baseline tests + version bump +
   Cargo.lock + .dmg deps).
2. **Apple Developer ID + notarization credentials.**
3. **Ollama install onboarding UX** — even for optional use,
   first-run should clearly distinguish "AI features require
   Ollama" from "core metrics work without."
4. **Jira CSV format stability** — Jira's export format has
   changed in past versions; the mapping UI mitigates this but
   verify behavior on the operator's Jira version.
5. **Slack transcript paste UX** — Slack has multiple paste-from
   formats (legacy desktop / Mac app / web); verify timeline
   extraction handles them.
6. **Evidence-citation Truth Gate enforcement** — verify AI
   outputs without proper citations are rejected, not just
   flagged.
7. **HINSITE.md** — unusual filename; inspect what it contains
   before announce.
8. **Frontend QA tooling rationalization** — decide if Lighthouse
   production config makes sense for a desktop app or remove.

Estimated operator time: ~6-8 hours.

---

## Portfolio operating system instructions

| Aspect               | Posture                                                                                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Portfolio status     | `Active (Tauri 2, optional local-LLM)`                                                                                                                 |
| Distribution channel | **DMG via Apple Developer ID** (planned)                                                                                                               |
| Review cadence       | Active — release closeout cadence pending                                                                                                              |
| Resurface conditions | (a) v1.0 release closeout cadence applied, (b) Jira CSV format change, (c) Slack paste format change, (d) Ollama API change, (e) `HINSITE.md` decision |
| Co-batch with        | Signing cluster — **now 33 repos**                                                                                                                     |
| Sub-pattern          | **Optional local-LLM-dependent** (Ollama opt-in; not required for core features)                                                                       |
| Special concern      | **Truth Gate evidence-citation enforcement.** AI outputs must cite evidence chunks or mark UNKNOWN. Same pattern as ApplyKit.                          |
| Special concern      | **Heavy frontend QA tooling rationalization.** Lighthouse production config unusual for Tauri desktop app.                                             |
| Special concern      | **`HINSITE.md` unusual filename** — investigate before announce.                                                                                       |
| Special concern      | **Jira / Slack format drift.** Vendor export formats change; mapping UI mitigates but doesn't eliminate.                                               |

---

## Reactivation procedure

1. Verify branch tracking.
2. Review stash `r18-ir-stash` (`.github/PULL_REQUEST_TEMPLATE.md`
   mod only — minor).
3. Apply Tauri 2 v1.0 release closeout cadence to ship.
4. Test Jira CSV import on operator's actual Jira version.
5. Test Slack transcript paste flow.
6. Test optional Ollama AI drafting with evidence citation.
7. Inspect `HINSITE.md` for context / decision points.
8. Rationalize Lighthouse / husky / stylelint tooling vs
   desktop-app realities.
9. Run `cargo test` + `npm test`.

---

## Last known reference

| Field                  | Value                                                                                                                                                                                                                   |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `origin/main` tip      | `efcfb5d` Merge pull request #22 from saagpatel/codex/chore/sync-local-changes                                                                                                                                          |
| Default branch         | `main`                                                                                                                                                                                                                  |
| Build system           | Tauri 2 + Rust + TypeScript + ECharts + husky + Lighthouse + stylelint + .perf-baselines                                                                                                                                |
| Phases shipped         | v1 features (theme + report flows + drill-down dashboards UI) + schema/type refresh + codex bootstrap. **No v1.0 release closeout cadence yet.**                                                                        |
| Distribution channel   | **DMG via Apple Developer ID** (planned)                                                                                                                                                                                |
| Local LLM              | **Optional Ollama** (127.0.0.1 only; evidence-citation required)                                                                                                                                                        |
| Distinguishing tech    | Deterministic incident metrics (MTTD/MTTA/MTTR + weighted "pain" scores) + Jira CSV import with mapping UI + Slack transcript structured timeline extraction + ECharts dashboards + optional evidence-cited AI drafting |
| Notable files          | `HINSITE.md` (operator artifact, unusual name)                                                                                                                                                                          |
| Heavy QA tooling       | `.husky/` + `.lighthouserc.{json,production.json}` + `.stylelintrc.cjs` + `.perf-baselines/` + `.perf-results/`                                                                                                         |
| Migration state        | No `legacy-origin` remote                                                                                                                                                                                               |
| Distinguishing feature | **33rd signing cluster candidate (Active). Third local-LLM-dependent sub-pattern member, first with optional LLM use.** Heavy frontend QA tooling stack (Lighthouse + husky + stylelint + perf baselines).              |
