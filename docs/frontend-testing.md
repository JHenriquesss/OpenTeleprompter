# Frontend Testing Guide

## Overview

The frontend crate (`openprompter-rs`) is a Leptos CSR app compiled to WASM.
Testing is split into three categories:
1. **Pure-logic WASM tests** (run in browser via `wasm-pack test`)
2. **Architecture assertions** (automated source inspection)
3. **Manual fallback** (requires user interaction in running app)

## What We Test Today (Phase 11)

All WASM tests use `wasm_bindgen_test` and run in a headless browser:

```
wasm-pack test --headless --chrome
```

### Pure-logic + state tests (41 tests)

| Module | File | Tests | What they cover |
|--------|------|-------|-----------------|
| `speed` | `src/prompter/speed.rs` | 5 | validate_speed, speed_label, presets, word_count, estimated_reading_seconds |
| `mirror` | `src/prompter/mirror.rs` | 6 | mirror_transform, mirror_transform_combined (4 combos) |
| `toast` | `src/state/toast.rs` | 8 | add_success/error/warning/info, dismiss, unique ids, css_class, icon |
| `playback_state` | `src/state/playback_state.rs` | 14 | initial state, toggle_play, restart, speed up/down/set, jump (small/big), clamps |
| `ui_state` | `src/state/ui_state.rs` | 8 | initial state, font size up/down/clamps, toggle mirror/reading_guide/shortcut_help |

### Component integration tests (20 tests) — `src/component_tests.rs`

Added in Phase 11 (11) and Phase 12 (9). Components are mounted into a detached
DOM node and driven by `MockApi` (no Tauri, no `invoke`, no native dialogs).
Total WASM tests: **61**.

| Test | What it proves |
|------|----------------|
| `mock_api_lists_and_searches_scripts` | MockApi list + search (title and content match) |
| `mock_api_creates_deletes_duplicates` | MockApi CRUD + `(copy)` duplicate semantics |
| `mock_api_updates_and_resets_settings` | MockApi settings persist + reset to defaults |
| `mock_api_saves_loads_clears_playback` | MockApi playback save/load/clear round-trip |
| `mock_api_error_injection_fails_all` | `.failing()` forces every command to `Err` |
| `script_library_renders_scripts_from_mock` | `<ScriptLibrary>` renders titles from mock `list_scripts` |
| `script_library_shows_empty_state` | Empty library renders "No scripts yet" |
| `settings_panel_loads_through_mock` | `<SettingsPanel>` calls `get_settings` on mount |
| `script_editor_loads_selected_script` | `<ScriptEditor>` calls `get_script` for selected id |
| `prompter_view_shows_resume_dialog_when_state_exists` | `<PrompterView>` loads playback + shows Resume dialog |

#### Phase 12 — ScriptLibrary action flows (9 tests)

Drive real DOM clicks on the rendered buttons (`click_by_title` for row actions,
`click_by_aria` for the ConfirmModal, `click_text` for Import) and assert via
`MockApi` (`call_count`, `was_not_called`, `exported`, `scripts`) and
`ToastState::snapshot` (`assert_toast_contains_success/error`, `assert_no_error_toast`).
Targeted failures use `MockApi::fail_on(cmd)` so a flow can reach its intended
failure point past earlier successful calls (e.g. import = dialog → read → import).

| Test | What it proves |
|------|----------------|
| `import_success_creates_script_with_content` | Import creates a script with filename-derived title + file body; success toast |
| `export_success_exports_correct_script` | Export calls the command once with the correct script id (`exported()`); success toast |
| `duplicate_creates_copy` | Duplicate adds one script; success toast |
| `delete_confirm_full_sequence` | Delete not called before confirm; called exactly once after; row removed; success toast |
| `delete_cancel_keeps_row` | Cancel in modal → delete never called; row retained; no error toast |
| `import_cancel_does_nothing` | Cancelled open dialog (None) → import not called; no error toast |
| `import_failure_shows_error_toast` | `fail_on("import_script_from_txt")` → error toast; no script added |
| `export_cancel_does_not_export` | Cancelled save dialog (None) → export not called; no error toast |
| `delete_failure_keeps_row_and_errors` | `fail_on("delete_script")` → error toast; row retained |

### Architecture assertions (verified by smoke-phase10.ps1)

1. **Animation loop is frontend-only.** `src/prompter/engine.rs` has zero tauri_api calls.
2. **Persistence calls are event-driven.** All save calls in `prompter_view.rs` are in setInterval or event handlers, not requestAnimationFrame.
3. **Toast position is non-obstructive.** CSS: `position: fixed; bottom: 20px; right: 20px`. Pointer-events delegated.
4. **Toast auto-dismiss uses setTimeout (4s)**, independent of animation frame.

### Manual fallback items (17)

Items requiring user interaction with the running app:
- App launch, script loading, Tauri-IPC feedback (save/import/export/delete toasts), visual confirmation of smooth playback, theme, native dialogs.
- Documented in `scripts/smoke-phase10.ps1` and `wiki/02-test-tree.md`.

## Tauri API Abstraction (implemented in Phase 11)

### Problem (pre–Phase 11)

Every component called `tauri_api::*` free functions directly, making it
impossible to test component logic without the full Tauri backend. WASM tests
cannot `invoke` Tauri commands.

### Solution

`src/bindings/` now layers the API:

| File | Role |
|------|------|
| `tauri_api.rs` | Raw `invoke` wrappers + typed data structs (`ScriptData`, `AppSettingsData`, `ScriptPlaybackStateData`). Unchanged behavior. |
| `app_api.rs` | `AppApi` trait (`#[async_trait(?Send)]`) over every command, plus `RealTauriApi` which delegates to the `tauri_api` free functions. |
| `mock_api.rs` | `MockApi` — in-memory `RefCell` store with builders (`with_scripts`, `with_settings`, `with_playback`, `with_open_dialog`, `with_save_dialog`, `with_file`), a call log (`was_called`/`was_not_called`/`call_count`), state snapshots (`scripts`, `exported`), and error injection — global `failing(msg)` or targeted `fail_on(cmd)`. `#[cfg(test)]` only, so it never ships in the production bundle. |
| `mod.rs` | `pub type ApiCtx = Rc<dyn AppApi>` — the Leptos context handle. |

- **Production** provides `Rc::new(RealTauriApi)` in `app.rs`.
- **Tests** provide `Rc::new(MockApi::...)`.
- Components fetch it with `use_context::<ApiCtx>()` and call `api.<command>().await`.
  Because `Rc<dyn AppApi>` is not `Copy`, handlers used in more than one place
  (or inside reactive `Fn` closures / `.map` over a list) are wrapped in a Leptos
  `Callback` (which is `Copy`); single-use handlers clone the `Rc` inline.

### Why `async-trait` and not `impl Future`

`AppApi` is used as a trait object (`dyn AppApi`) so it can live in Leptos
context. Trait objects are not object-safe with bare `async fn` / RPITIT, so we
use `async-trait` with `?Send` (the frontend is single-threaded WASM).

### Native dialogs in tests

Component tests never touch OS dialogs. `MockApi::open_file_dialog` /
`save_file_dialog` return preconfigured paths (`with_open_dialog` /
`with_save_dialog`), and `read_text_file` returns content seeded via `with_file`.
This lets import/export paths be exercised without a running shell (those flows
remain manual-only for now; see limitations).

### How this improves the Phase 10 smoke checklist

Five of the previously manual-only items are now automated end-to-end against a
mock backend instead of requiring a running Tauri app:

- script library load + render
- empty-library state
- settings load
- editor load of a selected script
- resume-playback dialog appearance

Save/import/export toasts and native-dialog round-trips still require manual
verification (they depend on real Tauri IPC / OS dialogs), but their *logic* is
now reachable through `MockApi` for future expansion.

## Running Tests

```bash
# Frontend WASM tests (requires Chrome)
wasm-pack test --headless --chrome

# Backend tests (no browser needed)
cargo test -p openprompter-rs-tauri

# Full validation
cargo fmt --all -- --check
cargo check -p openprompter-rs-tauri --all-targets
cargo test -p openprompter-rs-tauri
cargo clippy -p openprompter-rs-tauri --all-targets --all-features -- -D warnings
cargo check
trunk build
wasm-pack test --headless --chrome

# Phase 10 smoke script (includes architecture assertions)
.\scripts\smoke-phase10.ps1

# Full release build
cargo tauri build
```

## Known Limitations

- `wasm-pack test` needs Chrome installed. Ubuntu GitHub runners have it.
  Windows users need Chrome or can use `--headless --firefox`.
- AtomicU32 for toast IDs: works on WASM even without atomics proposal
  (Rust std provides fallback).
- CI wasm-pack install adds ~5 minutes to pipeline.
- Toast auto-dismiss cannot be tested in WASM (requires wall-clock setTimeout).
- Component tests settle async via a bounded `tick`/`settle` poll (≈6 × 10 ms),
  not a fixed sleep, to avoid flakiness. If a future component adds longer async
  chains, increase the `settle` rounds rather than the per-tick delay.
- Still manual-only (real Tauri IPC / OS dialogs): save/import/export toast
  round-trips, native file dialogs, smooth-playback visual confirmation, theme
  application in the real window.
