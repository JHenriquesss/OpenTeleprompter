# OpenPrompter RS — Release Process

## Supported Platforms

| Platform | Status | Notes |
|----------|--------|-------|
| Windows (x64) | ✅ Supported | MSI + NSIS installer |
| Linux (x86_64) | ✅ Supported | AppImage + deb package |
| macOS (x64, ARM) | ✅ Supported | DMG (unsigned) |

Current builds target **Windows x64**, **Linux x86_64**, and **macOS** (Apple Silicon / Intel).

### CI / runner maintenance

- Windows release builds are pinned to **`windows-2025`** (not `windows-latest`) to avoid surprise breakage when GitHub migrates the `windows-latest` label.
- `release.yml` can be **manually dispatched** (`workflow_dispatch`, or `gh workflow run release.yml --ref <branch>`) to verify a build without publishing — every upload step is gated on `startsWith(github.ref, 'refs/tags/')`, so a non-tag run produces **no** GitHub Release or assets.
- `release.yml` push trigger matches public-release tags only (`v[0-9]+.[0-9]+.[0-9]+`, `-beta.[0-9]+`); internal phase tags do not trigger it.
- Node 24: `softprops/action-gh-release@v3` and `actions/checkout@v5` are Node 24-native; `Swatinem/rust-cache@v2` is bridged via `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: true` until it ships a Node 24 release.

---

## Version Policy

### Release Tags (public distribution)

Release tags follow semantic versioning with optional pre-release suffixes:

- `v0.6.0` — stable release
- `v0.6.0-beta.1` — beta pre-release
- `v0.6.0-rc.1` — release candidate

Pushing a tag matching `v*` triggers the **Release workflow** (`.github/workflows/release.yml`), which builds Windows installers, Linux packages, and macOS DMGs, then uploads all artifacts to a GitHub Release.

### Phase Tags (internal milestones)

Phase tags mark internal development milestones and **do** trigger the release workflow (since they match `v*`), but are intended for CI validation, not public distribution:

- `v0.7.0-phase7` — Phase 7 development complete (cross-platform packaging)
- `v0.6.0-phase6` — Phase 6 development complete
- `v0.5.1-phase5.1` — Phase 5.1 development complete

Phase tags verify that the packaging and release pipeline functions correctly without publishing a public release.

---

## Local Build

### Prerequisites

- Rust (stable): `rustup install stable` or update via `rustup update`
- WASM target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk --locked`
- Tauri CLI v2: `cargo install tauri-cli --version "^2"`

### Build the Installer

```bash
cargo tauri build
```

This compiles the frontend (Leptos → WASM), the backend (Rust native), and packages everything into platform-specific installers.

### Generated Artifacts

After a successful build, artifacts are written to `target/release/bundle/`.

| Platform | Artifact | Path | Description |
|----------|----------|------|-------------|
| Windows | Standalone EXE | `target/release/openprompter-rs-tauri.exe` | Portable executable (no installer) |
| Windows | MSI | `target/release/bundle/msi/OpenPrompter RS_*.msi` | Windows Installer package |
| Windows | NSIS | `target/release/bundle/nsis/OpenPrompter RS_*-setup.exe` | NSIS setup executable |
| Windows | Portable ZIP | `target/release/OpenPrompter.RS_portable_x64.zip` | No-install archive (EXE + README) |
| Linux | AppImage | `target/release/bundle/appimage/*.AppImage` | Portable AppImage |
| Linux | deb | `target/release/bundle/deb/*.deb` | Debian/Ubuntu package |
| macOS | DMG | `target/release/bundle/dmg/*.dmg` | macOS disk image |

---

## GitHub Release Workflow

The release workflow (`.github/workflows/release.yml`) runs automatically when a tag matching `v*` is pushed.

### What it does

1. Checks out the repository
2. Installs Rust stable with `wasm32-unknown-unknown` target
3. Installs Trunk and Tauri CLI
4. Runs validation checks:
   - `cargo fmt --all -- --check`
   - `cargo check -p openprompter-rs-tauri --all-targets`
   - `cargo test -p openprompter-rs-tauri`
   - `cargo clippy -p openprompter-rs-tauri --all-targets --all-features -- -D warnings`
   - `trunk build`
5. Runs `cargo tauri build` on each platform
6. Uploads generated artifacts to the GitHub Release:
   - **Windows**: MSI, NSIS installer, and portable ZIP
   - **Linux**: AppImage + deb package
   - **macOS**: DMG disk image
7. Marks the release as a pre-release (for beta/RC tags)

### SHA256 Checksums

Each platform job generates a per-platform checksum file:

| File | Contains |
|------|----------|
| `SHA256SUMS-windows.txt` | MSI, NSIS, portable ZIP |
| `SHA256SUMS-linux.txt` | AppImage, deb |
| `SHA256SUMS-macos.txt` | DMG |

Format (same as `sha256sum`):

```
<hex>  <filename>
```

Checksum files contain only release asset basenames (no build-directory paths). Users should download the artifact and its matching `SHA256SUMS-*` file into the same directory, then verify:

```bash
# Windows (PowerShell)
Get-FileHash .\OpenPrompter.RS_*.msi

# Linux / macOS (place checksum file next to artifacts)
sha256sum --check SHA256SUMS-linux.txt
```

### Release Notes Template

When creating a release tag, the workflow generates automatic release notes. The description should communicate:

- OpenPrompter RS is **free and open-source** (MIT / Apache 2.0).
- **No account required** — download and use immediately.
- **No internet connection required** after download — runs entirely offline.
- Builds are **unsigned beta builds**:
  - Windows may show SmartScreen warning (click "More info → Run anyway").
  - macOS may show Gatekeeper warning (right-click → Open on first launch).
  - Linux users may need to `chmod +x` the AppImage.
- **Windows is the most manually tested platform.**
- **Linux and macOS are beta artifacts** generated by CI — community testing is welcome.

### How to Create a Release

```bash
# 1. Ensure main is up to date
git checkout main
git pull --ff-only origin main

# 2. Tag the release
git tag -a v0.7.0-beta.1 -m "OpenPrompter RS v0.7.0 beta 1 — cross-platform packaged desktop release"

# 3. Push the tag
git push origin v0.7.0-beta.1

# 4. Monitor the release workflow
#    https://github.com/JHenriquesss/OpenTeleprompter/actions/workflows/release.yml

# 5. Once complete, artifacts appear on the GitHub Release page
#    https://github.com/JHenriquesss/OpenTeleprompter/releases/tag/v0.7.0-beta.1
```
---

## Auto-Update (Updater)

The app ships with `tauri-plugin-updater`. On launch the frontend `UpdateBanner`
calls the backend `check_for_update` command, which queries the updater endpoint:

```
https://github.com/JHenriquesss/OpenTeleprompter/releases/latest/download/latest.json
```

If a newer version is published there, the banner offers **Install** (download +
install + relaunch) or **Dismiss**. Install is always user-initiated — there is
no silent auto-install. Being up to date is silent; a failed check shows one
toast.

### Updater signature (mandatory, not OS code-signing)

The updater requires its **own minisign signature** (separate from Authenticode
/ notarization, which remain out of scope). Each updater artifact is signed with
a private key; the app embeds the matching **public key** in
`src-tauri/tauri.conf.json` under `plugins.updater.pubkey`.

- The public key is committed (safe to publish).
- The **private key is never committed** — it lives in `.updater-keys/`
  (git-ignored) locally and in the GitHub Actions secret
  `TAURI_SIGNING_PRIVATE_KEY` (with `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` for the
  key passphrase).

Generate a keypair with:

```bash
cargo tauri signer generate -w .updater-keys/openprompter.key
```

### Publishing an updater-enabled release (deferred)

Updater *config* is wired, but no signed release is published yet. To enable
end-to-end updates:

1. Add repo secrets `TAURI_SIGNING_PRIVATE_KEY` and
   `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (the `release.yml` build steps already
   pass them through; they are ignored while artifacts are disabled).
2. Set `bundle.createUpdaterArtifacts: true` in `tauri.conf.json` so
   `cargo tauri build` emits the signed updater bundles + `.sig` files.
3. Upload the updater artifacts **and** a `latest.json` manifest to the GitHub
   Release. `latest.json` shape:

   ```json
   {
     "version": "0.13.0",
     "notes": "Release notes",
     "pub_date": "2026-06-07T00:00:00Z",
     "platforms": {
       "windows-x86_64": { "signature": "<contents of .sig>", "url": "https://github.com/.../OpenPrompter.RS_0.13.0_x64-setup.nsis.zip" }
     }
   }
   ```

Until then, `check_for_update` against the missing `latest.json` simply reports
no update (or an update-check error) — the app keeps working offline.

## Unsigned Installer Warning

All platform builds are **unsigned**:

| Platform | Warning | Workaround |
|----------|---------|------------|
| Windows | SmartScreen blocks download/install | Click **More info → Run anyway** |
| macOS | Gatekeeper blocks unsigned app | Right-click → **Open** (once), or `xattr -dr com.apple.quarantine /Applications/OpenPrompter\ RS.app` |
| Linux | No signing — packages install without warnings | Standard `dpkg -i` or AppImage execution |

Code signing and notarization will be addressed in a future phase.

To install an unsigned build:

**Windows:**
1. Download the `.msi` or `-setup.exe` from the GitHub Release
2. If SmartScreen blocks it, click **More info** → **Run anyway**
3. Proceed through the installer

**Linux:**
1. Download the `.deb` or `.AppImage` from the GitHub Release
2. Install the deb: `sudo dpkg -i openprompter-rs_*.deb`
3. Or run the AppImage: `chmod +x *.AppImage && ./OpenPrompter\ RS_*.AppImage`

**macOS:**
1. Download the `.dmg` from the GitHub Release
2. Open the DMG and drag the app to Applications
3. First launch: right-click the app → **Open** (instead of double-click)
4. Subsequent launches work normally

---

## CI Validation (Regular)

The regular CI workflow (`.github/workflows/ci.yml`) runs on every pull request and push to `main`. It validates:

- Code formatting
- Backend compilation
- Backend tests (14+ tests)
- Clippy linting (zero warnings)
- Frontend WASM build

This is separate from the release workflow and ensures day-to-day code quality.

---

## Manual Installer Test Checklist

After building or downloading a release, verify:

- [ ] Installer file is generated
- [ ] Installer launches
- [ ] App installs
- [ ] App opens after install
- [ ] Script library loads
- [ ] New script can be created
- [ ] Auto-save works
- [ ] App can be closed and reopened with data preserved
- [ ] Prompter mode opens
- [ ] Playback works
- [ ] Native import/export dialogs work
- [ ] Delete confirmation works
- [ ] Theme toggle works
- [ ] App uninstalls cleanly
- [ ] No private test data is bundled

---

## MSI Reinstall (Error 1603)

### Symptom

Installing the MSI via `msiexec /i *.msi /quiet` exits with code **1603** (fatal error) when a previous version is already installed. The NSIS installer and portable ZIP are unaffected.

### Investigation

- **Expected behavior:** Windows Installer (msiexec) returns 1603 when an MSI package tries to install over an existing installation without a proper upgrade path (higher version number + matching UpgradeCode).
- **Tauri v2 behavior:** Tauri generates a unique **ProductCode** per build, but the **UpgradeCode** is derived from the app identifier (`com.openprompter.rs`). The MSI version embeds the `tauri.conf.json` version (`0.7.0`).
- **Root cause:** The MSI from `v0.7.0-beta.1` and the locally built MSI have the same version (`0.7.0`). Windows Installer treats same-version reinstall as a "reinstall" rather than an "upgrade" when MSI file differences trigger component table inconsistencies. The first install of any MSI version works.
- **NSIS vs MSI:** NSIS does not use Windows Installer — it is a custom installer that handles version detection and overwrites files directly. Portable ZIP has no installer at all.

### Recommendation

- **Most Windows users should use NSIS (`_x64-setup.exe`)** — it handles upgrades and reinstalls reliably.
- **For no-install use, use the portable ZIP.**
- **MSI is suitable for:** clean first-time deployments, enterprise deployment via Group Policy.
- **If upgrading via MSI is required:** uninstall the previous version first, or wait for a future version that increments the product version.

### Portable ZIP

The portable ZIP (`OpenPrompter.RS_portable_x64.zip`) contains only the standalone EXE and a README file. No installer, no registry changes, no dependencies. Recommended for:
- USB drive use
- Users who prefer not to run installers
- Testing without system modification

## Future Goals

- **Code signing** — Windows Authenticode, macOS notarization
- **Automated installer testing** — smoke tests on CI
- **Auto-update** — `tauri-plugin-updater` wired (see [Auto-Update](#auto-update-updater)); first signed release still pending
- **RPM package** — Linux RPM for Fedora/RHEL
