# Open Threads

## CI maintenance — RESOLVED in Phase 13

| Thread | Resolution |
|--------|-----------|
| **windows-latest → windows-2025** | `release.yml` Windows job pinned to `windows-2025` (explicit). No `windows-latest` left in any workflow. Verified via `workflow_dispatch` build. |
| **Node 24 action deprecation** | `softprops/action-gh-release` bumped v2 → v3 (Node 24). `Swatinem/rust-cache@v2` (no Node 24 release yet) bridged via workflow-level `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: true` in `ci.yml` + `release.yml` — remove the env once rust-cache ships a Node 24 major. `actions/checkout@v5` already Node 24. |
| **Phase/`v*` tags triggered release.yml** | `release.yml` push trigger narrowed to semver (`v[0-9]+.[0-9]+.[0-9]+` + `-beta.[0-9]+`); phase tags like `v0.13.0-phase13` no longer match. Plus `workflow_dispatch` added for build verification without publishing (upload steps tag-guarded). |

## Phase 11–12 (testing)

| Thread | Detail |
|--------|--------|
| **Still manual-only** | Real Tauri IPC / OS dialogs: save/import/export toast round-trips against the *real* backend, native file dialogs, smooth-playback visual confirmation, theme in the real window. MockApi covers the *logic* of these flows. |
| **Frontend clippy ungated** | CI runs clippy on backend only. Frontend wasm has baseline warnings (engine.rs complex type, speed.rs RangeInclusive, Default impls, clone-on-Copy, let-unit). Not a gate; don't add *new* ones gratuitously. |
| **Phase/`v*` tags trigger release.yml** | Tags like `v0.12.0-phase12` match `v*` → fire build-linux/macos/windows jobs (noise on PRs, not PR gates). Could restrict to `v[0-9]+.[0-9]+.[0-9]+`. |
| **`.dv-state.json` local-only** | Phase-close edits not pushed (direct main push blocked; PR-only). dv-state is local orchestration; remote copy reflects last merged phase's mid-state. |

## Carried over (still open)

| Thread | Detail |
|--------|--------|
| **Old Teleprompter repo kept private** | `JHenriquesss/Teleprompter` is private archive, not deleted. Contains rewritten history. Can be deleted after confirming migration is complete |
| **4 PR refs require GitHub Support** | `refs/pull/1/head` through `refs/pull/4/head` in old repo still reference pre-filter-repo commits. Cannot force-push from client. Need GitHub Support to purge |
| **Public repo now at OpenTeleprompter** | `JHenriquesss/OpenTeleprompter` is the new public home. Clean history, single commit, no old tags/PRs/releases |
| **Safe screenshots not yet regenerated** | Use synthetic demo content only. Blocked by `docs/screenshot-safety.md` checklist requirement |
| **wasm-bindgen-cli network dep** | trunk downloads wasm-bindgen CLI from GitHub on first build. 504 timeout when GitHub under load. Pre-install via `cargo install wasm-bindgen-cli --version 0.2.122` as workaround |
| **Non-ASCII Windows paths** | Cross-platform paths with Unicode break trunk's `canonicalize()` in watch-ignore scanning. Keep `Trunk.toml` watch.ignore empty |
| **Tray icon** | No system tray in current scope |
| **CI workflow** | `.github/workflows/ci.yml` runs on PR + push to main. Feature branches do not trigger CI. Phase 6 merged into main (040bce1); CI ran green on push |
| **Library refresh granularity** | `refresh_library()` re-fetches entire list on any mutation. Could optimize: insert/remove in-place on create/delete |
| **ScriptEditor stale read** | `perform_save()` reads signals at call time. Low risk (async gap ~ms) |
| **Cargo PDB filename collision warning** | `warning: output filename collision at ...\openprompter_rs_tauri.pdb` when running `cargo test`. lib + bin targets in same crate produce same PDB name. Harmless but noisy. Consider renaming bin target |

## Phase 5 deferrals

| Thread | Detail |
|--------|--------|
| **Screenshots freshness** | Regenerate screenshots with synthetic demo content before next public release. Deferred — no UI changes since Phase 5. Use `docs/screenshot-safety.md` checklist when doing so. |

## Phase 7/7.1 carry-overs

| Thread | Detail |
|--------|--------|
| **Unsigned Windows installers** | Authenticode signing not configured. Users see SmartScreen warning on download/install. Requires code signing certificate (paid, org validation). Will be addressed in a future phase. SmartScreen can be bypassed via "More info → Run anyway" |
| **Unsigned macOS builds** | Gatekeeper blocks unsigned app on first launch. Requires Apple Developer Program enrollment + notarization. Workaround: right-click → Open |
| **`cargo install tauri-cli` compile time** | Compiles from source on every cache-miss CI run. Adds ~10 min per platform. Could use `cargo binstall tauri-cli` for pre-built binary, or `tauri-apps/tauri-action` which handles CLI download and build in one step |
| **Linux AppImage large size** | 78 MB due to bundled webkit2gtk runtime. Could strip or compress, or document minimum size expectation |
| **Checksum file path cosmetics (Linux/macOS)** | SHA256SUMS files use build-directory paths. `sha256sum --check` requires same directory structure. Manual hash comparison is the primary workflow. Could fix with `sha256sum --basename` or `cd` + `sha256sum *.AppImage` |
| **First signed updater release** | Updater is wired (Phase 14) but no signed release is published yet. Needs repo secrets `TAURI_SIGNING_PRIVATE_KEY`/`_PASSWORD`, `bundle.createUpdaterArtifacts: true`, and a `latest.json` manifest on a public release. See docs/release.md → Auto-Update |
| **macOS Intel runner queue (`macos-13`)** | The Intel matrix leg is correct (parity with the green `macos-14` leg) but GitHub `macos-13` runners are scarce — a Phase 15 verification dispatch left it queued >70 min unscheduled. First real Intel DMG lands on the next tagged release; if Intel queues stay pathological, consider dropping Intel or building it only on tags |

## Resolved

### Phase 1a (stabilization)

| Thread | Resolution |
|--------|-----------|
| Export to file | Frontend Blob download |
| Animation loop leak | `Rc<Cell<bool>>` + `on_cleanup` |
| Keyboard listener leak | `on_cleanup` + `remove_event_listener_with_callback` |
| Unused imports/vars | 6 instances removed |
| Test location | Moved to `src-tauri/tests/` |
| Workspace config | `[workspace]` added to root |

### Phase 1.2 (real build validation)

| Thread | Resolution |
|--------|-----------|
| Compilation verification | Windows 10: Rust 1.96.0, trunk 0.21.14, tauri-cli 2.11.2 |
| Backend check + test + clippy | All green |
| Frontend trunk build | 58 compilation errors fixed |
| GUI launch | `cargo tauri dev` opens window |
| Icons | Generated via `npx @tauri-apps/cli icon` |
| Unused deps | `uuid` + `chrono` removed from frontend |
| wasm-bindgen-futures | Added to frontend deps |

### Phase 2 (Tauri command integration)

| Thread | Resolution |
|--------|-----------|
| Frontend cargo check | Fixed missing `SignalUpdate` import |
| Loading states | ScriptLibrary, ScriptEditor, SettingsPanel |
| Error display | Per-component inline errors |
| Stubs removed | All components call real backend |

### Phase 4 (production readiness)

| Thread | Resolution |
|--------|-----------|
| Delete confirmation UX | ConfirmModal component replaces window.confirm() |
| Native file dialogs | tauri-plugin-dialog v2.7.1 for import/export |
| Auto-save | Debounced 500ms auto-save with status indicator |
| Import stub | Native file dialog wired |

### Phase 5 (public UX polish)

| Thread | Resolution |
|--------|-----------|
| Theme toggle | CSS variables + toggle + persistence. Partial coverage (root container + scrollbars only) |
| Ctrl+S save shortcut | `on:keydown` on title + textarea, `perform_save()` closure |

### Phase 5.1 (CSS variable migration)

| Thread | Resolution |
|--------|-----------|
| Light theme usability | 25+ CSS vars, 6 components migrated (`sidebar`, `script_library`, `script_editor`, `settings_panel`, `confirm_modal`, `app_shell`). Prompter stays dark-first |
| Hardcoded colors | `#16213e`, `#e94560`, `#333`, `#555`, `#aaa`, `#e0e0e0` → `var()` in all components |

### Phase 6 (packaging and release)

| Thread | Resolution |
|--------|-----------|
| Windows installer | MSI + NSIS via `cargo tauri build`, verified locally and on CI |
| GitHub Release | `release.yml` workflow, `softprops/action-gh-release@v2`, assets uploaded to `v0.6.0-beta.1` |
| Version consistency | Bumped `0.1.0` → `0.6.0` across all metadata files + sidebar |
| Release docs | `docs/release.md` with build/release/test instructions |
| README releases | Releases section with badge + link to GitHub Releases |
| Installer test | Manual: install → launch → close → uninstall (all pass) |

### Phase 6.1 (merge into main + release verification)

| Thread | Resolution |
|--------|-----------|
| Phase 6 in main | Merged `--no-ff` into main (040bce1). No conflicts |
| Post-merge validation | fmt/check/14 tests/clippy/trunk/build — all green on main |
| `cargo tauri build` on main | 2m13s, MSI + NSIS produced |
| CI on main | Green (push event) |
| Public download test | MSI from GitHub Release, NSIS install+launch+uninstall — all pass |
| SmartScreen warning | Noted as expected for unsigned beta |

### Phase 7 (cross-platform packaging)

| Thread | Resolution |
|--------|-----------|
| Linux build in release.yml | Added `build-linux` job (ubuntu-latest): validation → `cargo tauri build` → AppImage + deb |
| macOS build in release.yml | Added `build-macos` job (macos-latest): validation → `cargo tauri build` → DMG |
| Windows job preserved | Unchanged: MSI + NSIS still generated and uploaded |
| Version bump | `0.6.0` → `0.7.0` across Cargo.toml, src-tauri/Cargo.toml, tauri.conf.json, sidebar.rs |
| Release tags | `v0.7.0-phase7` + `v0.7.0-beta.1` pushed; release workflow triggered for both |
| CI results | macOS 15m49s ✅, Linux 20m35s ✅, Windows 34m39s ✅ |
| Release artifacts | 5 assets on GitHub Release: MSI, NSIS, AppImage, deb, DMG |
| Docs updated | README.md (cross-platform download table), docs/release.md (platform table, unsigned instructions per platform) |

### Phase 7b (user-friendly distribution)

| Thread | Resolution |
|--------|-----------|
| Windows portable ZIP | `OpenPrompter.RS_portable_x64.zip` created in release.yml via `Compress-Archive` (EXE + README-INSTALL.txt); uploaded alongside MSI+NSIS |
| User-friendly install docs | `docs/install.md` — non-developer guide for all platforms with SmartScreen/Gatekeeper workarounds |
| README clarity | Restructured with "Download and Install" section above dev docs; explicitly states no account/cloud/payment |
| Portable readme | `docs/portable-readme.txt` bundled inside ZIP |
| All builds unsigned | Documented transparently in `docs/install.md` with "Is it safe?" section |
| Tags force-updated | `v0.7.0-phase7` + `v0.7.0-beta.1` moved to latest commit; release replaced with 6 artifacts |

### Phase 7.1 (release trust and install reliability)

| Thread | Resolution |
|--------|-----------|
| File integrity verification | Per-platform SHA256 checksum files generated and uploaded to GitHub Release alongside binaries |
| docs/install.md clarity | "Which file should I download?" table + checksum verification guide |
| docs/release.md completeness | Release notes template, checksums table, MSI 1603 investigation with docs |
| MSI reinstall error 1603 | Investigated, root cause documented (Windows Installer component table inconsistency), NSIS recommended for upgrades |
| README download section | Clearer recommendations per platform, current beta version linked |
| Screenshots freshness | Reviewed — still accurate (dark theme default). No update needed |

### Phase 8 (recording experience)

| Thread | Resolution |
|--------|-----------|
| Speed presets | 6 preset buttons (0.5×–3×) wired to `set_scroll_speed`. Keyboard 1–5 preserved |
| Pause markers | `parse_pause_markers()` returns `Vec<PauseMarker>`. Text highlighted red. Playback pauses at markers |
| Rehearsal mode | Toggle in ScriptLibrary with word count + estimated reading time |
| Recording-safe controls | Disabled controls during recording via `is_recording` flag |
| 3 Rust bugs | Fixed E0716, E0597, E0382 in prompter_view.rs |

### Phase 9 (recording continuity)

| Thread | Resolution |
|--------|-----------|
| Custom speed input | `<input type="number">` with validation, clamped 0.25–5.0, error toast |
| Resume playback | `ScriptPlaybackState` domain + SQLite table + resume dialog on re-entry |
| Per-script preferences | Speed, font, mirror restored from `script_playback_state` on resume |
| Jump controls | Arrow: ~5s, Shift+Arrow: ~20s. Visual toast feedback |
| Reset saved position | Reset button clears `script_playback_state`. R key resets scroll to top |
| 4 new backend tests | save/load/update/delete playback state |
| cargo fmt CI fix | 4 closures + chained method reformatted. --amend + force-push. Re-run green |

### Phase 10 (frontend reliability)

| Thread | Resolution |
|--------|-----------|
| Error handling UX | ToastState + ToastContainer: 4 levels, auto-dismiss 4s, centralized in all components. Jump feedback migrated to toasts. Settings inline errors removed |
| Frontend WASM test harness | 11 WASM tests running via `wasm-pack test --headless --chrome`. CI step added. Pure-logic modules (speed, mirror) tested |
| Component tests blocked | Component integration tests still blocked on Tauri API trait + mock refactor (docs/frontend-testing.md has roadmap, ~1250 lines estimate) |

### Phase 13 (CI runner & action maintenance)

| Thread | Resolution |
|--------|-----------|
| windows-latest → windows-2025 | `release.yml` Windows job pinned to `windows-2025`. Verified via `workflow_dispatch` run on main (build-windows green) |
| Node.js 20 action deprecation | `softprops/action-gh-release` v2→v3 (Node 24); `actions/checkout@v5` already Node 24; `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: true` bridges `Swatinem/rust-cache@v2`. No Node 20 warnings in verification run |
| Phase tags trigger release workflow | Push trigger narrowed to public semver (`v[0-9]+.[0-9]+.[0-9]+` + `-beta.[0-9]+`); phase tags no longer fire `release.yml`. Added `workflow_dispatch` for build verification without publishing |

### Phase 14 (self-update)

| Thread | Resolution |
|--------|-----------|
| Auto-update | `tauri-plugin-updater` wired: backend `check_for_update`/`install_update` (two-step via `PendingUpdate` state), `plugins.updater` config (pubkey + GitHub `latest.json` endpoint), `UpdateBanner` prompt. 6 WASM tests. First *signed release* deferred (see open threads above) |
| Updater signing key | Minisign keypair generated; public key embedded in `tauri.conf.json`, private key git-ignored + documented as GH secret. `release.yml` passes the secret through |

### Phase 15 (platform coverage)

| Thread | Resolution |
|--------|-----------|
| macOS Intel builds | `build-macos` matrixed over `macos-13` (native Intel x86_64) + `macos-14` (native aarch64). Per-arch DMG + `SHA256SUMS-macos-<arch>.txt`. No cross-compile |
| Linux RPM | Added `bundle/rpm/*.rpm` (from Tauri's `"all"` target, pure-Rust builder — no `rpmbuild`) to Linux checksum + upload globs. Fedora/RHEL/openSUSE covered |

### Phase 16 (system tray)

| Thread | Resolution |
|--------|-----------|
| Tray icon | `src-tauri/src/tray.rs` + `lib.rs`: tray icon with Show/Hide/Quit menu, left-click toggles window, X hides-to-tray (Quit = full exit), one-time hint toast via `event.once`. `tray-icon` cargo feature (no new crate). Pure `tray_action` mapper unit-tested; runtime manual-verified |
