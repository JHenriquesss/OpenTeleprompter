# OpenPrompter RS — LLM Wiki

**Elevator pitch:** Local-first, offline desktop teleprompter. Tauri v2 + Rust backend (rusqlite) + Leptos CSR frontend (WASM). Script library, fullscreen prompter with smooth scrolling, adjustable settings, mirror mode, keyboard shortcuts. No cloud, no JS, no AI.

## Repository

- **Primary (public):** `JHenriquesss/OpenTeleprompter` — active development
- **Archive (private):** `JHenriquesss/Teleprompter` — retained with rewritten history after privacy incident; not deleted

## Files

| File | What |
|------|------|
| `01-architecture.md` | Layered architecture, animation rule, module boundaries, theme system, repo split, frontend API abstraction (Phase 11), self-update (Phase 14), system tray (Phase 16), document import + drag-drop + PiP + invoke-catch (Phase 21/22) |
| `02-test-tree.md` | Trunk path + tests per phase (incl. component tests, updater tests, document import, scroll-math + resume regression, e2e/cdp suite) |
| `03-phases.md` | Phase 1→22 (closed) — … → v1.0.0 release+manifest (17) → cross-platform verify (18) → docs (19) → release hardening v1.0.1 (20) → **v1.1.0 features: multi-format import/drag-drop/PiP/0.05-speed (21)** → **v1.1.x critical fixes: dead buttons, deadlock, freeze, import, resume, two-clicks, PiP (22)** |
| `04-decisions.md` | Decision log (… AppApi trait, updater minisign, **withGlobalTauri, invoke catch, camelCase IPC args, non-blocking dialog, document extraction, backend drag-drop, PiP=pin-main-window, scroll_delta_px, resume sync-save, settings lock-drop, unsigned reaffirmed, CDP regression suite**) |
| `05-glossary.md` | Domain terms |
| `06-open-threads.md` | Active: public 1.0.x was broken→superseded by v1.1.x (no auto-update for old installs), orphaned v1.1.0/1.1.1 tags, e2e non-blocking (CDP suite covers integration), launch-dir DB path, unsigned. Resolved: v1.1.x fixes; Phase 13–20 CI/updater/tray/hardening |

---
last-consolidated: 2026-06-09T03:30
sessions: 25 (Phase 1→Phase 22). v1.1.x: **the shipped 1.0.0/1.0.1 were non-functional** — dead buttons (missing `withGlobalTauri`) + first-run settings deadlock. Phase 21 added multi-format import (.txt/.md/.pdf/.docx) + drag-drop + picture-in-picture (pin main window) + 0.05 speed; Phase 22 fixed dead buttons, silent invoke errors, import camelCase `fileName`/dialog deadlock, prompter freeze (scroll ×0.06), settings deadlock, resume-to-0%, two-clicks (mousemove re-render), PiP blank/close/exit. Verified by driving the real build over WebView2 CDP (UI Automation can't see WebView2). v1.1.0/v1.1.1 tags pushed then cancelled pre-publish (orphaned) — next clean tag is the real release. OS code signing still out of scope (unsigned).
