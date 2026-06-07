# OpenPrompter RS — LLM Wiki

**Elevator pitch:** Local-first, offline desktop teleprompter. Tauri v2 + Rust backend (rusqlite) + Leptos CSR frontend (WASM). Script library, fullscreen prompter with smooth scrolling, adjustable settings, mirror mode, keyboard shortcuts. No cloud, no JS, no AI.

## Repository

- **Primary (public):** `JHenriquesss/OpenTeleprompter` — active development
- **Archive (private):** `JHenriquesss/Teleprompter` — retained with rewritten history after privacy incident; not deleted

## Files

| File | What |
|------|------|
| `01-architecture.md` | Layered architecture, animation rule, module boundaries, theme system, repo split |
| `02-test-tree.md` | Trunk path + tests per phase |
| `03-phases.md` | Phase 1→Phase 10 (closed) — scaffold → build validation → Tauri commands → prompter features → native dialogs → public UX polish → CSS vars → packaging & release workflow → cross-platform packaging → user-friendly distribution → SHA256 checksums → recording continuity & precision controls → public beta releases → toast system & WASM test foundation |
| `04-decisions.md` | Decision log (rusqlite, Signal traits, wasm-bindgen, icon gen, theme, Ctrl+S, screenshots, release permissions, resume playback, per-script prefs, privacy incident, repo migration) |
| `05-glossary.md` | Domain terms |
| `06-open-threads.md` | Carried threads across all phases, Phase 5 deferrals |

---
last-consolidated: 2026-06-07T07:30
sessions: 16 (Phase 1→Phase 10, privacy incident, repo migration)
