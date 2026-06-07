# OpenPrompter RS

[![CI](https://github.com/JHenriquesss/Teleprompter/actions/workflows/ci.yml/badge.svg)](https://github.com/JHenriquesss/Teleprompter/actions/workflows/ci.yml)

A lightweight, local-first, offline desktop teleprompter application for Windows, macOS, and Linux.

Built with [Tauri v2](https://v2.tauri.app/), [Rust](https://www.rust-lang.org/), and [Leptos](https://leptos.dev/).

> **Offline-first** — no cloud, no network, no accounts. Your scripts never leave your machine.

---

## Screenshots

Screenshots temporarily removed while demo images are regenerated with synthetic content.

---

## Features

- **Script Library** — create, edit, save, duplicate, delete, search, import (`.txt`), and export (`.txt`) scripts
- **Teleprompter Mode** — fullscreen playback with smooth `requestAnimationFrame` scrolling
- **Adjustable typography** — font size, line height, text width
- **Variable speed** — scroll speed from 0.25× to 10×
- **Mirror modes** — horizontal and/or vertical mirror for teleprompter hardware
- **Countdown timer** — optional countdown before playback (0–10 s)
- **Reading guide** — horizontal focus band overlay
- **Progress display** — percentage complete and estimated remaining time
- **Floating controls** — overlay auto-hides during playback, appears on mouse move
- **Keyboard shortcuts** — full shortcut reference with on-screen help (`H`)
- **Settings persistence** — all preferences saved to local SQLite
- **Theme toggle** — dark (default) / light theme, persisted across sessions
- **Custom scrollbars** — themed scrollbars across the application
- **Auto-save** — debounced save (500 ms) with `Unsaved → Saving → Saved` status
- **Ctrl+S** — explicit save shortcut anywhere in the editor
- **Delete confirmation** — modal with guard against double-click
- **Native file dialogs** — OS-native open/save dialogs for import/export
- **Speed presets** — one-click speed buttons (0.5×, 1×, 1.5×, 2×, 3×) in the floating controls
- **Custom speed input** — type any speed (0.25×–5.0×) directly in the controls
- **Pause markers** — embed `[pause:N]` or `[breath]` in scripts for automatic pauses during playback (pauses highlighted in red)
- **Rehearsal mode** — dedicated practice mode with word count and estimated reading time, launched from the script library
- **Resume playback** — saves scroll position, speed, font size, and mirror mode per script; resume dialog on re-entry
- **Jump controls** — Arrow Left/Right jumps 5s, Shift+Arrow jumps 20s, with visual feedback
- **Progress reset** — "Start from beginning" and "Reset" to clear saved position
- **Improved countdown** — "Get ready" text displayed during countdown
- **Recording-safe controls** — overlay designed for safe use while screen recording
- **No JavaScript** — 100% Rust, compiled to WASM for the frontend
- **Toast notifications** — non-blocking success/error/warning/info toasts for all user actions; 4 s auto-dismiss
- **Centralized error messages** — all error feedback unified through the toast system instead of scattered inline error banners
- **WASM test suite** — 11 pure-logic tests run via `wasm-pack test --headless --chrome`

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Space` | Play / Pause |
| `Esc` | Exit prompter mode |
| `F` | Toggle fullscreen |
| `R` | Restart from top (with countdown if enabled) |
| `Arrow Up` | Increase scroll speed |
| `Arrow Down` | Decrease scroll speed |
| `Arrow Left` / `Arrow Right` | Jump backward / forward |
| `Shift+Arrow Left` / `Shift+Arrow Right` | Big jump backward / forward |
| `M` | Toggle horizontal mirror |
| `V` | Toggle vertical mirror |
| `+` / `=` | Increase font size |
| `-` / `_` | Decrease font size |
| `H` | Toggle shortcut help overlay |
| `Ctrl+S` | Save script (editor view) |

## Download and Install

[![Latest Release](https://img.shields.io/github/v/release/JHenriquesss/Teleprompter?include_prereleases&label=download&logo=github)](https://github.com/JHenriquesss/Teleprompter/releases)

**OpenPrompter RS is free, open-source, and works entirely offline.** No account. No login. No payment. No cloud. No telemetry.

The current public beta is **[v0.9.0-beta.1](https://github.com/JHenriquesss/Teleprompter/releases/tag/v0.9.0-beta.1)**.

| Platform | Recommended download | Also available |
|----------|---------------------|----------------|
| Windows | `*_x64-setup.exe` — NSIS installer (handles upgrades) | MSI (enterprise), portable ZIP (no install) |
| Linux | `*.AppImage` — works on any distro, no install needed | `.deb` (Debian/Ubuntu) |
| macOS | `*_aarch64.dmg` — drag to Applications | (Apple Silicon only) |

All builds are **unsigned beta releases**. Your operating system may show a warning — this is normal for open-source software that hasn't paid for code signing certificates. See the [installation guide](docs/install.md) for step-by-step help.

### Quick start

**Windows:** Download `OpenPrompter.RS_0.9.0_x64-setup.exe`, double-click, follow the installer.

**Linux:** Download `OpenPrompter.RS_0.9.0_amd64.AppImage`, run:
```bash
chmod +x *.AppImage && ./OpenPrompter.RS_*.AppImage
```

**macOS:** Download `OpenPrompter.RS_0.9.0_aarch64.dmg`, open it, drag the app to Applications. Right-click → Open on first launch (unsigned).

> Read the full [installation guide](docs/install.md) — it explains SmartScreen, Gatekeeper, and which file to pick.

## Platform Support

| Platform | Status | Manually tested | Notes |
|----------|--------|----------------|-------|
| Windows | ✅ Primary | Yes — install, launch, uninstall all verified | Most tested platform |
| Linux | ⚠️ Beta | CI only | AppImage + deb generated. Community testing welcome. |
| macOS | ⚠️ Beta | CI only | DMG generated (ARM). Community testing welcome. |

Windows is the primary development platform and receives the most manual testing. Linux and macOS builds are generated automatically by CI for every release but have **not been manually tested on real hardware**. If you try them and run into issues, please [open an issue](https://github.com/JHenriquesss/Teleprompter/issues) — your feedback helps us improve cross-platform support.

## Building from Source

### Prerequisites

| Tool | Install |
|------|---------|
| Rust (stable) | [rustup.rs](https://rustup.rs/) |
| WASM target | `rustup target add wasm32-unknown-unknown` |
| Trunk | `cargo install trunk --locked` |
| wasm-pack | `cargo install wasm-pack --locked` (for frontend tests) |
| Tauri CLI | `cargo install tauri-cli --version "^2"` |

### Platform Dependencies

**Windows:**
- Microsoft Visual Studio C++ Build Tools or [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)

**Linux:**
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

**macOS:**
```bash
xcode-select --install
```

### Development

```bash
git clone <repo-url>
cd teleprompter
cargo tauri dev
```

This starts the Trunk dev server and launches the Tauri desktop window with hot-reload.

### Production Build

```bash
cargo tauri build
```

The installer will be in `src-tauri/target/release/bundle/`.

## Validation

Run the full validation pipeline before committing:

**Linux / macOS:**
```bash
./scripts/check.sh
```

**Windows PowerShell:**
```powershell
.\scripts\check.ps1
```

This runs:

| Step | What it validates |
|------|-------------------|
| `cargo fmt --all -- --check` | Code formatting |
| `cargo check -p openprompter-rs-tauri --all-targets` | Backend compilation |
| `cargo test -p openprompter-rs-tauri` | Backend unit + integration tests (18+) |
| `cargo clippy -p openprompter-rs-tauri --all-targets --all-features -- -D warnings` | Linting (zero warnings) |
| `trunk build` | Frontend WASM compilation |
| `wasm-pack test --headless --chrome` | Frontend WASM unit tests (11+) |

> The frontend crate (`openprompter-rs`) is excluded from `cargo check` and `cargo test` at the workspace level because it depends on `web-sys` / `wasm-bindgen` (WASM-only targets). Trunk + wasm-pack handle frontend validation.

## Backend Tests

```bash
cargo test -p openprompter-rs-tauri
```

## Project Structure

```
teleprompter/
├── src/                    # Leptos frontend (Rust → WASM)
│   ├── main.rs            # Entry point
│   ├── app.rs             # Root component
│   ├── components/        # UI components
│   ├── state/             # Reactive state management
│   ├── prompter/          # Animation engine
│   └── bindings/          # Tauri API bindings
├── src-tauri/             # Tauri backend (native Rust)
│   ├── src/
│   │   ├── commands/      # Command handlers
│   │   ├── domain/        # Domain models
│   │   ├── services/      # Business logic
│   │   ├── persistence/   # SQLite database
│   │   └── adapters/      # System integration
│   └── tauri.conf.json
├── src-tauri/tests/       # Backend integration tests
├── scripts/               # check.sh / check.ps1
├── docs/screenshots/      # Application screenshots
├── rust-toolchain.toml    # Rust toolchain pinning
├── .github/workflows/     # GitHub Actions CI
├── Cargo.toml             # Workspace root
└── Trunk.toml             # Trunk config
```

## Architecture

- **Animation loop** runs on the frontend using `requestAnimationFrame` — zero backend calls during playback
- **Tauri commands** handle persistence, settings, and system integration only
- **SQLite** stores all scripts and settings locally
- **No JavaScript** — the entire application is written in Rust

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) and our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
