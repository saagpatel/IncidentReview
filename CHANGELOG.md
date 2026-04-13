# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-03-21

### Added

- Initial repository baseline
- Backup/restore and sanitized dataset export
- Sanitized dataset import with round-trip tests
- Sanitized dataset inspect/import Tauri commands
- Sanitized import flow in UI with error branching by code
- Workspace create/open/switch UI and Tauri commands
- AI evidence store and deterministic chunking
- AI embeddings index build and status
- AI evidence retrieval with citations
- AI citation-enforced drafting
- AI Phase 5 gating and guidance
- Local Ollama model listing and selection
- AI audit trail and upgrade safety
- Bundle chunking and session audit trail
- Release infrastructure (Phase 1)
- Dashboard caching layer (Phase 3.1)
- v1.1 Phase 1: performance optimizations foundation
- v1.1 Phase 2: evidence pagination backend
- v1.1 Phase 3: multi-turn drafting schema and core logic
- v1.1 Phase 4: localization foundation and i18n setup
- Lean run mode and cleanup automation
- Deterministic frontend quality gates
- Codex reliability gate workflow
- Tests and living docs enforcement

### Fixed

- CI: harden IncidentReview workflows
- CI: emit quality check from main CI workflow
- CI: remove blocked third-party actions from quality gates
- CI: use stable action versions for quality gates
- CI: remove duplicate trufflehog fail flag
- CI: pin pnpm and normalize Rust target dir
- CI: remove stale ts-expect-error in vite config
- CI: stabilize frontend gates across environments
- UI: tune visual tolerances for Linux CI
- UI: reduce cross-OS visual test flake
- UI: stabilize mobile snapshot tolerance

### Changed

- Prepared public rollout baseline
- Added AI boundary ADR
- Refreshed generated references
- Modularized App into feature sections
- Robustness testing and performance profiling (Phase 2)
- Finalized codex OS bootstrap baseline
- Bootstrapped codex OS guardrails
- Pruned bloat and hardened local cleanup
