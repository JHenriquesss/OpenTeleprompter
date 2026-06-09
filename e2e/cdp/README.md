# Real-app CDP regression suite

`regression.mjs` drives the **real built Windows app** over the WebView2
DevTools Protocol and checks the frontend↔backend integration bugs that the
headless-WebKit CI can't run and unit tests can't see:

| Check | Guards the bug |
|------|----------------|
| `get_app_version` over IPC returns | "dead buttons" (`withGlobalTauri` off → every `invoke` threw) |
| `save_playback_state` → `load_playback_state` round-trip | playback IPC contract (camelCase args) |
| `set_pip(true/false)` resizes window 560 ↔ 1280 | picture-in-picture pin/unpin |
| control button is the same node after 12 `mousemove`s | "needs two clicks" (controls recreated on mousemove) |

> The "resume restores the real scroll position" bug is a *frontend* logic bug
> (save read scroll_y after it was zeroed). That one is covered by the in-CI WASM
> test `exiting_prompter_saves_current_scroll_not_zero`. Markdown/PDF/DOCX text
> extraction is covered by the backend tests in `src-tauri/tests/`.

## Run (Windows, WebView2)

```powershell
# build + launch with remote debugging
$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=9222"
& ".\target\release\openprompter-rs-tauri.exe"

# in another shell
node e2e/cdp/regression.mjs
```

Exit code is non-zero if any check fails. Requires Node 21+ (global `WebSocket`).

This is a **manual/local** suite (needs a real WebView2 runtime + a debug port);
it is intentionally not wired into CI, where WebView2 isn't available.
