# End-to-end tests (real app, real IPC)

These tests launch the **actual built desktop binary** and click **real buttons**
through [`tauri-driver`](https://v2.tauri.app/develop/tests/webdriver/). Every
step crosses the real `window.__TAURI__` IPC bridge — the layer where the
"buttons do nothing" bug lived (`withGlobalTauri` was off, so every `invoke`
threw). Unit tests and the `MockApi` component tests **cannot** catch that class
of bug because they never touch the real bridge.

This is what lets us ship updates without hand-testing the whole app each time.

## What it covers

`test/specs/smoke.e2e.js`:
- frontend mounts (library shell visible)
- **+ New Script** → `create_script` → editor opens pre-filled
- edit + Ctrl+S → `update_script`, then back to library → `list_scripts` shows it
- open Settings → `get_settings`

If the IPC bridge is broken, these fail.

## Platforms

`tauri-driver` supports **Linux** (WebKitWebDriver) and **Windows**
(msedgedriver). No macOS WebDriver exists for WKWebView. The IPC bridge is
OS-independent, so Linux CI is sufficient to guard the regression; Windows can
be added later.

## Run locally (Windows)

```powershell
# one-time
cargo install tauri-driver --locked
# msedgedriver must match your installed Edge/WebView2 version and be on PATH
#   (https://developer.microsoft.com/microsoft-edge/tools/webdriver/)

# build the app (embeds the frontend)
trunk build
cargo build --release -p openprompter-rs-tauri

cd e2e
npm install
npm test
```

## Run locally (Linux)

```bash
sudo apt-get install -y webkit2gtk-driver xvfb
cargo install tauri-driver --locked
trunk build
cargo build --release -p openprompter-rs-tauri
cd e2e && npm install
xvfb-run -a npm test
```

## CI

The `e2e` job in `.github/workflows/ci.yml` runs this on every PR/push (Linux,
headless via `xvfb`).
