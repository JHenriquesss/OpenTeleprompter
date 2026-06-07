# OpenPrompter RS — LLM Wiki

**Elevator pitch:** Local-first, offline desktop teleprompter. Tauri v2 + Rust backend (rusqlite) + Leptos CSR frontend (WASM). Script library, fullscreen prompter with smooth scrolling, adjustable settings, mirror mode, keyboard shortcuts. No cloud, no JS, no AI.

## Repository

- **Primary (public):** `JHenriquesss/OpenTeleprompter` — active development
- **Archive (private):** `JHenriquesss/Teleprompter` — retained with rewritten history after privacy incident; not deleted

## Files

| File | What |
|------|------|
| `01-architecture.md` | Layered architecture, animation rule, module boundaries, theme system, repo split, frontend API abstraction (Phase 11) |
| `02-test-tree.md` | Trunk path + tests per phase (incl. Phase 11–12 component tests) |
| `03-phases.md` | Phase 1→Phase 12 (closed) — scaffold → build validation → Tauri commands → prompter features → native dialogs → public UX polish → CSS vars → packaging & release → cross-platform → user-friendly distribution → SHA256 checksums → recording continuity → public betas → toast system & WASM tests → Tauri API abstraction (11) → import/export/delete component tests (12) |
| `04-decisions.md` | Decision log (rusqlite, Signal traits, wasm-bindgen, theme, resume playback, privacy incident, repo migration, AppApi trait, async-trait ?Send, Callback-for-non-Copy, MockApi cfg(test), fail_on, ConfirmModal aria, ToastState::snapshot) |
| `05-glossary.md` | Domain terms |
| `06-open-threads.md` | Phase 13 CI-maintenance deadline, Phase 11–12 testing threads, carried threads |

---
last-consolidated: 2026-06-07T17:00
sessions: 18 (Phase 1→Phase 12, privacy incident, repo migration; Phase 11 API abstraction + Phase 12 import/export/delete component tests)
