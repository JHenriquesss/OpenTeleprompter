# Glossary

| Term | Meaning |
|------|---------|
| **Script** | A teleprompter document: id, title, content, created_at, updated_at |
| **AppSettings** | Persistent user preferences: font_size, line_height, text_width, scroll_speed, mirror_mode, theme, countdown_seconds, mirror_vertical, reading_guide_enabled |
| **PlaybackState** | Runtime prompter state: is_playing, scroll_y, speed |
| **UiState** | Frontend-only UI controls: font_size, line_height, text_width, mirror_mode, mirror_vertical, countdown_seconds, reading_guide, show_shortcut_help, theme |
| **Mirror mode** | CSS `scaleX(-1)` transform for beam-splitter teleprompter hardware |
| **Mirror vertical** | CSS `scaleY(-1)` transform for vertical flip of prompter text |
| **Prompter mode** | Fullscreen view where the script scrolls automatically |
| **View** | Leptos enum routing: `Library \| Editor \| Prompter \| Settings` |
| **ScriptData** | Frontend serializable mirror of backend Script: id, title, content, created_at, updated_at |
| **AppSettingsData** | Frontend serializable mirror of backend AppSettings: font_size, line_height, text_width, scroll_speed, mirror_mode, theme, countdown_seconds, mirror_vertical, reading_guide_enabled |
| **library_refresh_trigger** | `RwSignal<u32>` in AppState. Incremented after any mutation → ScriptLibrary re-fetches |
| **theme CSS variables** | `--bg-main`, `--bg-panel`, `--text-main`, `--text-muted`, `--accent`, `--border-color`, `--input-bg`, `--input-border`, `--button-primary-bg`, `--button-primary-text`, `--button-ghost-border`, `--button-ghost-text`, `--bg-overlay`, `--scrollbar-thumb`, `--scrollbar-track`, etc. Defined in `<style>` tag in AppShell. Applied via `.theme-dark` / `.theme-light` class on root div. Full component coverage (Phase 5.1). PrompterView excluded (dark-first) |
| **release.yml** | `.github/workflows/release.yml`. Builds Windows (windows-2025) MSI+NSIS, Linux AppImage+deb, macOS DMG; uploads to GitHub Release. Triggers: **public semver tags only** (`v[0-9]+.[0-9]+.[0-9]+` + `-beta.[0-9]+`) + `workflow_dispatch`. Since Phase 13 phase tags no longer match. Upload steps tag-guarded. [[03-phases.md#phase-13-ci-runner--action-maintenance]] |
| **phase tag** | Internal milestone tag, e.g. `v0.12.0-phase12`. Since Phase 13 does **NOT** match the narrowed `release.yml` trigger → no release build. Phases 13–14 created **no** phase tag at all |
| **release tag** | Public distribution tag, e.g. `v0.6.0-beta.1`. Matches release.yml semver trigger → artifacts uploaded to GitHub Release |
| **FORCE_JAVASCRIPT_ACTIONS_TO_NODE24** | Env (`true`) in ci.yml + release.yml. Bridges Node 20 JS actions (`Swatinem/rust-cache@v2`, no Node 24 release yet) onto the Node 24 runtime, clearing deprecation warnings. Remove once rust-cache ships Node 24 (Phase 13) |
| **updater (tauri-plugin-updater)** | In-app self-update plugin (Phase 14). Backend checks the endpoint, downloads + installs, relaunches. Config in `tauri.conf.json` `plugins.updater`. [[01-architecture.md#self-update-phase-14]] |
| **UpdateInfo** | Serializable update metadata sent to frontend: version, current_version, notes, date. Backend `Update` (from plugin) is NOT serializable → summarized into this |
| **PendingUpdate** | Backend managed state `Mutex<Option<Update>>`. Holds the non-serializable `Update` handle between `check_for_update` and `install_update` so the UI can prompt in between |
| **UpdateBanner** | Frontend component (`src/components/update_banner.rs`). Auto-checks on mount (silent when current, error toast on check-fail), renders Install/Dismiss when an update exists. Install user-initiated only — no silent auto-install. Wired into AppShell above sidebar+content; absent in fullscreen prompter |
| **updater pubkey / signing** | `tauri-plugin-updater` requires a **minisign** signature (mandatory, distinct from OS code-signing). Public key embedded in `tauri.conf.json` `plugins.updater.pubkey`; private key git-ignored (`.updater-keys/`) + GH secret `TAURI_SIGNING_PRIVATE_KEY` |
| **createUpdaterArtifacts** | `tauri.conf.json` `bundle` flag (currently off). When `true`, `cargo tauri build` emits signed updater bundles + `.sig`. Flip + add secret + upload `latest.json` to publish the first updater-enabled release |
| **latest.json** | Updater manifest at the endpoint (`releases/latest/download/latest.json`): version, notes, pub_date, per-platform `{signature, url}`. Not published yet (deferred) |
| **unsigned build** | Windows installer not Authenticode-signed. Users see SmartScreen warning. Acceptable for beta; planned for future phase |
| **SHA256SUMS** | Per-platform checksum files uploaded to GitHub Release. `SHA256SUMS-windows.txt` (MSI+NSIS+ZIP), `SHA256SUMS-linux.txt` (AppImage+deb+RPM), `SHA256SUMS-macos-x64.txt` + `SHA256SUMS-macos-aarch64.txt` (per-arch DMG). Generated via `Get-FileHash` (Windows), `sha256sum` (Linux), `shasum -a 256` (macOS). Per-arch macOS since Phase 15 |
| **macOS build matrix** | `release.yml build-macos` runs on `macos-13` (native Intel x86_64) + `macos-14` (native Apple Silicon aarch64), `fail-fast: false`. Each runner builds its own DMG natively — no cross-compile (Phase 15) |
| **Linux RPM** | `bundle/rpm/*.rpm` from Tauri v2 `"targets": "all"` (pure-Rust rpm builder, no system `rpmbuild`). Covers Fedora/RHEL/openSUSE. Added to Linux artifacts in Phase 15 |
| **MSI Error 1603** | Windows Installer fatal error on same-version reinstall. Root cause: Tauri's MSI has same product version on rebuild → component table inconsistency. Affects MSI only. NSIS and portable ZIP unaffected. Resolution: use NSIS for upgrades, MSI for clean installs only |
| **release notes template** | Standardized messaging in docs/release.md for every beta release: unsigned builds, no account required, no internet after download, platform-specific warnings, Windows as primary tested platform |
| **ScriptPlaybackState** | Domain entity holding per-script prompter state: script_id, scroll_offset_px, speed_multiplier, font_size, line_height, mirror_mode, mirror_vertical, updated_at. Stored in its own SQLite table with FK→scripts ON DELETE CASCADE |
| **ScriptPlaybackStateData** | Frontend serializable mirror of backend ScriptPlaybackState: all fields as Option<T> for partial updates |
| **Pause marker** | Embedded `[pause:N]` or `[breath]` in script content. Parsed into `PauseMarker{position: usize, duration_secs: f64}`. Playback pauses at marker position for specified duration. Text between markers highlighted in red |
| **Rehearsal mode** | ScriptLibrary action showing word count + estimated reading time without entering fullscreen playback |
| **Resume playback** | Periodic save (3s interval) of ScriptPlaybackState during playback. On re-entry, shows dialog with progress % and choice to resume or start over |
| **Jump controls** | Arrow Left/Right: ~5s jump. Shift+Arrow Left/Right: ~20s big jump. Visual toast feedback ("+5s", "-5s", "+20s", "-20s") with fade-out animation |
| **Speed preset** | One-click speed buttons in floating controls: 0.5×, 1×, 1.5×, 2×, 3×. Keyboard shortcuts 1–5 also work |
| **Custom speed input** | `<input type="number">` in floating controls. `validate_speed()` clamps to 0.25–5.0. Shows friendly error on invalid input |
| **ToastState** | Leptos context: `RwSignal<Vec<ToastMessage>>`. Global unique IDs via `AtomicU32` static. 4 add methods: `add_success`, `add_error`, `add_warning`, `add_info`. Auto-dismiss 4s via `setTimeout` + `Closure::once`. Provided in `App::new()`, consumed via `expect_context::<ToastState>()`. [[src/state/toast.rs]] |
| **ToastContainer** | Leptos component. Renders fixed bottom-right, `z-index: 1000`, `pointer-events: none` container with `auto` on each card. Uses `<For>` for reactive list. Animation `toastIn` (0.2s ease-out). Close button per card. [[src/state/toast.rs]] |
| **ToastLevel** | Enum: `Success`, `Error`, `Warning`, `Info`. Each has CSS class (`toast-success`, `toast-error`, `toast-warning`, `toast-info`) and icon character. [[src/state/toast.rs]] |
| **wasm-bindgen-test** | Dev-dependency for frontend WASM unit tests. Tests annotated with `#[wasm_bindgen_test::wasm_bindgen_test]`. Run via `wasm-pack test --headless --chrome`. 11 tests total (5 speed + 6 mirror). [[src/prompter/speed.rs]] [[src/prompter/mirror.rs]] |
| **WASM frontend tests** | 11 tests running in headless Chrome via `wasm-pack test --headless --chrome`. All pure-logic (no DOM, no Tauri IPC). Tests: speed (validate_speed, speed_label, presets, word_count, estimated_reading_seconds), mirror (mirror_transform 2 combos, mirror_transform_combined 4 combos). CI step runs after `trunk build`. |
