# OpenPrompter RS — LLM Wiki

**Elevator pitch:** Local-first, offline desktop teleprompter. Tauri v2 + Rust backend (rusqlite) + Leptos CSR frontend (WASM). Script library, fullscreen prompter with smooth scrolling, adjustable settings, mirror mode, keyboard shortcuts. No cloud, no JS, no AI.

## Repository

- **Primary (public):** `JHenriquesss/OpenTeleprompter` — active development
- **Archive (private):** `JHenriquesss/Teleprompter` — retained with rewritten history after privacy incident; not deleted

## Files

| File | What |
|------|------|
| `01-architecture.md` | Layered architecture, animation rule, module boundaries, theme system, repo split, frontend API abstraction (Phase 11), self-update (Phase 14), system tray (Phase 16) |
| `02-test-tree.md` | Trunk path + tests per phase (incl. Phase 11–12 component tests + Phase 14 updater tests) |
| `03-phases.md` | Phase 1→Phase 15 (closed) — scaffold → build validation → Tauri commands → prompter features → native dialogs → public UX polish → CSS vars → packaging & release → cross-platform → user-friendly distribution → SHA256 checksums → recording continuity → public betas → toast system & WASM tests → Tauri API abstraction (11) → import/export/delete component tests (12) → CI runner/action maintenance (13) → tauri-plugin-updater self-update (14) → macOS Intel + Linux RPM builds (15) → system tray icon (16) |
| `04-decisions.md` | Decision log (rusqlite, Signal traits, wasm-bindgen, theme, resume playback, privacy incident, repo migration, AppApi trait, async-trait ?Send, Callback-for-non-Copy, MockApi cfg(test), fail_on, ConfirmModal aria, ToastState::snapshot, semver-only release trigger, windows-2025 pin, two-step updater, updater minisign signing) |
| `05-glossary.md` | Domain terms |
| `06-open-threads.md` | First signed updater release (deferred), macos-13 Intel runner queue, AppImage size, tauri-cli compile time; resolved: Phase 13 CI threads, Phase 14 auto-update, Phase 15 macOS Intel + Linux RPM, Phase 16 tray icon |

---
last-consolidated: 2026-06-07T23:55
sessions: 22 (Phase 1→Phase 16; Phase 13 CI maintenance, Phase 14 self-update, Phase 15 macOS Intel + Linux RPM, Phase 16 system tray)
