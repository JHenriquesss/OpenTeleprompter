# Test Tree

## Trunk path

End-to-end user journey:
1. App launches → ScriptLibrary shows empty state
2. User clicks "+ New Script" → Script created via Tauri command
3. User edits title + content in ScriptEditor → Save
4. User clicks "Open in Prompter" → PrompterView opens
5. User presses Space → smooth scroll animates
6. User adjusts speed → scroll rate changes
7. User presses Esc → returns to Library
8. User searches → filtered results
9. User duplicates → "(Copy)" appears
10. User deletes → removed from list
11. User opens Settings → adjusts sliders → values persist

## Phase 1 tests

| Test file | What it covers |
|-----------|---------------|
| `src-tauri/tests/domain_tests.rs` | Script/Settings serde roundtrip, defaults validation, field access |
| `src-tauri/tests/persistence_tests.rs` | SQLite CRUD, update, delete, count, search LIKE, settings save/load |
| `src-tauri/tests/import_export_tests.rs` | File read/write, title extraction from filename |

All tests use in-memory SQLite (persistence_tests) or temp files (import_export_tests). No mocked DB. Run with `cargo test` (workspace root) or `cargo test --manifest-path src-tauri/Cargo.toml`.

### Phase 2 notes

No new automated tests. The 13 typed Tauri wrapper functions (`bindings/tauri_api.rs`) are exercised through the manual trunk-path checklist (Steps 2–11) and all compile in `cargo check` + `trunk build`. Automated WASM-in-browser tests could be added in a future phase (e.g. `wasm-pack test --chrome`).

### Phase 5 trunk additions

Extended trunk path with:

12. User changes theme in Settings → theme persists across restart
13. User presses Ctrl+S in Editor → script saves
14. User searches with no match → search-specific empty state displays
15. Library empty state shows inline "+ New Script" button
16. Scrollbar renders with themed color (dark/light)

No new automated tests. WASM test infrastructure investigated in `docs/frontend-testing.md`: `wasm-bindgen-test` viable for pure-logic modules (PlaybackState, UiState, prompter speed/mirror/keyboard), but blocked for components that call Tauri APIs. Recommendation: refactor Tauri API into trait + mock impl, then add `wasm-pack test --headless --chrome` to CI.

### Phase 8 trunk additions

Extended trunk path with:

17. Speed preset buttons (0.5x, 1x, 1.5x, 2x, 3x) change scroll rate
18. Script with `[pause:3]` marker pauses playback for 3 seconds
19. Script with `[breath]` marker pauses playback for ~1s (highlighted red in text)
20. Rehearsal mode launches from script library with word count + estimated reading time

### Phase 8 backend tests

| Test file | What it covers |
|-----------|---------------|
| `src-tauri/tests/persistence_tests.rs` | Added 6 more tests: settings save/load, script CRUD expanded (total: 10 persistence tests) |

### Phase 9 trunk additions

Extended trunk path with:

21. Custom speed input (type any value 0.25x-5.0x) — validates, clamps, shows errors
22. Resume dialog on re-entry — shows progress %, "Resume" or "Start from beginning" choice
23. Shift+Arrow Left/Right — big jump ~20s with visual toast feedback
24. Reset button clears saved position
25. R key resets scroll to top
26. Per-script speed, font, mirror preferences restored on resume

### Phase 9 backend tests

| Test file | What it covers |
|-----------|---------------|
| `src-tauri/tests/persistence_tests.rs` | Added 4 tests: save/load/update/delete `script_playback_state` (total: 14 persistence tests) |

### Phase 10 trunk additions

Extended trunk path with:

27. User error-toasts show on failed save/import/export/delete
28. User success-toasts show on successful save/export/delete/duplicate
29. Jump feedback appears as bottom-right toast (not centered overlay)
30. Toast auto-dismisses after 4s, can be dismissed manually via X

## Phase 10 smoke-test tree

Phase 10 items classified by testability:

| # | Item | Classification | Notes |
|---|------|---------------|-------|
| **A. App lifecycle** | | | |
| 1 | App launches | Manual fallback | Requires OS-level window launch (cannot automate without Tauri driver) |
| 2 | Existing scripts load | Manual fallback | Requires live backend + DB with seed data |
| **B. Toast feedback** | | | |
| 3 | Toast on manual save | Manual fallback | Requires Tauri IPC (save_script command) |
| 4 | Auto-save feedback | Manual fallback | Requires Tauri IPC + timer observation |
| 5 | Import success/error toast | Manual fallback | Requires native file dialog + Tauri IPC |
| 6 | Export success/error toast | Manual fallback | Requires native file dialog + Tauri IPC |
| 7 | Delete success/error toast | Manual fallback | Requires Tauri IPC + confirmation dialog |
| 8 | Invalid speed shows toast | Manual fallback | Form validation tooltip is inline (not toast) per Phase 10 decision |
| 9 | Resume/reset feedback | Manual fallback | Requires Tauri IPC + resume dialog state machine |
| 10 | Jump feedback as toast | Manual fallback | Requires keyboard event + DOM render |
| **C. Safety & UX** | | | |
| 11 | Delete confirmation before deletion | Manual fallback | Requires native dialog (cannot mock without Tauri IPC) |
| 12 | Toasts not obstructing reading area | Architecture assertion | Toast CSS: fixed bottom-right (20px), pointer-events none on container. Verified in `src/state/toast.rs` line 103-107 |
| 13 | Theme support still works | Manual fallback | Requires CSS rendering + user interaction |
| 14 | Native dialogs still work | Manual fallback | Requires OS dialog invocation |
| **D. Prompter behavior** | | | |
| 15 | Prompter playback smooth | Manual fallback | Requires visual rendering + timing observation |
| 16 | Resume playback still works | Manual fallback | Requires Tauri IPC + resume UI state machine |
| 17 | Speed presets still work | Manual fallback | Requires keyboard/click + DOM render |
| 18 | Custom speed input still works | Manual fallback | Requires text input + DOM render |
| 19 | No backend calls during animation frames | Architecture assertion | Verified: `engine.rs` uses no tauri_api. All persistence calls are in event-driven effects or setInterval, not requestAnimationFrame |

### Phase 10 WASM tests

`wasm-pack test --headless --chrome` runs these in a headless browser:

| Module | File | Tests | What they cover |
|--------|------|-------|-----------------|
| `speed` | `src/prompter/speed.rs` | 5 | `validate_speed` (range, NaN, inf, boundary), `speed_label` (5 thresholds), `speed_presets` (4 entries), `word_count` (empty, normal, leading/trailing spaces), `estimated_reading_seconds` (0 words, 130 words at 130 wpm = 60s) |
| `mirror` | `src/prompter/mirror.rs` | 6 | `mirror_transform` (enabled/disabled), `mirror_transform_combined` (4 combos: none, horizontal, vertical, both) |
| `toast` | `src/state/toast.rs` | 8 | `add_success`, `add_error`, `add_warning`, `add_info`, `dismiss`, `unique_ids`, `css_class`, `icon` |
| `playback_state` | `src/state/playback_state.rs` | 14 | Initial state, toggle_play, restart, increase/decrease speed, set_speed, jump_forward/backward (small+big), clamps (max scroll, min scroll, max speed, min speed) |
| `ui_state` | `src/state/ui_state.rs` | 8 | Initial state, increase/decrease font size, font clamps (max/min), toggle_mirror, toggle_mirror_vertical, toggle_reading_guide, toggle_shortcut_help |

All tests use `wasm_bindgen_test::wasm_bindgen_test` attribute. Speed tests need no signal setup (pure functions). Mirror/toast/playback/ui tests use `create_rw_signal` + `RwSignal` for signal inputs. Run via `wasm-pack test --headless --chrome`. [[docs/frontend-testing.md]]

## Architecture assertions

1. **Animation loop is frontend-only.** `src/prompter/engine.rs` — `start_scroll_loop` reads `is_playing` and `speed` signals, writes `scroll_y`. No tauri_api calls. No IPC. No backend persistence.
2. **Persistence calls are event-driven.** `src/components/prompter_view.rs` saves via: (a) 3-second `setInterval` during playback, (b) on-pause effect, (c) exit button handler. None of these run inside `requestAnimationFrame`.
3. **Toast position is non-obstructive.** `src/state/toast.rs` lines 103-107: `position: fixed; bottom: 20px; right: 20px; z-index: 1000; pointer-events: none` on container. Individual cards have `pointer-events: auto` for dismissal. Position avoids the prompter reading line (centered/near-top).
4. **Toast auto-dismiss uses setTimeout (4s), independent of animation frame.**

## Summary

| Category | Count |
|----------|-------|
| Backend tests | 18 (5 domain + 3 import_export + 10 persistence) |
| Frontend WASM tests | 41 (5 speed + 6 mirror + 8 toast + 14 playback + 8 ui_state) |
| Architecture assertions | 4 (animation, persistence, toast position, toast timer) |
| Manual fallback items | 17 (documented in smoke-phase10.ps1) |
| **Total tests** | **59 (18 backend + 41 WASM)** |
