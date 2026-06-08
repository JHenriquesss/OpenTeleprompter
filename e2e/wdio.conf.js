// WebdriverIO config for OpenPrompter end-to-end tests.
//
// Drives the REAL built desktop binary through `tauri-driver`, which proxies to
// the platform's native WebView WebDriver (Linux: WebKitWebDriver, Windows:
// msedgedriver). This is the only test layer that exercises the real
// `window.__TAURI__` IPC bridge — i.e. the layer where the dead-buttons bug
// (missing `withGlobalTauri`) lived. Unit + MockApi tests cannot see it.
//
// Prereqs (see e2e/README.md):
//   cargo install tauri-driver --locked
//   Linux: apt-get install webkit2gtk-driver xvfb  (run under `xvfb-run`)
//   the app binary must be built first (trunk build && cargo build --release)

const os = require("os");
const path = require("path");
const { spawn, spawnSync } = require("child_process");

const isWindows = process.platform === "win32";

// Built (unbundled) binary at the workspace target dir.
const binName = isWindows
  ? "openprompter-rs-tauri.exe"
  : "openprompter-rs-tauri";
const application = path.resolve(
  __dirname,
  "..",
  "target",
  "release",
  binName,
);

const tauriDriverPath = path.resolve(
  os.homedir(),
  ".cargo",
  "bin",
  isWindows ? "tauri-driver.exe" : "tauri-driver",
);

let tauriDriver;

exports.config = {
  runner: "local",
  specs: ["./test/specs/**/*.e2e.js"],
  maxInstances: 1,
  capabilities: [
    {
      "tauri:options": {
        application,
      },
    },
  ],
  // tauri-driver listens here and forwards to the native webview driver.
  hostname: "127.0.0.1",
  port: 4444,
  path: "/",
  logLevel: "info",
  framework: "mocha",
  reporters: ["spec"],
  mochaOpts: {
    ui: "bdd",
    timeout: 60000,
  },

  // Make sure the binary exists before the run. We don't build here so CI can
  // control build flags / caching; fail loudly if missing.
  onPrepare: () => {
    const fs = require("fs");
    if (!fs.existsSync(application)) {
      throw new Error(
        `App binary not found at ${application}. Build it first: ` +
          `trunk build && cargo build --release -p openprompter-rs-tauri`,
      );
    }
  },

  beforeSession: () => {
    tauriDriver = spawn(tauriDriverPath, [], {
      stdio: [null, process.stdout, process.stderr],
    });
  },

  afterSession: () => {
    if (tauriDriver) tauriDriver.kill();
  },
};
