# Changelog

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
