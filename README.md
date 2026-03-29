# IncidentReview

[![Rust](https://img.shields.io/badge/Rust-dea584?style=flat-square&logo=rust)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&logo=typescript)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> Quarterly reviews shouldn't take a week to assemble. Ingest your Jira exports and Slack transcripts, get deterministic metrics and storytelling dashboards in minutes.

IncidentReview is a local-first macOS desktop app for engineering teams running Quarterly Incident Reviews. Import Jira incident CSVs and Slack transcripts, compute MTTD/MTTA/MTTR and severity distributions with audited deterministic logic, and render ECharts dashboards ready for executive presentation — without sending internal incident data to any third party. Optional local AI via Ollama adds evidence-backed narrative drafting.

## Features

### Data Ingest
- Jira incident CSV import with a mapping UI and saved column mappings
- Slack transcript ingestion (file or paste) with structured timeline extraction
- Demo seed, sanitized export/import, and backup/restore for portability
- Validation report for missing timestamps and out-of-range values

### Deterministic Metrics
- Per-incident: MTTD, IT awareness lag, MTTA, time to mitigation, MTTR
- Quarter rollups: volume, severity/impact distributions, repeat offenders by vendor/service
- Weighted "pain" scores (impact × degradation × duration) with configurable weights

### Storytelling Dashboards
- Quarter at a glance with KPI deltas
- Timeline and trend views (ECharts, drill-down enabled)
- Severity and impact distributions, vendor heat maps
- Repeat offender analysis

### AI Drafting (Optional)
- Ollama local AI only — no external calls, `127.0.0.1` only
- Evidence-first: AI must cite evidence chunks or mark UNKNOWN
- Executive summary, theme synthesis, and recommendations

## Quick Start

### Prerequisites

- Node.js 18+
- Rust stable toolchain (`rustup`)
- Tauri system dependencies: [tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/)
- Ollama (optional, for AI narrative)

### Installation

```bash
git clone https://github.com/saagpatel/IncidentReview
cd IncidentReview
npm install
```

### Usage

```bash
# Start in development mode
npm run tauri dev
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2 |
| Frontend | React, TypeScript, Tailwind CSS |
| Charts | ECharts |
| Metrics engine | Rust — deterministic, audited calculations |
| AI narrative | Ollama (localhost only, optional) |
| Storage | SQLite (local app data dir) |

## Architecture

All metrics computation runs in Rust where the logic is deterministic and testable — no AI arithmetic. The AI layer is strictly read-only on the data model; it can only draft narrative text and must cite evidence. The frontend never exposes raw incident data outside the machine. Jira mappings and analysis configs are stored locally and travel with backup exports.

## License

MIT
