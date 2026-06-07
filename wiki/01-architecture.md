# Architecture

## Stack

| Layer | Tech | Role |
|-------|------|------|
| Desktop shell | Tauri v2 | Window, native menus, system integration |
| Backend | Rust (native) | SQLite persistence, file I/O, business logic |
| Frontend | Leptos 0.6 CSR → WASM | UI, animation loop, keyboard handling, resume dialog, jump feedback |
| Build | Trunk | WASM bundling, dev server |
| Release | GitHub Actions + Tauri CLI | Tag-triggered builds: Windows (MSI+NSIS+ZIP), Linux (AppImage+deb), macOS (DMG) |
| DB | rusqlite 0.31 (bundled) | Local SQLite, WAL mode |

## Project layout

```
OpenTeleprompter/
├── src/                     Leptos frontend (compiled to WASM)
├── src-tauri/               Rust backend (native binary)
├── src-tauri/tests/         Integration tests (backend)
├── wiki/                    LLM wiki (this)
├── scripts/                 check.sh + check.ps1 validation pipeline
├── .github/workflows/       CI (ci.yml) + Release (release.yml)
├── rust-toolchain.toml      Toolchain pinning (stable, wasm32, rustfmt, clippy)
├── index.html               Trunk entry point (<link data-trunk rel="rust" data-bin="openprompter-rs" />)
├── Cargo.toml               Workspace root + frontend deps (no uuid/chrono — break wasm32)
└── Trunk.toml               Trunk config (watch.ignore empty — canonicalize bug on non-ASCII paths)
```

## Backend layers (`src-tauri/src/`)

```
commands/    ← Tauri #[tauri::command] handlers (scripts, settings, playback_state)
services/    ← business logic (script_service, settings_service, playback_state_service)
persistence/ ← rusqlite repos + migrations (script_repository, settings_repository, playback_state_repository)
domain/      ← pure data structs + errors (script, settings, playback_state)
adapters/    ← file system, dialogs (stub)
```

Dependency direction: `commands → services → persistence → domain`. No circular deps.

## Frontend layers (`src/`)

```
components/  ← UI components (AppShell → Sidebar, ScriptLibrary, etc.)
state/       ← Leptos RwSignal contexts (AppState, PlaybackState, UiState, ToastState)
prompter/    ← Engine, mirror transform, keyboard handler (Box<dyn Fn()> actions)
bindings/    ← tauri_api.rs: typed wrappers for all 16 commands (ScriptData, AppSettingsData, ScriptPlaybackStateData structs, invoke_tauri helper)
```

## Leptos 0.6 API quirks (found in Phase 1.2)

- **`Signal<T>` access**: use `.get()` (not `()`). Signal does NOT implement `Fn() -> T` in 0.6 CSR. `SignalGet` trait must be in scope.
- **`RwSignal::update()` / `set()`**: require `SignalUpdate` / `SignalSet` traits in scope (`use leptos::{SignalUpdate, SignalSet}`). Easy to miss when adding new RwSignal fields.
- **`NodeRef<T>`**: use `.get()` (not `()`).
- **Closures with captured state**: cannot coerce to `fn()`. Use `Box<dyn Fn() + 'static>` in prop/struct types. Mark with `#[prop(into)]` where possible.
- **Disjoint async block types**: two `async {}` blocks in if/else have different types. Use single `async move {}` with conditional inside.
- **`wasm-bindgen-futures`**: required for `async fn` inside `extern "C"` block (tauri_api bindings).

## Critical rule: animation lives in frontend

Teleprompter scrolling runs via `requestAnimationFrame` in `prompter/engine.rs`. The loop reads `speed` and `is_playing` signals, increments `scroll_y`, applies CSS `transform: translateY()`. **No Tauri commands are called during playback.** [[src/prompter/engine.rs]]

### Cleanup

Animation loop returns `Rc<Cell<bool>>` alive flag. On component unmount, `on_cleanup` sets it `false` → next frame skips re-scheduling, stopping the loop. Keyboard listener uses `on_cleanup` with `remove_event_listener_with_callback`. Both prevent leaks on remount. [[src/components/prompter_view.rs]]

## State management

Four Leptos contexts provided at `App::new()`:

- `AppState` — view routing (`View::Library|Editor|Prompter|Settings`), selected/editing script ID, search query, `library_refresh_trigger: RwSignal<u32>` + `refresh_library()` method (increments trigger → ScriptLibrary create_resource re-fetches)
- `PlaybackState` — is_playing, scroll_y, speed, jump_big_forward(), jump_big_backward() methods (20s jumps)
- `UiState` — font_size, line_height, text_width, mirror_mode, mirror_vertical, countdown_seconds, reading_guide, show_shortcut_help, **theme**
- `ToastState` — `RwSignal<Vec<ToastMessage>>`, 4 add methods (success/error/warning/info). Global unique IDs via `AtomicU32` static counter. Auto-dismiss after 4s via `setTimeout` + `Closure::once`. `ToastContainer` component renders fixed bottom-right, uses `<For>` for reactive list. [[src/state/toast.rs]]

All use `RwSignal<T>` for mutable reactivity. [[src/state/]]

## Theme system (Phase 5 + 5.1)

Theme toggle uses existing backend `Theme` enum (`Light`/`Dark`, `Dark` default) serialized as string. Applied via:

1. **`UiState.theme: RwSignal<String>`** — reactive state in frontend. Set on mount from `get_settings()`, written via `update_settings()` on save.
2. **CSS variables on root div** — `AppShell` renders `<div class="app-shell theme-{dark|light}">`. A `<style>` tag injects `:root` + `.theme-dark` + `.theme-light` CSS variable blocks defining `--bg-main`, `--bg-panel`, `--text-main`, `--text-muted`, `--accent`, `--border-color`, `--scrollbar-thumb`, etc. (25+ variables per theme).
3. **All component colors use `var()`** — Phase 5.1 replaced hardcoded `#16213e`, `#e94560`, `#e0e0e0`, `#333`, `#555` etc. with CSS variables in `sidebar.rs`, `script_library.rs`, `script_editor.rs`, `settings_panel.rs`, `confirm_modal.rs`. PrompterView stays dark-first (distraction-free mode, hardcoded `#000`/`#fff`). `index.html` body background simplified to fallback. [[src/components/]]
4. **Scrollbar styling** — `::-webkit-scrollbar` rules use `var(--scrollbar-thumb)` for per-theme colors.
5. **inline styles use `var(--xxx)` directly** — browser resolves at render time; no Leptos reactivity needed for color values. `format!("var(--xxx)")` used in dynamic style expressions (e.g. toggle switch colors).

### Ctrl+S save shortcut (Editor)

`ScriptEditor` registers `on:keydown=on_keydown` on both title `<input>` and content `<textarea>`. Handler checks `ev.key() == "s" && (ev.ctrl_key() || ev.meta_key())`, calls `ev.prevent_default()` + `perform_save()`. Save hint `"Ctrl+S"` rendered next to Save button. [[src/components/script_editor.rs]]

Keyboard shortcut summary:
| Key | Action | Context |
|-----|--------|---------|
| Ctrl+S | Save script | Editor (title input, content textarea) |
| Space/Space | Play/Pause | Prompter |
| ... | (see table below) | Prompter |

### Loading/empty state polish (Phase 5)

| Component | Loading | Empty | Error |
|-----------|---------|-------|-------|
| ScriptLibrary | Icon + "Loading scripts..." + subtitle | Icon + "No scripts yet." + inline "+ New Script" button, or search-specific "No matches" | Icon + error text |
| SettingsPanel | "Loading settings..." text | — | Inline red banner |

## Release workflow (Phase 6 → 7 → 7.1)

`.github/workflows/release.yml` triggered on `v*` tags. Three parallel jobs (Phase 7):

### Jobs

| Job | Runner | Produces | Duration |
|-----|--------|----------|----------|
| `build-windows` | `windows-latest` | MSI + NSIS + portable ZIP | ~33 min |
| `build-linux` | `ubuntu-latest` | AppImage + deb | ~20 min |
| `build-macos` | `macos-latest` (ARM64) | DMG | ~16 min |

### Pipeline (per job)

1. Checkout + dtolnay/rust-toolchain (stable, wasm32, rustfmt, clippy) + Swatinem/rust-cache
2. `cargo install trunk --locked` + `cargo install tauri-cli --version "^2"`
3. Validation: fmt → backend check → 14 tests → clippy (`-D warnings`) → trunk build
4. `cargo tauri build` (release mode, produces platform installers)
5. **SHA256 checksum generation** (Phase 7.1): per-platform checksums via `Get-FileHash` (Windows), `sha256sum` (Linux), `shasum -a 256` (macOS)
6. `softprops/action-gh-release@v2` uploads binaries + checksum files to GitHub Release (pre-release)

### Checksum files (Phase 7.1)

| File | Entry format | Contains hashes for |
|------|-------------|-------------------|
| `SHA256SUMS-windows.txt` | `<hash>  <filename>` (leaf) | MSI, NSIS, portable ZIP |
| `SHA256SUMS-linux.txt` | `<hash>  <relative-path>` | AppImage, deb |
| `SHA256SUMS-macos.txt` | `<hash>  <relative-path>` | DMG |

Linux/macOS checksum files use build-directory relative paths (cosmetic — manual comparison via `Get-FileHash` or visual hash check works fine; `sha256sum --check` requires same directory structure).

### Version policy

- **Source version:** `0.9.0` (bumped `0.1.0` → `0.6.0` → `0.7.0` → `0.7.1` → `0.8.0` → `0.9.0`). Stored in `Cargo.toml`, `src-tauri/Cargo.toml`, `tauri.conf.json`. Displayed in sidebar.
- **Phase tags:** internal dev milestones, e.g. `v0.9.0-phase9`. Match `v*` pattern but create duplicate releases (harmless).
- **Release tags:** public distribution, e.g. `v0.9.0-beta.1`. Trigger release workflow.

### Generated artifacts (v0.9.0)

| Platform | Artifacts | Notes |
|----------|-----------|-------|
| Windows | MSI (~3.9 MB), NSIS (~3.0 MB), portable ZIP (~3.6 MB) | ZIP contains EXE + README-INSTALL.txt |
| Linux | AppImage (~78 MB), deb (~3.9 MB) | AppImage bundles webkit2gtk runtime |
| macOS | DMG (~3.9 MB) | `aarch64` only (macos-latest = ARM runner) |

## Recording continuity (Phase 9)

`ScriptPlaybackState` is a third domain entity alongside Script and AppSettings, stored in its own SQLite table `script_playback_state` with FK ON DELETE CASCADE to scripts. [[src-tauri/src/domain/playback_state.rs]]

### Fields

- `script_id` TEXT (FK → scripts.id ON DELETE CASCADE)
- `scroll_offset_px` REAL
- `speed_multiplier` REAL
- `font_size` REAL
- `line_height` REAL
- `mirror_mode` TEXT (none/horizontal/both)
- `mirror_vertical` INTEGER (0/1)
- `updated_at` TEXT (ISO 8601, auto-generated)

### Backend

| Layer | File | Role |
|-------|------|------|
| Domain | `src-tauri/src/domain/playback_state.rs` | `ScriptPlaybackState` struct |
| Persistence | `src-tauri/src/persistence/playback_state_repository.rs` | CRUD operations in `script_playback_state` table (migration v3) |
| Service | `src-tauri/src/services/playback_state_service.rs` | Business logic, `#[allow(clippy::too_many_arguments)]` |
| Commands | `src-tauri/src/commands/playback_state.rs` | 3 Tauri commands: `save_playback_state`, `load_playback_state`, `clear_playback_state` |

Registered in `lib.rs`: `PlaybackStateRepository` → `PlaybackStateService` → `PlaybackStateCommandHandler`. [[src-tauri/src/lib.rs]]

### Frontend

- `ScriptPlaybackStateData` struct in `bindings/tauri_api.rs` mirrors backend domain. [[src/bindings/tauri_api.rs]]
- Resume dialog on prompter entry: shows progress % (scroll position / content length × 100), offers "Resume" or "Start from beginning". [[src/components/prompter_view.rs]]
- Periodic save: throttled via `set_interval` (3s) during playback; save on pause, save on exit. Uses `web_sys::window().set_interval_with_callback_and_timeout_and_arguments_0` with `Closure::forget()`. No backend calls during animation frame.
- Jump controls: `PlaybackState.jump_big_forward()`/`jump_big_backward()` add/subtract 20s of scroll distance. Visual toast feedback ("+5s", "-5s", "+20s", "-20s") fades out. [[src/state/playback_state.rs]]
- Reset button clears saved position via `clear_playback_state` command. R key resets scroll to top.
- Custom speed input: `<input type="number">` in floating controls with `validate_speed()` (clamps 0.25–5.0, validates input, shows error toast). [[src/prompter/speed.rs]]

### Key constraints

- All builds **unsigned** — SmartScreen, Gatekeeper warnings expected for beta.
- macOS: `aarch64` only. No x86_64 (would need `macos-13` runner or matrix).
- Linux: deb + AppImage only. No RPM.
- `cargo install tauri-cli` compiles from source (~10 min on cache-miss).
- `actions/checkout@v4` + `softprops/action-gh-release@v2` run on Node.js 20 (deprecated; Node.js 24 by Sep 2026).
- `windows-latest` migrating to `windows-2025-vs2026` by Jun 15, 2026.

## Typed Tauri bindings pattern

`bindings/tauri_api.rs` exposes two serializable structs mirroring the backend domain:

- `ScriptData` — id, title, content, created_at, updated_at
- `AppSettingsData` — font_size, line_height, text_width, scroll_speed, mirror_mode, **theme** (String), countdown_seconds, mirror_vertical, reading_guide_enabled

Plus 13 typed async functions wrapping `invoke()` calls. Each function deserializes the JSON response via a generic `invoke_tauri<T, R>(cmd, args)` helper. [[src/bindings/tauri_api.rs]]

Components import `crate::bindings::tauri_api` (not individual types) to avoid name conflicts.

### Library refresh pattern

`AppState.refresh_library()` increments `library_refresh_trigger`. `ScriptLibrary`'s `create_resource` depends on `(search_query, library_refresh_trigger)`, so any mutation (create, update, delete, duplicate, import) triggers a re-fetch. [[src/state/app_state.rs:32]]

## Keyboard shortcuts

Handled in `PrompterView` via `window.add_event_listener("keydown")`. A single `KeyboardActions` struct dispatches to signal mutations. Action handlers are `Box<dyn Fn() + 'static>` because closures capture RwSignal references — cannot coerce to `fn()`. [[src/prompter/keyboard.rs]]

| Key | Action |
|-----|--------|
| Space | toggle_play |
| Esc | exit_prompter → View::Library |
| F | request_fullscreen |
| R | restart (scroll_y=0, play) |
| ArrowUp | speed += 0.25 |
| ArrowDown | speed -= 0.25 |
| ArrowLeft | jump backward ~5s |
| ArrowRight | jump forward ~5s |
| Shift+ArrowLeft | big jump backward ~20s |
| Shift+ArrowRight | big jump forward ~20s |
| M | toggle_mirror |
| V | toggle_vertical_mirror |
| +/= | font_size += 2 |
| -/_ | font_size -= 2 |
| H | toggle shortcut help overlay |

Custom speed input (Phase 9): text field in floating controls, validates input, clamps 0.25×–5.0×, shows friendly error. [[src/prompter/speed.rs]]

Speed presets (Phase 8): 0.5×, 1×, 1.5×, 2×, 3× buttons in floating controls.

## Repository split

Two repos after privacy incident (2026-06-07):

| Repo | Visibility | Purpose |
|------|-----------|---------|
| `JHenriquesss/Teleprompter` | PRIVATE | Archived. History rewritten (git-filter-repo) to remove docs/screenshots/. 4 PR refs still on GitHub — pending Support purge |
| `JHenriquesss/OpenTeleprompter` | PUBLIC | Active development. Fresh single commit (no old history, tags, PRs, releases). Clean export + validation before push |

**Migration method:** `robocopy` source (excluding .git, target, dist, src-tauri/target, *.db, *.sqlite), sanitized references, re-initialized git, pushed to new public repo. Old repo kept private for archive access.

## Frontend API abstraction (Phase 11)

`src/bindings/` layers Tauri access so components are testable without a backend:

| File | Role |
|------|------|
| `tauri_api.rs` | Raw `invoke` wrappers + typed structs (`ScriptData`, `AppSettingsData`, `ScriptPlaybackStateData`). Unchanged. |
| `app_api.rs` | `AppApi` trait (`#[async_trait(?Send)]`) over all commands; `RealTauriApi` delegates to the `tauri_api` free fns (prod path byte-identical). |
| `mock_api.rs` | `MockApi` test double — `#[cfg(test)]` only; in-memory `RefCell` store, builders, call log, error injection. |
| `mod.rs` | `pub type ApiCtx = Rc<dyn AppApi>` — Leptos context handle. |

- Flow: `app.rs` provides `Rc::new(RealTauriApi)`; components do `use_context::<ApiCtx>()` → `api.<cmd>().await`. Tests provide `Rc::new(MockApi)`.
- `Rc<dyn AppApi>` is not `Copy` → handlers used >once / in reactive `Fn` closures / list `.map` are wrapped in Leptos `Callback` (Copy); single-use handlers clone the `Rc` inline. See [[04-decisions.md]].
- Component tests (`src/component_tests.rs`, Phase 11–12, 14): mount real components into a detached DOM via `mount_to`, drive real DOM clicks (`click_by_title`/`click_by_aria`/`click_text`), assert via MockApi (`call_count`/`was_not_called`/`exported`/`scripts`/`with_update`) + `ToastState::snapshot()`. Async settles via bounded `tick`/`settle` poll. [[02-test-tree.md]]

## Self-update (Phase 14)

`tauri-plugin-updater` drives in-app updates against GitHub Releases:

- **Backend** (`src-tauri/src/commands/updater.rs`): two-step `check_for_update` → `install_update`. The non-serializable `Update` handle is stashed in `PendingUpdate` (`Mutex<Option<Update>>`) managed state between the calls; only the serializable `UpdateInfo` crosses IPC. `install_update` downloads, installs, then `app.restart()`.
- **Config** (`tauri.conf.json` `plugins.updater`): embedded minisign `pubkey`, endpoint `releases/latest/download/latest.json`, Windows `installMode: passive`. Private signing key is git-ignored (`.updater-keys/`) + lives in GH secret `TAURI_SIGNING_PRIVATE_KEY`.
- **Frontend**: `UpdateBanner` (`src/components/update_banner.rs`) auto-checks on mount through `ApiCtx`; silent when up to date, error toast on check failure, Install/Dismiss when an update exists. Wired into `AppShell` above sidebar+content (hidden in fullscreen prompter). Install is user-initiated only — no silent auto-install.
- First *signed release* is deferred (needs `bundle.createUpdaterArtifacts` + secrets + a `latest.json`); see docs/release.md → Auto-Update and [[06-open-threads.md]].

## System tray (Phase 16)

`src-tauri/src/tray.rs` + `lib.rs` wiring. Requires the `tray-icon` **cargo feature** on `tauri` (no new crate).

- **Testable seam:** `tray_action(menu_id: &str) -> Option<TrayAction>` — pure id→`{Show,Hide,Quit}` map (3 unit tests). Menu ids are `MENU_SHOW`/`MENU_HIDE`/`MENU_QUIT` constants shared by builder + mapper so they can't drift.
- **Tray:** `build_tray` (called from `.setup`) builds a `TrayIconBuilder` with the app icon, tooltip, and a Show/Hide/Quit menu. `show_menu_on_left_click(false)`: left-click toggles window visibility (`on_tray_icon_event`, `Click`+`Left`+`Up`), right-click opens the menu (`on_menu_event` → `tray_action` → `apply_action`).
- **Hide-to-tray:** `lib.rs` `.on_window_event` intercepts `CloseRequested` → `api.prevent_close()` + `window.hide()` + `window.emit("close-to-tray", ())`. App keeps running; **Quit (tray) is the only full exit** (`app.exit(0)`).
- **One-time hint:** frontend `AppShell` effect calls `tauri_api::on_close_to_tray_once` (wraps Tauri `event.once`) → info toast at most once per app run.
- Tray/window runtime behavior is **manual-verified** (GUI, not unit-testable); `tray_action` is the automated part. [[03-phases.md#phase-16-system-tray-icon]]
