# Frontend Testing Guide

## Overview

The frontend crate (`openprompter-rs`) is a Leptos CSR app compiled to WASM.
Testing is split into three categories:
1. **Pure-logic WASM tests** (run in browser via `wasm-pack test`)
2. **Architecture assertions** (automated source inspection)
3. **Manual fallback** (requires user interaction in running app)

## What We Test Today (Phase 10)

All WASM tests use `wasm_bindgen_test` and run in a headless browser:

```
wasm-pack test --headless --chrome
```

### Tested modules (41 tests)

| Module | File | Tests | What they cover |
|--------|------|-------|-----------------|
| `speed` | `src/prompter/speed.rs` | 5 | validate_speed, speed_label, presets, word_count, estimated_reading_seconds |
| `mirror` | `src/prompter/mirror.rs` | 6 | mirror_transform, mirror_transform_combined (4 combos) |
| `toast` | `src/state/toast.rs` | 8 | add_success/error/warning/info, dismiss, unique ids, css_class, icon |
| `playback_state` | `src/state/playback_state.rs` | 14 | initial state, toggle_play, restart, speed up/down/set, jump (small/big), clamps |
| `ui_state` | `src/state/ui_state.rs` | 8 | initial state, font size up/down/clamps, toggle mirror/reading_guide/shortcut_help |

### Architecture assertions (verified by smoke-phase10.ps1)

1. **Animation loop is frontend-only.** `src/prompter/engine.rs` has zero tauri_api calls.
2. **Persistence calls are event-driven.** All save calls in `prompter_view.rs` are in setInterval or event handlers, not requestAnimationFrame.
3. **Toast position is non-obstructive.** CSS: `position: fixed; bottom: 20px; right: 20px`. Pointer-events delegated.
4. **Toast auto-dismiss uses setTimeout (4s)**, independent of animation frame.

### Manual fallback items (17)

Items requiring user interaction with the running app:
- App launch, script loading, Tauri-IPC feedback (save/import/export/delete toasts), visual confirmation of smooth playback, theme, native dialogs.
- Documented in `scripts/smoke-phase10.ps1` and `wiki/02-test-tree.md`.

## Tauri API Abstraction Roadmap

### Problem

Every component calls `tauri_api::invoke_tauri` directly, making it impossible
to test any component logic without the full Tauri backend running. WASM tests
cannot invoke Tauri commands.

### Solution (suggested for Phase 11+)

1. **Define a trait** in `src/bindings/tauri_trait.rs`:

```rust
#[cfg(not(test))]
pub use real::TauriApiProvider;
#[cfg(test)]
pub use mock::TauriApiProvider;

pub trait TauriApiProvider: Clone + 'static {
    fn get_script(&self, id: &str) -> impl Future<Output = Result<Script, String>>;
    fn list_scripts(&self) -> impl Future<Output = Result<Vec<ScriptListItem>, String>>;
    fn create_script(&self, title: &str, content: &str) -> impl Future<Output = Result<Script, String>>;
    fn update_script(&self, id: &str, title: &str, content: &str) -> impl Future<Output = Result<Script, String>>;
    fn delete_script(&self, id: &str) -> impl Future<Output = Result<(), String>>;
    fn duplicate_script(&self, id: &str) -> impl Future<Output = Result<Script, String>>;
    fn search_scripts(&self, query: &str) -> impl Future<Output = Result<Vec<ScriptListItem>, String>>;
    fn open_file_dialog(&self) -> impl Future<Output = Result<Option<String>, String>>;
    fn save_file_dialog(&self) -> impl Future<Output = Result<Option<String>, String>>;
    fn read_text_file(&self, path: &str) -> impl Future<Output = Result<Option<String>, String>>;
    fn get_settings(&self) -> impl Future<Output = Result<AppSettingsData, String>>;
    fn update_settings(&self, s: &AppSettingsData) -> impl Future<Output = Result<(), String>>;
    fn reset_settings(&self) -> impl Future<Output = Result<AppSettingsData, String>>;
    fn export_script_to_txt_file(&self, id: &str, path: &str) -> impl Future<Output = Result<(), String>>;
    fn import_script_from_txt(&self, content: &str, name: &str) -> impl Future<Output = Result<Script, String>>;
    fn save_playback_state(&self, ...) -> ...;
    fn load_playback_state(&self, id: &str) -> ...;
    fn clear_playback_state(&self, id: &str) -> ...;
    fn get_app_version(&self) -> ...;
}
```

2. **Real impl** wraps current `tauri_api::invoke_tauri`.
3. **Mock impl** returns canned data from in-memory storage.
4. **Provide via context** (`provide_context`) — components call
   `expect_context::<Box<dyn TauriApiProvider>>()`.
5. In tests, provide the mock impl; in production, provide the real impl.

### Effort estimate

- ~120 lines trait definition
- ~80 lines real impl (mostly delegates to existing functions)
- ~300 lines mock impl (HashMap-based in-memory store)
- ~50 lines per-component test (14 components -> ~700 lines)
- Total: ~1250 lines new code

### Priority (Phase 11+)

1. Trait + mock for `Script` CRUD (highest value: can test library + editor)
2. Trait + mock for `Settings` (can test settings panel)
3. Trait + mock for `PlaybackState` persistence (can test resume dialog)
4. Trait + mock for file dialogs (can test import/export)
5. Component integration tests for the 4 main views

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
- Component integration tests remain blocked on Tauri API trait + mock
  (see roadmap above). 17 of 19 smoke items require manual verification
  due to Tauri IPC or native OS dialog dependency.
