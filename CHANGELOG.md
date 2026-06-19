# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.0 (2026-06-19)


### Features

* **ai:** add citation-enforced drafting ([2e2ae05](https://github.com/saagpatel/IncidentReview/commit/2e2ae05ada9333280ba1a4029bf9f4998442f3b4))
* **ai:** add embeddings index build and status ([733cf11](https://github.com/saagpatel/IncidentReview/commit/733cf112fd55c12f23e7f68e309fd526b3345223))
* **ai:** add evidence retrieval with citations ([6432094](https://github.com/saagpatel/IncidentReview/commit/6432094af8120baad73ccd287f742186a4f46bfc))
* **ai:** add evidence store and deterministic chunking ([1d2abc2](https://github.com/saagpatel/IncidentReview/commit/1d2abc2d3dc4e510574cc40cdcdf99c243afb4d3))
* **ai:** add Phase 5 gating and guidance ([1856471](https://github.com/saagpatel/IncidentReview/commit/18564711fa0fb864a1a07c5b98cb50ca836c7ec8))
* **ai:** list local Ollama models and select models ([13872ba](https://github.com/saagpatel/IncidentReview/commit/13872baa9f51ce2198a9f7b2db25be9a08286a1a))
* **data:** add backup/restore and sanitized export ([32e4621](https://github.com/saagpatel/IncidentReview/commit/32e46217049b1e7a7343748ae9d6969c6146a827))
* **dev:** add lean run mode and cleanup automation ([54713c6](https://github.com/saagpatel/IncidentReview/commit/54713c674558139c0619f7bd3682fe4fb6125885))
* **incidentreview:** prepare public rollout baseline ([7dc21b3](https://github.com/saagpatel/IncidentReview/commit/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d))
* **productization:** add AI audit trail and upgrade safety ([ad2a44c](https://github.com/saagpatel/IncidentReview/commit/ad2a44c0fedc284d189cf64295af9f20f1146448))
* **qir_core:** add sanitized dataset import ([b007879](https://github.com/saagpatel/IncidentReview/commit/b007879ae3487e6e6167892832774ac90feddedc))
* **qir_core:** add workspace DB open/create primitives ([7c51c94](https://github.com/saagpatel/IncidentReview/commit/7c51c945587781190faf2a3ab9b96949d89fc097))
* **quality:** enforce tests and living docs by default ([fa4457f](https://github.com/saagpatel/IncidentReview/commit/fa4457f78085812c15cd94931e9603044d270a42))
* **tauri:** add sanitized dataset inspect/import commands ([133f90f](https://github.com/saagpatel/IncidentReview/commit/133f90fb019cff1ecf8b2581dc75c72427ab4790))
* **tauri:** add workspace open/create/switch commands ([751faec](https://github.com/saagpatel/IncidentReview/commit/751faecc05a28d7ff8f00fc2a669a3709e2e6c88))
* **ui:** add sanitized dataset import flow ([68ec711](https://github.com/saagpatel/IncidentReview/commit/68ec711f060849f95d6cb22d103a99a00e831335))
* **ui:** branch sanitized import errors by code ([7923209](https://github.com/saagpatel/IncidentReview/commit/7923209d1b80e62d05c747332f1a734ccea56715))
* **ui:** enforce deterministic frontend quality gates ([85a3c25](https://github.com/saagpatel/IncidentReview/commit/85a3c25c76b330f0674e74ff2360e9ebd2e416ad))
* **workspace:** add create/open/switch UI and refresh ([4c91707](https://github.com/saagpatel/IncidentReview/commit/4c91707efd114575469e67d46f539cd2b08037af))


### Bug Fixes

* **ci:** emit quality check from main CI workflow ([27e8b0e](https://github.com/saagpatel/IncidentReview/commit/27e8b0e99f0373c7333a45832a02cea28208fd10))
* **ci:** harden incidentreview workflows ([f25f9d5](https://github.com/saagpatel/IncidentReview/commit/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f))
* **ci:** pin pnpm and normalize rust target dir ([8a0314a](https://github.com/saagpatel/IncidentReview/commit/8a0314a9ff1829207f2b797e17f09e3a722a7809))
* **ci:** remove blocked third-party actions from quality gates ([adb1400](https://github.com/saagpatel/IncidentReview/commit/adb1400662f7ffafd08a7431e3912ec802057371))
* **ci:** remove duplicate trufflehog fail flag ([5d8c590](https://github.com/saagpatel/IncidentReview/commit/5d8c590a93bc17ecd01508474e7271975941c898))
* **ci:** remove stale ts-expect-error in vite config ([17225df](https://github.com/saagpatel/IncidentReview/commit/17225dffced423acb649d740c08dbd0ee44b59c8))
* **ci:** stabilize frontend gates across environments ([52755f6](https://github.com/saagpatel/IncidentReview/commit/52755f6a4a2d3b4d94ba86d668967e7d69701462))
* **ci:** use stable action versions for quality gates ([5067773](https://github.com/saagpatel/IncidentReview/commit/506777384a4bc5aadb2ff53c93bf8f270cd72d68))
* **ui:** reduce cross-os visual test flake ([63411ec](https://github.com/saagpatel/IncidentReview/commit/63411ecfc463c0550f65c66947be12ff23228f0f))
* **ui:** stabilize mobile snapshot tolerance ([a9cf924](https://github.com/saagpatel/IncidentReview/commit/a9cf924557443ff5c7a21b8b17848307745c75ac))
* **ui:** tune visual tolerances for linux ci ([ac1574b](https://github.com/saagpatel/IncidentReview/commit/ac1574b629885b8d5deceedcbad8d6de5273db4d))

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
