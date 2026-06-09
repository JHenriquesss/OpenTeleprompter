# Changelog

## [1.1.1] — 2026-06-09

- **Fix:** exiting the prompter while pinned (picture-in-picture) left the window stuck small and always-on-top. The window now auto-unpins/restores whenever you leave the prompter (Exit button, Esc, or any view change), not only via the Unpin button.

## [1.1.0] — 2026-06-08

New features.

- **Import more formats.** The library Import button (and drag-and-drop) now accept **`.txt`, `.md`/`.markdown`, `.pdf`, and `.docx`** in addition to plain text. PDFs use text-layer extraction, DOCX reads the WordprocessingML body, and Markdown is reduced to clean prose (headings/emphasis/links/list markers stripped). New `adapters/document.rs` with unit tests.
- **Drag-and-drop import.** Drop one or more supported files anywhere on the window to import them (opens the last in the editor). Uses Tauri's `tauri://drag-drop` event.
- **Picture-in-picture prompter.** A `📌 PiP` button in the prompter opens a small, always-on-top floating window running the prompter for the current script — read while another app (call/recording) is focused. The PiP window boots via `?pip=<script_id>`; backend command `open_pip_window`.
- **Finer speed control.** The prompter `◀◀` / `▶▶` buttons (and Up/Down keys) now step **0.05** instead of 0.25 for precise pacing.

## [1.0.2] — 2026-06-08

**Critical fix: all UI actions were dead in the shipped 1.0.0/1.0.1 builds.**

- Enabled `app.withGlobalTauri` in `tauri.conf.json`. The WASM frontend calls the backend through `window.__TAURI__.core.invoke`, but that global is only injected when `withGlobalTauri` is `true` (Tauri v2 defaults it to `false`). Without it every command — create/open/save/delete/import/export/settings/playback/update-check — threw, so buttons appeared to do nothing.
- This slipped through because every automated test used the in-memory `MockApi`; none exercised the real `window.__TAURI__` IPC bridge.
- Added `src-tauri/tests/full_flow_tests.rs`: 11 tests that drive the **real** service stack (real SQLite, real migrations, real import/export file round-trip) the way the Tauri command handlers do. Exposed `domain`/`persistence`/`services` as `pub` so tests reach actual app code instead of re-implementing SQL.
- Added `examples/scripts/*.txt` sample scripts for import testing.

**Also fixed in 1.0.2 (found by actually using the app):**

- **First-run settings deadlock.** `SettingsRepository::get()` held the `Mutex<Connection>` while calling `self.save()` to seed defaults on an empty DB. `std::sync::Mutex` is not re-entrant → first settings load froze. Now drops the lock before seeding.
- **Prompter looked frozen.** The scroll engine advanced `speed * 0.001` px/ms — 1 px/s at 1× (≈ one line per minute), and disagreed with the rest of the code (which assumes 60 px/s per 1×). Corrected to `speed * 0.06` (60 px/s per 1×). Added `e2e/test/specs/prompter.e2e.js` to assert the text actually scrolls after Play.
- **Import did nothing.** `open_file_dialog` / `save_file_dialog` were `async` commands calling the blocking dialog API, which can hang the async executor so the picker never resolves. Made them synchronous (Tauri runs sync commands on a thread pool, where blocking is safe). `read_text_file` now surfaces OS errors instead of silently returning `None`, and `dialog:default` was added to the capabilities.

> Note: auto-update cannot ship this fix to existing 1.0.0/1.0.1 installs (the update check also goes through the broken IPC bridge). Download 1.0.2 manually.

## [1.0.1] — 2026-06-08

Release-pipeline hardening (no app behavior change).

- Native **Intel macOS (x86_64)** DMG + updater now built on its own job (v1.0.0 shipped Apple Silicon only).
- `release.yml`: `prerelease` is set automatically from the tag (pre-release only for `-` tags), so stable tags publish as full releases (the updater endpoint resolves without a manual promote).
- The Intel-mac job no longer blocks the updater manifest: `latest.json` publishes from Windows + Linux + Apple-Silicon even if the scarce Intel runner is queue-stalled (Intel is added when it completes).
- `SHA256SUMS-*` filenames normalized (spaces → dots) to match the published GitHub asset names, so `sha256sum --check` works directly.

## [1.0.0] — 2026-06-08

First stable release. Cross-platform, offline, auto-updating desktop teleprompter.

### Teleprompter
- Fullscreen prompter with smooth `requestAnimationFrame` scrolling
- Variable speed (0.25×–10×), speed presets, and custom speed input
- Horizontal + vertical mirror modes for teleprompter hardware
- Adjustable typography (font size, line height, text width)
- Countdown timer, reading-guide band, progress % + estimated time remaining
- Pause markers (`[pause:N]` / `[breath]`) highlighted in-text
- Rehearsal mode (word count + estimated reading time)
- Resume playback (per-script scroll/speed/font/mirror) with resume dialog
- Jump controls (Arrow ±5s, Shift+Arrow ±20s) with toast feedback
- Full keyboard shortcuts with on-screen help (`H`); auto-hiding floating controls

### Library & editing
- Script library: create, edit, duplicate, delete, search
- Import / export `.txt` via native OS file dialogs
- Auto-save (debounced) with status; `Ctrl+S`; delete confirmation modal
- Dark / light theme, persisted; settings persisted to local SQLite

### Platform & distribution
- Windows (MSI, NSIS, portable ZIP), Linux (AppImage, deb, RPM), macOS DMG (Apple Silicon; Intel in v1.0.1)
- **System tray** — close-to-tray (app keeps running); left-click toggle; Show/Hide/Quit menu
- **Automatic updates** via `tauri-plugin-updater` — launch-time check, one-click install, minisign-verified, never silent
- Per-platform SHA256 checksums on every release

### Engineering
- Tauri v2 + Rust backend (rusqlite), Leptos CSR → WASM frontend (no JavaScript)
- Fully offline: no cloud, accounts, telemetry, or AI
- Mockable frontend API (`AppApi` trait) enabling component tests
- 85 automated tests (18 backend + 67 WASM); CI gate (fmt, clippy `-D warnings`, build, tests)

## [0.1.0] — 2026-06-06

### Added

- Initial scaffold: Tauri v2 + Leptos CSR + Trunk
- Domain models: Script, AppSettings, Theme, PlaybackState
- SQLite persistence with migrations
- Script CRUD commands: create, update, delete, get, list, search, duplicate
- Settings commands: get, update, reset
- Import/export commands (TXT)
- Frontend UI: AppShell, Sidebar, ScriptLibrary, ScriptEditor, PrompterView, SettingsPanel
- Smooth scrolling prompter using requestAnimationFrame
- Horizontal mirror mode (CSS scaleX)
- Keyboard shortcuts: Space, Esc, F, R, Arrow keys, M, +/-
- Full offline/local-first architecture
- Open-source docs: README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY
