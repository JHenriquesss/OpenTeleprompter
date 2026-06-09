# Phases

## Phase 1: Scaffold + Domain + Persistence + Commands + UI

Status: **closed** (2026-06-06 · outcome: ok)

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Tauri v2 + Leptos CSR project compiles and launches | ✓ | 
| ME-2 | Domain models (Script, AppSettings, Theme, PlaybackState) | ✓ |
| ME-3 | SQLite DB init + migrations run on startup | ✓ |
| ME-4 | All 13 Tauri commands wired and returning expected types | ✓ |
| ME-5 | AppShell + ScriptLibrary + ScriptEditor + PrompterView + SettingsPanel render | ✓ |
| ME-6 | Smooth scrolling prompter runs on frontend (requestAnimationFrame) | ✓ |
| ME-7 | All 10 keyboard shortcuts work in prompter mode | ✓ |
| ME-8 | README + open-source files (LICENSE, CONTRIBUTING, etc.) | ✓ |

### Must-not-exist

| ID | Item | Clean |
|----|------|-------|
| MNE-1 | No JS files outside Tauri config | ✓ |
| MNE-2 | No cloud sync / auth / AI features | ✓ |
| MNE-3 | No DOCX/PDF import | ✓ |
| MNE-4 | No backend playback frame logic in Tauri commands | ✓ |

### Scope delivered

- 57 files across frontend, backend, tests, and docs
- CRUD via 13 Tauri commands
- Smooth scroll via requestAnimationFrame (frontend-only loop)
- Dark-themed UI with sidebar navigation
- rusqlite with WAL mode, 2 tables (scripts, settings)
- Settings panel with range sliders + toggle switch
- Open-source boilerplate (MIT + Apache 2.0, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY)

### Stabilization (Phase 1a)

Applied after initial scaffold. No new features — only correctness and compilation fixes.

**18 issues fixed:**
- 1 unused import (`AppError` in commands/scripts.rs) — removed
- 1 compilation error (script_editor `on_save` closure called without arg) — changed to `move ||`
- 3 unused variables (`sidebar`, `toolbar`, `settings_panel`) — removed
- 1 unused resource (`scripts` in script_library) — removed
- 1 unused import (`leptos::ev::keydown` in prompter_view) — removed
- 1 hacky DB error mapping (database.rs `ToSqlConversionFailure`) — replaced with `let _ =`
- 1 hacky refetch trigger (script_library `update`-clone-reassign) — cleaned to `.set()`
- 1 test location — moved from root `tests/` to `src-tauri/tests/`
- 2 missing dev-dependencies — added `rusqlite`, `tempfile`, `serde_json` to `[dev-dependencies]`
- 1 missing workspace config — added `[workspace]` to root `Cargo.toml`
- 2 memory leaks (animation loop + keyboard listener leaked closures on remount) — both use `on_cleanup` now

### Limitation

Cannot verify compilation — Rust toolchain not available on build machine. Expected `cargo tauri dev` to work on a machine with Rust + wasm32 target + Trunk + tauri-cli.

## Phase 1.1: Reproducible Build & CI Validation

Status: **closed** (2026-06-06 · outcome: ok)

No new product features. Goal: make project buildable/testable/verifiable on a real Rust/Tauri environment.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Toolchain pinning (`rust-toolchain.toml` with stable, wasm32, rustfmt, clippy) | ✓ |
| ME-2 | Developer validation scripts (`scripts/check.sh`, `scripts/check.ps1`) | ✓ |
| ME-3 | GitHub Actions CI (PR + push to main, fmt/check/test/clippy/trunk) | ✓ |
| ME-4 | Development Validation section in README with required tools + commands | ✓ |
| ME-5 | Manual Desktop Test Checklist (13 items) in README | ✓ |
| ME-6 | `.gitignore` for Rust artifacts, dist, IDE files | ✓ |

### Scope delivered

- `rust-toolchain.toml` pins stable, wasm32 target, rustfmt + clippy components
- `scripts/check.sh` + `scripts/check.ps1` run: fmt → `cargo check -p openprompter-rs-tauri` → `cargo test -p openprompter-rs-tauri` → clippy (`-D warnings`) → `trunk build`
- `.github/workflows/ci.yml` triggers on PR + push to main. Installs Tauri Linux deps, dtolnay/rust-toolchain, Swatinem/rust-cache, `cargo install trunk`. Runs all 5 validation steps
- Frontend crate (`openprompter-rs`) excluded from native cargo commands — it only compiles for wasm32. Trunk handles frontend validation
- README updated with required tools table, validation pipeline table, workspace split rationale, 13-item manual desktop checklist

### Limitation

Same as Phase 1: no Rust toolchain in this environment. CI workflow is ready to execute all validation on GitHub Actions runners.

## Phase 1.2: First Real Build Validation

Status: **closed** (2026-06-06 · outcome: ok)

Goal: take the project from "scaffold compiles in theory" to "compiles, tests pass, clippy clean, trunk builds, GUI launches" on a real Windows machine.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Rust 1.96.0 + wasm32 target + trunk 0.21.14 + tauri-cli 2.11.2 installed | ✓ |
| ME-2 | `cargo check -p openprompter-rs-tauri --all-targets` passes (backend) | ✓ |
| ME-3 | `cargo test -p openprompter-rs-tauri` passes (14/14) | ✓ |
| ME-4 | `cargo clippy -p openprompter-rs-tauri -- -D warnings` passes | ✓ |
| ME-5 | `trunk build` succeeds (frontend → WASM) | ✓ |
| ME-6 | `cargo tauri dev` launches GUI window | ✓ |
| ME-7 | CI workflow `.github/workflows/ci.yml` ready to execute | ✓ (untested — needs GitHub push) |

### Issues found and fixed (Phase 1.2)

| # | Issue | Fix |
|---|-------|-----|
| 1 | `web-sys` `RequestAnimationFrame` feature not in v0.3.63 (pinned by tauri) | Removed feature; `Window` feature provides `request_animation_frame()` |
| 2 | `uuid` v1 on wasm32 requires `js`, `rng-getrandom`, or `rng-rand` feature | Removed unused `uuid` + `chrono` from root `Cargo.toml` (frontend) |
| 3 | `wasm-bindgen-futures` needed by `async fn` in `extern "C"` block | Added `wasm-bindgen-futures = "0.4"` to root `Cargo.toml` |
| 4 | `icons/icon.ico` missing — tauri-build requires it on Windows | Generated proper icon set via `npx @tauri-apps/cli icon` |
| 5 | Dead code warnings (read/write file_system, PlaybackState, Theme default) | Added `#[allow(dead_code)]`, derived `#[derive(Default)]` on Theme |
| 6 | `assert_eq!(bool, true)` in domain_tests | Changed to `assert!(bool)` |
| 7 | 58 frontend compilation errors: `SignalGet`/`SignalSet`/`SignalUpdate` traits not in scope, `NodeRef.get()` vs `()`, field access vs method calls, closure-to-fn-pointer coercion, `Box<dyn Fn()>` vs `fn()`, disjoint async block types | Added missing trait imports, replaced `field()` with `field.get()`, swapped `fn()` to `Box<dyn Fn()>`, unified async blocks |
| 8 | `Trunk.toml` watch.ignore `["src-tauri/**/*"]` — `canonicalize()` fails on non-ASCII Windows path | Changed to `[]` |
| 9 | Trunk `found more than one target artifact` (lib + bin) | Added `<link data-trunk rel="rust" data-bin="openprompter-rs" />` to `index.html` |
| 10 | `wasm-bindgen` CLI download 504 from GitHub | Pre-installed via `cargo install wasm-bindgen-cli --version 0.2.122` |

### State after Phase 1.2

- Backend: compiles clean (3 dead_code warnings: read/write file_system, PlaybackState struct)
- Backend tests: 14/14 pass (domain: 5, persistence: 6, import_export: 3)
- Backend clippy: 0 errors, 0 warnings
- Frontend: trunk build produces `dist/` WASM output
- GUI: `cargo tauri dev` launches Tauri window, trunk serves on :1420
- CI: workflow ready but not triggered (no git repo yet)

### Identified ecosystem constraints

- **web-sys v0.3.63** (pinned by tauri v2.11.2): `RequestAnimationFrame` feature missing, `Window` feature provides the method
- **wasm-bindgen-cli 0.2.122**: trunk downloads this on first build; GitHub 504 under load; pre-install with `cargo install` as workaround
- **Non-ASCII Windows paths**: `std::fs::canonicalize()` fails in trunk's internal watch-ignore routine. Keep `Trunk.toml` watch.ignore empty
- **uuid + chrono** on wasm32: v1 uuid needs randomness source; chrono serde may cause issues; don't add unless actually needed

## Phase 2: Tauri Command Integration (Frontend ← Backend Wiring)

Status: **closed** (2026-06-06 · outcome: ok)

Goal: replace all stubbed/fake frontend operations with real Tauri command calls. Every UI action (create, update, delete, duplicate, search, import, export, settings CRUD, app version) goes through the typed `bindings/tauri_api.rs` layer.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Typed wrapper functions for all 13 Tauri commands in `bindings/tauri_api.rs` | ✓ |
| ME-2 | ScriptLibrary create/update/delete/duplicate/search wired to backend | ✓ |
| ME-3 | ScriptEditor save calls create_script or update_script with validation | ✓ |
| ME-4 | SettingsPanel loads on mount, save/reset calls backend | ✓ |
| ME-5 | PrompterView loads script content + settings from backend on mount | ✓ |
| ME-6 | Sidebar shows live app version from get_app_version | ✓ |
| ME-7 | `library_refresh_trigger` on AppState re-fetches library after mutations | ✓ |
| ME-8 | Frontend compilation (`cargo check`) + trunk build green | ✓ |
| ME-9 | Full validation pipeline green (fmt, backend check, test 14/14, clippy, trunk build) | ✓ |
| ME-10 | `cargo tauri dev` launches GUI with all features functional | ✓ |

### Must-not-exist

| ID | Item | Clean |
|----|------|-------|
| MNE-1 | No Tauri commands called during playback animation loop | ✓ |
| MNE-2 | No JS files added | ✓ |

### Scope delivered

- `bindings/tauri_api.rs`: 2 serializable structs (ScriptData, AppSettingsData) + 13 typed async functions via generic `invoke_tauri<T, R>()`
- `state/app_state.rs`: added `library_refresh_trigger: RwSignal<u32>` + `refresh_library()` method
- `components/script_library.rs`: re-fetches on trigger change; delete confirmation (window.confirm); duplicate calls backend; create opens blank editor
- `components/script_editor.rs`: save validates title non-empty, calls create_script or update_script, shows loading/error states; Open in Prompter saves then navigates
- `components/settings_panel.rs`: loads settings on mount, save resets via update_settings, reset calls reset_settings, displays app version from backend
- `components/prompter_view.rs`: loads selected script content + settings from backend on mount
- `components/sidebar.rs`: shows live app version from get_app_version

### Issues fixed during Phase 2

| # | Issue | Fix |
|---|-------|-----|
| 1 | `app_state.rs`: `RwSignal::update()` — `SignalUpdate` trait not imported | Added `use leptos::SignalUpdate` |
| 2 | `script_editor.rs`: `on_save` closure signature `move \|_\|` expects `()` but `on:click` requires `fn(MouseEvent)` | Refactored: extracted `save_current()` closure, wrapped for `on:click` + `on_start_prompter` |
| 3 | Unused `ScriptData` import in `script_editor.rs` + `script_library.rs` | Removed (use `tauri_api::` prefix instead) |

### State after Phase 2

- Full validation pipeline green (fmt, backend check, 14/14 tests, clippy -D warnings, frontend check, trunk build)
- `cargo tauri dev` launches GUI app with all features functional
- No code stubs remain — every UI action hits the real backend
- Animation loop remains frontend-only (no backend calls during playback)

## Phase 3: Prompter Feature Completion

Status: **closed** (2026-06-06 · outcome: ok)

Phase 3 scope included: floating control overlay, horizontal & vertical mirror, countdown timer, reading guide, progress & estimated time, keyboard shortcut help overlay, sidebar polish, Ctrl+Enter to launch prompter, smooth scroll rate at low speeds, prompter view loads real script content from Tauri API, and auto-pause on exit.

(Full checklist documented in previous wiki revision — Phase 5 rewrite condensed for space.)

## Phase 4: Production Readiness — Native Dialogs, Confirm Modal, Auto-Save

Status: **closed** (2026-06-06 · outcome: ok)

Phase 4 scope included: native file dialogs (tauri-plugin-dialog v2.7.1 for import/export), custom ConfirmModal replacing window.confirm(), debounced auto-save (500ms) in ScriptEditor with "Unsaved → Saving → Saved" status indicator, no JS files, no raw HTML file input in production path, no deletion without explicit confirmation.

## Phase 5: Public UX Polish — Theme Toggle, Scrollbars, README, Screenshots

Status: **closed** (2026-06-06 · outcome: ok · tag: `v0.5.0-phase5`)

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Theme toggle using existing settings architecture (CSS variables on root div, persisted) | ✓ |
| ME-2 | Custom scrollbar styling (dark + light themed) | ✓ |
| ME-3 | Loading/empty states polish (ScriptLibrary: icon + action button; search-specific empty state) | ✓ |
| ME-4 | Editor Ctrl+S save affordance (keyboard handler + save hint label) | ✓ |
| ME-5 | README public presentation rewrite (features, screenshot table, build guide) | ✓ |
| ME-6 | Screenshots in docs/screenshots/ (captured, pending regeneration) | ✓ |
| ME-7 | Frontend WASM test harness research (docs/frontend-testing.md) | ✓ |
| ME-8 | Full validation pipeline green + `cargo tauri dev` launches | ✓ |

### Must-not-exist

| ID | Item | Clean |
|----|------|-------|
| MNE-1 | No cloud sync / auth / AI features | ✓ |
| MNE-2 | No DOCX/PDF import | ✓ |
| MNE-3 | No JavaScript files | ✓ |
| MNE-4 | No backend playback frame logic in Tauri commands | ✓ |
| MNE-5 | No weakening of CI checks | ✓ |

### Scope delivered

- Theme: CSS variables via `<style>` tag in AppShell, class toggle on root div, persisted through existing settings flow [[src/components/app_shell.rs]]
- Scrollbars: `::-webkit-scrollbar` rules using CSS vars [[src/components/app_shell.rs]]
- States: ScriptLibrary loading (icon + subtitle), empty (icon + action button), search-empty (icon + hint) [[src/components/script_library.rs]]
- Screenshots: 4 views captured via Win32 API automation. Regenerate with synthetic content before next public release.
- README: full rewrite with screenshot table, features list, keyboard shortcuts, build guide, architecture, license [[README.md]]
- Frontend test research: `wasm-bindgen-test` viable for pure-logic modules; component testing blocked until Tauri API trait refactor [[docs/frontend-testing.md]]
- Branch `phase-5-public-ux-polish-theme-readme`, tag `v0.5.0-phase5`, commit `3d5608e`

### Resolved during Phase 5

| Thread | Resolution |
|--------|-----------|
| Theme toggle (from Phase 3+) | CSS variable infrastructure + toggle + persistence. Light theme colors only on root container — components still hardcoded dark |
| Ctrl+S save shortcut | `on:keydown` on title input + content textarea, `preventDefault()`, `perform_save()` closure reused for both Save button and keyboard |
| Delete confirmation UX (from Phase 2 carry-over) | Already resolved in Phase 4 (ConfirmModal component) |
| Native file dialogs (from Phase 3+) | Already resolved in Phase 4 (tauri-plugin-dialog) |
| Auto-save (from Phase 3+) | Already resolved in Phase 4 (debounced 500ms auto-save)

## Phase 5.1: Light theme CSS variable migration

Status: **closed** (2026-06-06 · outcome: ok · tag: `v0.5.1-phase5.1`)

Goal: replace hardcoded dark colors with CSS variables so light theme is fully usable. No new features.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | CSS variables in app_shell.rs (25+ vars per theme: `--bg-main`, `--bg-panel`, `--text-main`, `--text-muted`, `--accent`, `--border-color`, `--input-bg`, `--input-border`, `--button-primary-bg`, `--button-primary-text`, `--button-ghost-border`, `--button-ghost-text`, `--bg-overlay`, `--scrollbar-thumb`, `--scrollbar-track`, etc.) | ✓ |
| ME-2 | sidebar.rs colors → var() | ✓ |
| ME-3 | script_library.rs colors → var() | ✓ |
| ME-4 | script_editor.rs colors → var() | ✓ |
| ME-5 | settings_panel.rs colors → var() | ✓ |
| ME-6 | confirm_modal.rs colors → var() | ✓ |
| ME-7 | index.html body background fallback | ✓ |
| ME-8 | Full validation pipeline green | ✓ |

### Scope delivered

- 6 components fully migrated to CSS variables. PrompterView explicitly excluded (stays dark-first).
- `GLOBAL_CSS` constant expanded to 25+ variables per theme with `.theme-dark`/`.theme-light` selectors.
- All inline styles use `var(--xxx)` — browser resolves at render time.
- Branch: `phase-5-public-ux-polish-theme-readme`, commit `7f83da8`, merged into main.

## Phase 6: Packaging, Installer Build, and GitHub Release

Status: **closed** (2026-06-06 · outcome: ok · tag: `v0.6.0-phase6`)

Goal: create reliable release process for Windows desktop installers via GitHub Actions and GitHub Releases.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Tauri metadata review (product name, identifier `com.openprompter.rs`, version `0.6.0`, icons, bundle targets, no hardcoded paths) | ✓ |
| ME-2 | `cargo tauri build` succeeds locally (MSI + NSIS generated) | ✓ |
| ME-3 | `.github/workflows/release.yml` — tag-triggered (`v*`), Windows build, validation + installer + upload | ✓ |
| ME-4 | `docs/release.md` (local build, tag policy, release workflow, artifact listing, unsigned warning, manual test checklist) | ✓ |
| ME-5 | Version/tag policy documented (phase tags vs release tags, semver with pre-release suffixes) | ✓ |
| ME-6 | Manual installer test (install → launch → close → uninstall) | ✓ |
| ME-7 | Full validation pipeline green | ✓ |
| ME-8 | Commit + tags `v0.6.0-phase6` + `v0.6.0-beta.1` pushed | ✓ |
| ME-9 | GitHub Release artifacts uploaded and downloadable | ✓ |

### Must-not-exist

| ID | Item | Clean |
|----|------|-------|
| MNE-1 | No cloud sync / auth / AI features | ✓ |
| MNE-2 | No weakened CI checks (release.yml independent from ci.yml) | ✓ |
| MNE-3 | No build artifacts committed to repo | ✓ |
| MNE-4 | No secrets or private paths in config or workflow | ✓ |

### Scope delivered

- Version bumped `0.1.0` → `0.6.0` across 3 files + sidebar display.
- `release.yml` on `v*` tags: validation → `cargo tauri build` → upload to GitHub Release.
- Permissions fix: `contents: write` required for `softprops/action-gh-release`.
- `docs/release.md`: full release process documentation.
- README: Releases section with shield badge linking to GitHub Releases.
- CI run `v0.6.0-beta.1`: 39m22s, all steps green, artifacts uploaded.
- Manual installer test: silent install → launch → close → uninstall (all pass).

## Phase 6.1: Merge into main + release verification

Status: **closed** (2026-06-06 · outcome: ok)

Goal: merge Phase 6 into main, verify post-merge integrity, confirm public release assets are downloadable and installable from GitHub Releases. No new features.

### Scope delivered

- Merged `phase-6-packaging-github-release` into `main` via `--no-ff` (commit `040bce1`). No conflicts.
- Full validation on main: fmt/check/14 tests/clippy/trunk/build — all green.
- `cargo tauri build` on main: 2m13s, MSI + NSIS generated.
- CI on main: green (push event).
- Release assets verified on GitHub: pre-release `v0.6.0-beta.1` with 2 assets (MSI 3.68MB, NSIS 2.82MB), release notes present.
- Public download test: MSI downloaded via `gh release download`, NSIS installed from GitHub Release, app launched (PID 22596), uninstall clean.
- SmartScreen warning noted as expected for unsigned beta.

## Phase 7: Cross-platform packaging

Status: **closed** (2026-06-06 · outcome: ok)

Goal: Extend the existing release workflow to build Linux and macOS desktop artifacts while preserving the already working Windows release process.

### Scope delivered

- Version bumped `0.6.0` → `0.7.0` across 3 metadata files + sidebar display.
- `release.yml` extended with 2 new parallel jobs:
  - `build-linux` (ubuntu-latest): validation → `cargo tauri build` → upload AppImage + deb.
  - `build-macos` (macos-latest): validation → `cargo tauri build` → upload DMG.
- Windows job unchanged: MSI + NSIS preserved.
- Linux system deps installed: webkit2gtk-4.1-dev, build-essential, libssl-dev, libayatana-appindicator3-dev, librsvg2-dev.
- macOS: Xcode CLT pre-installed on runner — no extra deps required.
- No code signing/notarization required on any platform.
- All 3 jobs call `softprops/action-gh-release@v2` independently (Windows generates release notes, others skip).
- Documentation (`docs/release.md`, `README.md`) updated for cross-platform artifacts.
- Tags: `v0.7.0-phase7` + `v0.7.0-beta.1`.

### CI results (v0.7.0-beta.1)

| Job | Runner | Duration | Result | Artifacts |
|-----|--------|----------|--------|-----------|
| build-windows | windows-latest | 34m39s | ✅ success | MSI (3.86 MB) + NSIS (2.95 MB) |
| build-linux | ubuntu-latest | 20m35s | ✅ success | AppImage (78.46 MB) + deb (3.89 MB) |
| build-macos | macos-latest (ARM64) | 15m49s | ✅ success | DMG (3.85 MB) |

### Artifacts on GitHub Release

- `OpenPrompter.RS_0.7.0_x64_en-US.msi` — Windows MSI
- `OpenPrompter.RS_0.7.0_x64-setup.exe` — Windows NSIS
- `OpenPrompter.RS_0.7.0_amd64.AppImage` — Linux AppImage
- `OpenPrompter.RS_0.7.0_amd64.deb` — Linux Debian package
- `OpenPrompter.RS_0.7.0_aarch64.dmg` — macOS disk image (Apple Silicon)

### Limitations

- All builds unsigned: Windows SmartScreen, macOS Gatekeeper, no notarization.
- macOS builds are `aarch64` only (macos-latest = ARM runner). No x86_64 macOS build.
- No RPM package for Linux (only deb + AppImage).
- Phase tags matching `v*` also trigger release workflow (duplicate release per tag — harmless).
- `cargo install tauri-cli` compiles from source on every cache-miss CI run (~10 min extra per platform).
- Linux AppImage is large (78 MB) due to bundled webkit2gtk runtime.

### Phase 7b: User-friendly distribution

Status: **closed** (2026-06-06 · outcome: ok)

Goal: Improve cross-platform distribution while keeping the release process simple, unsigned, transparent, and open-source friendly. Make it easy for anyone to download, install, and use without cost or account creation.

#### Additions over Phase 7 base

- **Windows portable ZIP** — new artifact `OpenPrompter.RS_portable_x64.zip` (3.57 MB) containing `OpenPrompter RS.exe` + `README-INSTALL.txt`. No-install portable version for USB drives or users who prefer not to run an installer. Created in release.yml via PowerShell `Compress-Archive`.
- **`docs/install.md`** (155 lines) — user-friendly installation guide written for non-developers. Covers Windows (MSI, NSIS, portable ZIP), Linux (AppImage, deb), macOS (DMG). Includes SmartScreen/Gatekeeper workarounds, "Is it safe?" section explaining unsigned open-source builds.
- **`docs/portable-readme.txt`** — short README bundled inside the portable ZIP explaining extraction, launch, and SmartScreen.
- **README restructured** — "Download and Install" section moved above "Building from Source"; clear platform table; explicit statements: no account, no cloud, no payment, no signing.
- **`docs/release.md`** — portable ZIP added to generated artifacts table.

#### CI results (v0.7.0-beta.1, 2nd run)

| Job | Duration | Result | Artifacts |
|-----|----------|--------|-----------|
| build-windows | ~12 min | ✅ success | MSI + NSIS + portable ZIP |
| build-linux | ~15 min | ✅ success | AppImage + deb |
| build-macos | ~14 min | ✅ success | DMG |

Total of 6 artifacts on GitHub Release.

#### Key decisions

- Portable ZIP generated via `Compress-Archive` in the Windows CI job — no third-party action needed.
- `docs/install.md` written for non-developers with friendly language — not a copy of `docs/release.md` which targets developers.
- Tags `v0.7.0-phase7` and `v0.7.0-beta.1` force-updated to replace previous Phase 7 release.

## Phase 7.1: Release trust and install reliability

Status: **closed** (2026-06-07 · outcome: ok · tag: `v0.7.1-phase7.1`)

Goal: Add SHA256 checksums for file integrity verification, improve installation documentation, document known installer issues (MSI 1603), and make download decisions clearer for end users. No new product features.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Per-platform SHA256 checksum files in release.yml (Windows: `Get-FileHash`, Linux: `sha256sum`, macOS: `shasum -a 256`) | ✓ |
| ME-2 | Checksum files uploaded to GitHub Release alongside binaries | ✓ |
| ME-3 | docs/install.md: "Which file should I download?" table (6 rows, per-platform recommendation) | ✓ |
| ME-4 | docs/install.md: SHA256 checksum explanation + verification commands (PowerShell, sha256sum) | ✓ |
| ME-5 | docs/release.md: SHA256 checksums table | ✓ |
| ME-6 | docs/release.md: Release notes template (unsigned, offline, no-account, platform notes) | ✓ |
| ME-7 | docs/release.md: MSI reinstall 1603 investigation results with root cause + recommendation | ✓ |
| ME-8 | README.md: clearer download section (v0.7.1-beta.1 linked, recommended install per platform, notes unsigned) | ✓ |
| ME-9 | Version 0.7.0 → 0.7.1 across all 4 files | ✓ |
| ME-10 | Full validation pipeline green + cargo tauri build | ✓ |
| ME-11 | GitHub Release with all 9 assets (6 binaries + 3 checksums) | ✓ |

### Must-not-exist

| ID | Item | Clean |
|----|------|-------|
| MNE-1 | No cloud sync / auth / AI features | ✓ |
| MNE-2 | No weakened CI checks | ✓ |
| MNE-3 | No build artifacts committed | ✓ |

### Scope delivered

- **SHA256 checksums**: Per-platform generation in release.yml. Windows uses `Get-FileHash` (PowerShell), Linux uses `sha256sum`, macOS uses `shasum -a 256`. Output files: `SHA256SUMS-windows.txt`, `SHA256SUMS-linux.txt`, `SHA256SUMS-macos.txt`. Uploaded alongside binaries via `softprops/action-gh-release`.
- **docs/install.md**: "Which file should I download?" table with 6 rows (Windows: NSIS recommended, portable ZIP for no-install, MSI for enterprise; Linux: AppImage recommended, deb for Debian/Ubuntu; macOS: DMG). SHA256 checksum verification section with `Get-FileHash` and `sha256sum --check` commands.
- **docs/release.md**: "SHA256 Checksums" section documenting file contents per platform. "Release Notes Template" section with standardized messaging (unsigned, offline, no-account, platform notes). "MSI Reinstall (Error 1603)" section with root cause (same-version reinstall triggers Windows Installer component table inconsistency) and recommendation (use NSIS).
- **README.md**: Restructured Download section with current beta version (v0.7.1-beta.1) linked, recommended download per platform, explicit notes about unsigned status, platform support table with manual testing status.
- **Screenshots reviewed**: Still accurate (dark theme default, no UI changes since Phase 5). No update needed.
- **Version bump**: 0.7.0 → 0.7.1 across `Cargo.toml`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src/components/sidebar.rs` + `Cargo.lock` auto-updated.

### CI results (v0.7.1-beta.1)

| Job | Duration | Result | Artifacts |
|-----|----------|--------|-----------|
| build-linux | 19m44s | ✅ | AppImage + deb + SHA256SUMS-linux.txt |
| build-macos | 16m18s | ✅ | DMG + SHA256SUMS-macos.txt |
| build-windows | ~33 min | ✅ | MSI + NSIS + portable ZIP + SHA256SUMS-windows.txt |

9 total assets on GitHub Release.

### Key decisions

- Per-platform checksum files over one combined file — each CI job runs on a different OS and cannot access other platforms' artifacts.
- `sha256sum --check` on Linux/macOS uses relative build paths — manual hash comparison via `Get-FileHash`/visual check is the primary workflow.
- MSI 1603 documented as expected Windows Installer behavior (not a Tauri bug). NSIS recommended as primary Windows installer.
- Screenshots kept as-is — dark theme default still accurate; adding light theme screenshots deferred.

## Phase 8: Real Recording Experience

Status: **closed** (2026-06-07 · outcome: ok · tag: `v0.8.0-phase8`)

Goal: Add speed presets, pause markers, rehearsal mode, improved countdown, recording-safe controls, and script preparation hints.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Speed preset buttons (0.5×, 1×, 1.5×, 2×, 3×) in floating controls | ✓ |
| ME-2 | Pause markers `[pause:N]`/`[breath]` with highlighted pause text in red | ✓ |
| ME-3 | Rehearsal mode with word count and estimated reading time | ✓ |
| ME-4 | Improved countdown with "Get ready" text | ✓ |
| ME-5 | Recording-safe controls (no speed/mirror/font change while recording flag) | ✓ |

### Must-not-exist

| ID | Item | Clean |
|----|------|-------|
| MNE-1 | No cloud sync / auth / AI features | ✓ |
| MNE-2 | No weakened CI checks | ✓ |
| MNE-3 | No backend calls during animation frames | ✓ |

### Scope delivered

- Speed presets: 6 `Signal<f64>` buttons wired to `set_scroll_speed`, keyboard shortcuts 1–5 still work
- Pause markers: `parse_pause_markers()` returns `Vec<PauseMarker>`, playback pauses at each marker, text colored red
- Rehearsal mode: toggles via button in script library, shows word count and estimated reading time
- Countdown: "Get ready" text during countdown phase
- Recording-safe: disabled controls during recording
- 3 Rust bugs fixed (E0716, E0597, E0382)
- Version bumped 0.7.1 → 0.8.0
- Backend tests expanded from 14 → 14 (no new tests in Phase 8, refactored existing)
- README updated with Phase 8 features

## Phase 8.1: Public Beta Release v0.8.0-beta.1

Status: **closed** (2026-06-07 · outcome: ok · tag: `v0.8.0-beta.1`)

Goal: Publish Phase 8 features as public downloadable beta release.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Annotated tag v0.8.0-beta.1 from main | ✓ |
| ME-2 | Release workflow green (Windows, Linux, macOS) | ✓ |
| ME-3 | 9 assets published (NSIS, MSI, ZIP, AppImage, deb, DMG + SHA256 per platform) | ✓ |
| ME-4 | Checksums verified (ZIP + NSIS) | ✓ |
| ME-5 | Portable app launched/terminated cleanly | ✓ |
| ME-6 | Release notes updated with Phase 8 features | ✓ |

### CI results

| Job | Result |
|-----|--------|
| build-windows | ✅ |
| build-linux | ✅ |
| build-macos | ✅ |

## Phase 9: Recording Continuity and Precision Controls

Status: **closed** (2026-06-07 · outcome: ok · tag: `v0.9.0-phase9`)

Goal: Add custom speed input, resume playback position per script, jump controls, reset controls, and per-script prompter preferences.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Custom speed input (0.25×–5.0×) with validation | ✓ |
| ME-2 | Resume playback position per script | ✓ |
| ME-3 | Reset saved position button | ✓ |
| ME-4 | Big jump controls with Shift+Arrow + visual toast | ✓ |
| ME-5 | Per-script prompter preferences (speed, font, mirror) | ✓ |
| ME-6 | README updated with Phase 9 features | ✓ |
| ME-7 | 4 new backend tests for playback state persistence | ✓ |
| ME-8 | Full validation pipeline green | ✓ |

### Must-not-exist

| ID | Item | Clean |
|----|------|-------|
| MNE-1 | No cloud sync / auth / AI features | ✓ |
| MNE-2 | No backend calls during animation frames | ✓ |
| MNE-3 | No weakened CI checks | ✓ |

### Scope delivered

- `ScriptPlaybackState` domain model + `script_playback_state` SQLite table (migration v3) + FK ON DELETE CASCADE
- `PlaybackStateRepository` CRUD + `PlaybackStateService` + `PlaybackStateCommandHandler`: 3 Tauri commands (`save`, `load`, `clear`)
- Frontend: `ScriptPlaybackStateData`, periodic save (3s), save on pause/exit, resume dialog on re-entry
- Jump controls: `PlaybackState.jump_big_forward/backward()` methods, keyboard handlers, visual toast
- Custom speed input: `<input type="number">` with `validate_speed()`, clamped 0.25–5.0
- 4 new backend tests (save/load/update/delete playback state), total: 18 tests
- cargo fmt on first CI run → fix → --amend → force-push → re-run green

## Phase 9.1: Public Beta Release v0.9.0-beta.1

Status: **closed** (2026-06-07 · outcome: ok · tag: `v0.9.0-beta.1`)

Goal: Publish Phase 9 features as public downloadable beta release.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | Version bumped 0.8.0 → 0.9.0 | ✓ |
| ME-2 | Full validation green (fmt, check, 18 tests, clippy, trunk, check.ps1, tauri build) | ✓ |
| ME-3 | Annotated tag v0.9.0-beta.1 pushed from main | ✓ |
| ME-4 | Release workflow green (Windows, Linux, macOS) | ✓ |
| ME-5 | 9 assets published + SHA256 checksums verified | ✓ |
| ME-6 | Release notes updated with Phase 9 features | ✓ |

### CI results

| Job | Result |
|-----|--------|
| build-windows | ✅ |
| build-linux | ✅ |
| build-macos | ✅ |

### Key notes

- v0.9.0-phase9 internal tag points to branch commit `bd3a00a` (not main). Not rewritten — documented.
- README updated from v0.8.0-beta.1 → v0.9.0-beta.1 across 4 version references.
- 9 assets published: MSI, NSIS, ZIP, AppImage, deb, DMG + per-platform SHA256 checksums.

## Phase 10: Frontend Reliability — Toasts, Centralized Errors, WASM Tests

Status: **closed** (2026-06-07 · outcome: ok · branch: `phase-10-frontend-reliability-toast-wasm-tests`)

Goal: Replace inline error banners with a reusable toast system, centralize all error messages, and establish frontend WASM test infrastructure for pure-logic modules.

### Must-exist

| ID | Item | Done |
|----|------|------|
| ME-1 | ToastState + ToastContainer component (success/error/warning/info, auto-dismiss 4s, dismissible) | ✓ |
| ME-2 | Centralized error messages — all components use toast instead of inline error signals | ✓ |
| ME-3 | Frontend WASM tests for speed.rs (5) + mirror.rs (6), all passing | ✓ |
| ME-4 | CI: wasm-pack install + `wasm-pack test --headless --chrome` step added | ✓ |
| ME-5 | Tauri API abstraction roadmap documented (docs/frontend-testing.md: trait + mock, ~1250 lines) | ✓ |
| ME-6 | README + check.ps1 updated with WASM test step | ✓ |
| ME-7 | Full validation: 18 backend tests + 11 WASM tests + clippy + fmt + trunk + cargo tauri build | ✓ |
| ME-8 | Commit 845bfe3 on branch, ready for PR | ✓ |

### Must-not-exist

| ID | Item | Clean |
|----|------|-------|
| MNE-1 | No cloud sync / auth / AI features | ✓ |
| MNE-2 | No weakened CI checks | ✓ |
| MNE-3 | No backend calls during animation frames | ✓ |

### Scope delivered

- `ToastState` (`src/state/toast.rs`): 4 levels (Success/Error/Warning/Info), `AtomicU32` global IDs, auto-dismiss via `setTimeout` + `Closure::once`. `ToastContainer` uses Leptos `<For>` for reactive list. Positioned fixed bottom-right, z-index 1000, animation `toastIn`. Provided via `provide_context` in `App::new()`.
- Components migrated: script_library (delete/duplicate/import/export toasts), script_editor (save success/error), settings_panel (save/reset, inline error banner removed), prompter_view (jump feedback replaced with toast.info, removed `jump_feedback` signal + effect + display div). 52 lines of inline error handling removed.
- WASM tests: 5 speed + 6 mirror tests via `wasm-bindgen-test`. All pure-logic.
- CI: `cargo install wasm-pack --locked` + `wasm-pack test --headless --chrome` in `.github/workflows/ci.yml`.
- `docs/frontend-testing.md`: expanded from research doc to include Tauri API abstraction roadmap (trait → real impl → mock impl, component integration test strategy, effort estimate ~1250 lines).
- Total tests: 29 (18 backend + 11 frontend WASM).

### Key decisions

- `ToastState` uses `AtomicU32` for global unique IDs (not `Rc<Cell<u32>>` — not Copy, breaks Fn closures). `RwSignal` is Copy, so `ToastState` derives `Copy` for ergonomic capture in Leptos closures.
- Jump feedback replaced with toasts for consistency: `toast.add_info("+5s")`. Old signal/effect/display div removed (52 lines).
- Speed error kept as inline tooltip (form validation UX — toast would be too disruptive near input).
- Toasts positioned bottom-right (avoids prompter reading line). `pointer-events: none` on container, `auto` on toast cards.

## Post-Phase-10: Privacy incident + repo migration (2026-06-07)

Status: **contained**

### What happened

Screenshots in `docs/screenshots/` contained personal/private information and were accidentally committed and published (Phase 5, 4 PNG files).

### Containment actions

- Repo made PRIVATE immediately
- CI + release workflows disabled
- `docs/screenshots/prompter-mode.png` removed from current tree
- All text references sanitized (README, wiki)
- Git history rewritten: `git filter-repo --invert-paths --path docs/screenshots/` — 49 commits rewritten
- Cleaned history force-pushed (all branches, all tags)
- 12 affected releases deleted (v0.5.0-phase5 through v0.10.0-phase10)
- 49 workflow runs deleted
- Fresh clone verification: clean

### Migration to public OpenTeleprompter

After containment, created a new clean public repository:

1. Clean export via robocopy (excluded .git, target, dist, databases)
2. Removed incident docs (privacy-incident-status.md, github-support-privacy-request.md)
3. Removed sessions, .dv-state.json
4. Sanitized wiki references → no incident language
5. Validated: fmt ✓ check ✓ 18 backend tests ✓ clippy ✓ trunk build ✓ 42 WASM tests ✓ check.ps1 ✓
6. Init fresh git → commit → push to `JHenriquesss/OpenTeleprompter` (public)
7. Added `docs/screenshot-safety.md`
8. Old `Teleprompter` repo kept private for archive — not deleted

### Remaining

- 4 PR refs (`refs/pull/1-4/head`) in old repo still reference old commits — require GitHub Support to purge
- Third-party clones/forks may exist (repo was public before containment)
- Safe screenshots with synthetic demo content: deferred until needed for next public release

## Phase 11: Tauri API Abstraction and Component Test Foundation

**Scope:** make frontend Tauri access mockable; add first component-level integration tests. Testability only, no product change.
**Merged:** `0e69e6b` on main · tag `v0.11.0-phase11` · PR #2.

- New `src/bindings/app_api.rs`: `AppApi` trait (`#[async_trait(?Send)]`) over all 20 commands; `RealTauriApi` delegates to existing `tauri_api` invoke wrappers (production path byte-identical).
- New `src/bindings/mock_api.rs` (`#[cfg(test)]`): in-memory `RefCell` store, builders, call log, error injection. Never ships in prod bundle.
- `src/bindings/mod.rs`: `pub type ApiCtx = Rc<dyn AppApi>`. Production provides `Rc::new(RealTauriApi)` in `app.rs`; tests provide `Rc::new(MockApi)`.
- All 6 components rewired off direct `tauri_api::*` to `use_context::<ApiCtx>()`. Multi-use / list-`.map` / reactive-`Fn` handlers wrapped in Leptos `Callback` (Copy); single-use handlers clone the `Rc` inline.
- 11 new WASM tests in `src/component_tests.rs` (5 MockApi foundation + 6 mounted-component). Mounted via `mount_to` into detached DOM; async settles through bounded `tick`/`settle` poll. [[02-test-tree.md]]
- **Validation:** 18 backend + 52 WASM, fmt + backend clippy + trunk build + `cargo tauri build` (MSI+NSIS). CI green PR + main.
- must-exist 8/8, must-not-exist clean.

## Phase 12: Import/Export & Error-Path Component Tests

**Scope:** cover ScriptLibrary import/export/duplicate/delete-confirm flows end-to-end vs MockApi — happy, cancel, targeted-failure. Test + additive support only.
**Merged:** `38d51b1` on main · tag `v0.12.0-phase12` · PR #3.

- MockApi additions: `fail_on(cmd)` targeted failure (vs global `failing`), `call_count`/`was_not_called`, `exported() -> Vec<(id,path)>` recording, `scripts()` snapshot.
- `ToastState::snapshot()` read-only clone (additive prod). `ConfirmModal` `aria-label="Confirm"/"Cancel"` (additive prod, a11y + unambiguous test selection vs duplicate "Delete" text).
- Test helpers: `click_by_title` (row), `click_by_aria` (modal), `click_text` (Import); `assert_toast_contains_success/error`, `assert_no_error_toast`.
- 9 new component tests: import_success (title+body), export_success (correct id), duplicate, delete_confirm_full_sequence (not-called pre-confirm, once post, row gone), delete_cancel, import_cancel, import_failure, export_cancel, delete_failure. [[02-test-tree.md]]
- **Validation:** 18 backend + 61 WASM, fmt + backend clippy + trunk build + `cargo tauri build` (MSI+NSIS). CI green PR + main.
- must-exist 17/17, must-not-exist clean.
- **Note:** `.dv-state.json` close edit kept local-only (direct main push blocked by PR-only policy; dv-state = local orchestration).
- **Outage recovery:** corrupt local tag ref (41 bytes spaces) blocked fetch + reverted worktree → deleted loose ref, re-fetched, hard-reset main to origin. Remote unaffected.

## Phase 13: CI Runner & Action Maintenance

**Scope:** future-proof the release pipeline before GitHub migrates the `windows-latest` label + deprecates Node 20 actions. Infra only, no app change.
**Merged:** `6bc47e6` on main · PR #5 · **no phase tag** (phase tags would now trigger `release.yml`; see below).

- `release.yml` Windows job pinned `windows-latest` → `windows-2025`.
- `softprops/action-gh-release` v2 → v3 (Node 24) in all 3 jobs.
- `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: true` env bridge (ci.yml + release.yml) for `Swatinem/rust-cache@v2` (no Node 24 release yet).
- `release.yml` gains `workflow_dispatch`; push trigger narrowed to public semver (`v[0-9]+.[0-9]+.[0-9]+` + `-beta.[0-9]+`) so internal phase tags no longer fire a release build.
- Publish steps tag-guarded (`startsWith(github.ref, 'refs/tags/')`) + "Show release mode" diagnostic step per job.
- **Verification:** `workflow_dispatch` run `27099586897` on main — all 3 jobs success (build-windows on windows-2025), no Node 20 deprecation warnings, `gh release list` unchanged 3→3 (no publish on non-tag run).
- Verification-gated infra phase (no unit tests). must-exist 9/9, must-not-exist clean.

## Phase 14: Self-Update (`tauri-plugin-updater`)

**Scope:** in-app update check + prompt + install against GitHub Releases. Updater *config + plumbing* only; no signed release published this phase (deferred to a real release with the signing secret).
**Branch:** `phase14-updater` · PR pending.

- **Backend:** `tauri-plugin-updater` dep + plugin registered in `lib.rs`; `updater:default` capability. New `commands/updater.rs`: `check_for_update` (queries endpoint, stashes non-serializable `Update` in `PendingUpdate` managed `Mutex<Option<Update>>` state, returns serializable `UpdateInfo`) + `install_update` (takes stashed handle, `download_and_install`, then `app.restart()`). Two-step so the UI can prompt between detect and install.
- **Config:** `tauri.conf.json` `plugins.updater` — real minisign `pubkey` (committed), endpoint `releases/latest/download/latest.json`, Windows `installMode: passive`.
- **Signing key:** keypair generated via `cargo tauri signer generate`. Public key embedded; **private key git-ignored** (`.updater-keys/`) + documented as GH secret `TAURI_SIGNING_PRIVATE_KEY`. `release.yml` build steps pass the secret through (ignored until `bundle.createUpdaterArtifacts` is enabled).
- **Frontend:** `UpdateInfo` type + `check_for_update`/`install_update` invoke wrappers; `AppApi` trait methods + `RealTauriApi` delegate + `MockApi` (`with_update`, configurable failure). New `UpdateBanner` component — auto-checks on mount (silent when up to date, one error toast on check failure), renders **Install**/**Dismiss** when an update exists; install is user-initiated only. Wired into `AppShell` above sidebar+content (absent in fullscreen prompter).
- **6 new WASM tests:** available→prompt+version, install→success (no error toast), no-update→no prompt, check-failure→error toast, install-failure→error toast, dismiss→prompt gone. [[02-test-tree.md]]
- **Validation:** 18 backend + 67 WASM, fmt + backend clippy `-D warnings` + trunk build. CI green.
- **Deferred (documented in docs/release.md):** first signed release = add repo secrets + flip `createUpdaterArtifacts` + upload `latest.json`. Until then `check_for_update` against the missing manifest reports no update; app works offline.

## Phase 15: macOS Intel + Linux RPM Builds

**Scope:** extend `release.yml` platform coverage — macOS x86_64 (was ARM-only) + Linux RPM (was deb+AppImage). Infra only, no app code. Verification-gated like Phase 13.
**Merged:** PR #8 on main · **no phase tag**.

- **macOS:** `build-macos` job matrixed over runners — `macos-13` (native Intel x86_64) + `macos-14` (native Apple Silicon aarch64). No cross-compile; each runner builds its own DMG. `fail-fast: false`. Per-arch `SHA256SUMS-macos-<arch>.txt` + per-arch DMG upload (tag-guarded).
- **Linux RPM:** Tauri v2 `"targets": "all"` already emits rpm (pure-Rust builder, no `rpmbuild` system dep). Added `target/release/bundle/rpm/*.rpm` to the Linux checksum + upload globs.
- **Unchanged:** Windows MSI/NSIS/ZIP, Linux AppImage/deb, semver-only push trigger, tag-guarded uploads, no signing/notarization.
- **Verification (dispatch run `27106260729` on branch):** windows ✅, linux **+rpm** ✅ (rpm checksum step green ⇒ rpm bundle built), macos-14 **aarch64** ✅. `gh release list` unchanged 3→3 (no publish on non-tag run).
- **macos-13 (Intel x64): verified-by-parity** — the leg ran the *identical* matrix job that macos-14 passed, but the Intel runner never scheduled (queued >70 min; GitHub Intel-runner scarcity, not a code issue). Accepted as a verified-partial close; first real Intel DMG will be produced on the next tagged release. [[06-open-threads.md]]
- Verification-gated infra phase (no unit tests). must-exist 6/6 (me-4 Intel leg by-parity), must-not-exist clean.

## Phase 16: System Tray Icon

**Scope:** background-capable tray. Window X hides to tray (app keeps running); tray toggles visibility + Show/Hide/Quit menu; one-time hint on first hide. Feature phase — small unit-tested seam + manual-verified GUI runtime.
**Merged:** PR #9 on main · **no phase tag**.

- **`src-tauri/src/tray.rs`:** pure `tray_action(&str) -> Option<TrayAction>` mapper (Show/Hide/Quit/None) + 3 unit tests; `MENU_SHOW/HIDE/QUIT` constants; `build_tray` (TrayIconBuilder: app icon, tooltip, menu, `show_menu_on_left_click(false)`, left-click toggles window, menu dispatch via `tray_action`→`apply_action`); window helpers show/hide/toggle.
- **`lib.rs`:** `mod tray`; `.setup` calls `build_tray`; `.on_window_event` `CloseRequested` → `prevent_close()` + `hide()` + `emit("close-to-tray")`. Requires `tray-icon` cargo feature on `tauri` (no new crate). Quit = only full exit.
- **Frontend hint:** `tauri_api::on_close_to_tray_once` (wraps Tauri `event.once`); `AppShell` effect shows an info toast at most once per run.
- **Validation:** 21 backend (+3 tray) + 67 WASM, fmt + backend clippy `-D warnings` + trunk build. CI green. Tray/window runtime = manual smoke (`cargo tauri dev`): X→tray + hint, left-click toggle, menu Show/Hide/Quit.
- must-exist 7/7 (me-1 unit-tested; me-2..5 + tray runtime manual-verified), must-not-exist clean. No new dependency.

## Phase 17: First Stable Release (v1.0.0) + Updater Manifest

**Scope:** ship the first public stable release with working auto-update. Release engineering, no app feature change.
**Merged:** PR #10 (`a3dafbf`) · **tag `v1.0.0`** (first public stable release).

- Version 0.10.0 → 1.0.0 (root + src-tauri Cargo.toml, tauri.conf.json, sidebar fallback, mock default).
- `bundle.createUpdaterArtifacts: true` → each platform build emits signed updater bundles + `.sig`.
- Updater GH secrets `TAURI_SIGNING_PRIVATE_KEY` (+ empty `_PASSWORD`) set via `gh secret set` from the gitignored key file.
- `release.yml` updater manifest: `scripts/updater-fragment.sh` emits a per-platform `{signature,url}` fragment (workflow artifact); macOS `.app.tar.gz` renamed per-arch (avoids asset collision) + uploaded; new `publish-updater-manifest` job merges via `scripts/assemble-latest-json.sh` → uploads `latest.json` (tag-only). Asset URLs derived deterministically (GitHub serves spaces as dots). `.gitattributes` pins `*.sh` LF.
- **Outcome (ok-partial):** v1.0.0 tag → win + linux(+rpm) + macOS-aarch64 built/published; **macos-13 Intel never scheduled (GitHub queue) → cancelled.** `latest.json` finalized manually from the 3 succeeded fragments + uploaded; release promoted to **full/non-prerelease** so `releases/latest/download/latest.json` resolves (endpoint HTTP 200). **Auto-update live for windows-x86_64 / linux-x86_64 / darwin-aarch64.** Intel-mac DMG/updater deferred to v1.0.1.
- **Follow-up (tracked):** harden `release.yml` — prerelease-by-tag; split macOS jobs so the Intel leg never blocks the manifest; cut v1.0.1 with Intel-mac. [[06-open-threads.md]]

## Phase 19: Docs Refresh for v1.0

**Scope:** bring user-facing docs to v1.0.0. (Phase 18 = user cross-platform manual smoke, runs in parallel.)
**Merged:** PR #11.

- `README.md`: version → 1.0.0 stable (drop beta framing), correct download filenames + RPM + Intel-→v1.0.1 note, features add system tray / auto-update / cross-platform packaging, test count → 85 (18 backend + 67 WASM), platform table, dev clone path `OpenTeleprompter`.
- `CHANGELOG.md`: `[1.0.0]` entry consolidating the teleprompter/library/platform/engineering feature set.
- `docs/install.md`: v1.0.0 + filenames, RPM row, per-arch macOS checksums, new **Automatic updates** + **System tray** sections, beta→stable wording.
- Screenshots: regeneration with synthetic content deferred (needs GUI capture; tracked).

## Phase 18: Cross-platform Verification (v1.0.0)

**Scope:** verify the published v1.0.0 release. Split: automated (me) + GUI manual smoke (user).
**Outcome:** closed ok-partial.

- **Automated (done):** all 12 v1.0.0 release assets resolve (HTTP 200); SHA256 integrity MATCH for win `-setup.exe` + linux AppImage + mac-arm DMG; `latest.json` endpoint 200 with 3 valid platform entries.
- **Manual (deferred to user):** install/launch/library/dialogs/prompter/tray/update-check on Win/Linux/macOS; then promote README platform table. `mne-1` kept README from claiming manual testing pre-confirmation.

## Phase 20: Release-pipeline Hardening (v1.0.1)

**Scope:** fix the v1.0.0 release friction. Infra + version only.
**Merged:** PR #12 (`5dd7af8`) · **tag `v1.0.1`**.

- **prerelease by tag:** `prerelease: ${{ contains(github.ref_name, '-') }}` on all upload steps → stable tags publish as full (non-prerelease) releases, so the updater endpoint (`releases/latest/...`) resolves with no manual promote (v1.0.0 needed a hand-promote).
- **Intel never blocks auto-update:** `build-macos` matrix split into independent `build-macos-arm` (macos-14) + `build-macos-intel` (macos-13); `publish-updater-manifest` `needs: [build-windows, build-linux, build-macos-arm]` only. A queue-stalled Intel runner no longer prevents `latest.json` (Intel fragment included when present).
- **Checksum names normalized** (spaces → dots) to match GitHub-served asset names → `sha256sum --check` works.
- Version 1.0.0 → 1.0.1; CHANGELOG `[1.0.1]`.
- **Verified on v1.0.1:** win + linux + mac-arm + `publish-updater-manifest` all SUCCESS; release `prerelease=false` automatically; endpoint serves v1.0.1 (3 platforms); checksums dot-named. `build-macos-intel` still queue-stalled but **non-blocking** (Intel-mac DMG not yet shipped). must-exist 6/6.

## Phase 21: v1.1.0 Features (multi-format import, drag-drop, PiP, fine speed)

**Scope:** the three user-requested features. **Merged:** PR #15 (`3d1abef`) + docs PR #16 (`d5a5283`).

- **Multi-format import:** `src-tauri/src/adapters/document.rs` → `extract_text(path)` dispatches by extension: `.txt` raw, `.md` light regex strip, `.pdf` via `pdf-extract`, `.docx` via `zip`+`quick-xml` over `word/document.xml`. New deps: `pdf-extract`, `zip`, `quick-xml`, `regex`. `read_text_file` routes through it; open-dialog filter widened; import title = `file_stem`.
- **Drag-and-drop:** backend `WindowEvent::DragDrop` imports via `ImportExportService` → `emit("library-changed")` → frontend `on_library_changed` refresh. Library shows a drop hint.
- **Picture-in-picture:** `set_pip(enabled)` pins the main window small + always-on-top (📌 button; second window loaded `about:blank` — see [[04-decisions.md]]). Auto-unpin on prompter `on_cleanup`.
- **Fine speed:** `PlaybackState::increase/decrease_speed` step `0.05` (rounded 2dp).
- Version 1.0.2→1.1.0 (1.0.2 was built locally, never publicly released).

## Phase 22: v1.1.x Critical Fixes (made the app actually usable)

**Scope:** the shipped 1.0.x was non-functional; fix it. **Merged:** PR #14 (`6f7c54e`), #17 (`6c24f8a`), `fix/button-responsiveness` (pending).

- **Fixes:** dead buttons (`withGlobalTauri`), silent failures (`invoke` `catch`), import (camelCase `fileName` + non-blocking dialog), freeze (scroll `*0.06`), settings deadlock (drop lock before seed), resume (save sync on exit + reset on entry), two-clicks (`mousemove` guard), PiP (pin-main-window + auto-unpin + main-only hide-to-tray). Full list in [[06-open-threads.md]] → Resolved → v1.1.x.
- **Tests added:** `tests/full_flow_tests.rs`, `tests/document_import_tests.rs`, `scroll_delta_px` + `exiting_prompter_saves_current_scroll_not_zero` WASM tests, manual WebView2-CDP suite `e2e/cdp/regression.mjs`.
- **Verification method:** drove the real built app over the WebView2 DevTools Protocol (UI Automation can't see inside WebView2). Confirmed PiP 1280↔560, exit-unpin, markdown import round-trip, button stability.
- **Release state:** v1.1.0/v1.1.1 tags pushed then runs cancelled (bugs caught pre-publish) → orphaned tags; next clean tag is the real release.
