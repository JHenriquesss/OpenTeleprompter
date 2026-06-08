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
| `03-phases.md` | Phase 1→Phase 15 (closed) — scaffold → build validation → Tauri commands → prompter features → native dialogs → public UX polish → CSS vars → packaging & release → cross-platform → user-friendly distribution → SHA256 checksums → recording continuity → public betas → toast system & WASM tests → Tauri API abstraction (11) → import/export/delete component tests (12) → CI runner/action maintenance (13) → tauri-plugin-updater self-update (14) → macOS Intel + Linux RPM builds (15) → system tray icon (16) → v1.0.0 release + updater manifest (17) → cross-platform verification (18) → docs refresh (19) → release hardening v1.0.1 (20) |
| `04-decisions.md` | Decision log (rusqlite, Signal traits, wasm-bindgen, theme, resume playback, privacy incident, repo migration, AppApi trait, async-trait ?Send, Callback-for-non-Copy, MockApi cfg(test), fail_on, ConfirmModal aria, ToastState::snapshot, semver-only release trigger, windows-2025 pin, two-step updater, updater minisign signing) |
| `05-glossary.md` | Domain terms |
| `06-open-threads.md` | Open: macOS Intel build (GitHub runner queue, non-blocking), AppImage size, tauri-cli compile time, user GUI smoke; resolved: Phase 13 CI, 14 auto-update, 15 Intel+RPM, 16 tray, 17/20 release+updater+hardening |

---
last-consolidated: 2026-06-08T11:00
sessions: 24 (Phase 1→Phase 20; v1.0.0 + v1.0.1 shipped — stable, auto-update live win/linux/mac-arm, release pipeline hardened; Phase 17 release+manifest, 18 verification, 19 docs, 20 hardening. Intel-mac pending GitHub runner, non-blocking)
