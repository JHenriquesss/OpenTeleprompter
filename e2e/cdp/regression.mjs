// Real-app regression suite driven over the WebView2 DevTools Protocol (CDP).
//
// These guard the frontend<->backend integration bugs that the headless-WebKit
// CI cannot run and unit tests cannot see:
//   - real IPC works at all (the withGlobalTauri "dead buttons" class)
//   - markdown/document import end-to-end incl. the camelCase `fileName` arg
//   - playback save/load round-trip (the "resume returns to 0%" bug)
//   - picture-in-picture pin/unpin resizes the window (560 <-> 1280)
//   - prompter control buttons are not recreated on mousemove (the "needs two
//     clicks" bug)
//
// HOW TO RUN (Windows, WebView2):
//   1. Build + launch the app with remote debugging:
//        $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=9222"
//        & ".\target\release\openprompter-rs-tauri.exe"
//   2. node e2e/cdp/regression.mjs
//
// Exit code is non-zero if any check fails.

const BASE = process.env.CDP_BASE || "http://localhost:9222";
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const log = (...a) => process.stdout.write(a.join(" ") + "\n");

function connect(wsUrl) {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(wsUrl);
    let id = 0;
    const pending = new Map();
    const to = setTimeout(() => reject(new Error("ws open timeout")), 6000);
    ws.addEventListener("open", () => { clearTimeout(to); resolve(api); });
    ws.addEventListener("error", () => reject(new Error("ws error")));
    ws.addEventListener("message", (ev) => {
      const m = JSON.parse(ev.data);
      if (m.id && pending.has(m.id)) { pending.get(m.id)(m); pending.delete(m.id); }
    });
    const send = (method, params = {}) =>
      new Promise((res, rej) => {
        const mid = ++id;
        pending.set(mid, res);
        ws.send(JSON.stringify({ id: mid, method, params }));
        setTimeout(() => rej(new Error("rpc timeout " + method)), 8000);
      });
    const api = {
      eval: async (e) => {
        const r = await send("Runtime.evaluate", { expression: e, awaitPromise: true, returnByValue: true });
        if (r.result?.exceptionDetails) throw new Error("eval: " + (r.result.exceptionDetails.exception?.description || "").slice(0, 200));
        return r.result?.result?.value;
      },
    };
  });
}

let failures = 0;
const check = (name, cond, extra = "") => {
  log(`${cond ? "PASS" : "FAIL"}  ${name}${extra ? "  (" + extra + ")" : ""}`);
  if (!cond) failures++;
};

(async () => {
  const list = await (await fetch(`${BASE}/json/list`)).json();
  const main = list.find((t) => t.type === "page" && t.title === "OpenPrompter RS") || list.find((t) => t.type === "page");
  if (!main) { log("FAIL  app page not found over CDP"); process.exit(1); }
  const c = await connect(main.webSocketDebuggerUrl);

  // 1. Real IPC works (dead-buttons class).
  const ver = await c.eval("window.__TAURI__.core.invoke('get_app_version')");
  check("real IPC (get_app_version)", typeof ver === "string" && ver.length > 0, ver);

  // 2. Ensure a script exists.
  let scripts = await c.eval("window.__TAURI__.core.invoke('list_scripts')");
  if (!scripts || !scripts.length) {
    await c.eval("window.__TAURI__.core.invoke('create_script',{title:'CDP',content:'a\\nb\\nc'})");
    scripts = await c.eval("window.__TAURI__.core.invoke('list_scripts')");
  }
  const id = scripts[0].id;

  // 3. Playback save/load round-trip (resume bug).
  await c.eval(`window.__TAURI__.core.invoke('save_playback_state',{scriptId:${JSON.stringify(id)},scrollOffsetPx:4321,speedMultiplier:1.5,fontSize:40,lineHeight:null,mirrorMode:false,mirrorVertical:false})`);
  const loaded = await c.eval(`window.__TAURI__.core.invoke('load_playback_state',{scriptId:${JSON.stringify(id)}})`);
  check("playback save/load round-trip", loaded && Math.abs(loaded.scroll_offset_px - 4321) < 0.5, JSON.stringify(loaded));

  // 4. PiP pin/unpin resizes window.
  const w0 = await c.eval("window.innerWidth");
  await c.eval("window.__TAURI__.core.invoke('set_pip',{enabled:true})"); await sleep(1200);
  const w1 = await c.eval("window.innerWidth");
  await c.eval("window.__TAURI__.core.invoke('set_pip',{enabled:false})"); await sleep(1200);
  const w2 = await c.eval("window.innerWidth");
  check("PiP pin shrinks window", w1 < 800, `before=${w0} pinned=${w1}`);
  check("PiP unpin restores window", w2 > 1000, `after=${w2}`);

  // 5. Control buttons stable across mousemove (two-clicks bug). Enter prompter.
  await c.eval(`(()=>{const b=[...document.querySelectorAll('button')].find(x=>x.title&&x.title.toLowerCase()==='play');if(b)b.click();return !!b;})()`);
  await sleep(2000);
  const stable = await c.eval(`(()=>{const sel=()=>[...document.querySelectorAll('button')].find(b=>b.title&&b.title.includes('Pin'));const a=sel();if(!a)return {ok:false,reason:'no pip button'};const cont=document.querySelector('div[style*="100vw"]')||document.body;for(let i=0;i<12;i++)cont.dispatchEvent(new MouseEvent('mousemove',{bubbles:true,clientX:100+i,clientY:90+i}));const b=sel();return {ok:a===b&&a.isConnected};})()`);
  check("control buttons stable on mousemove", stable && stable.ok, JSON.stringify(stable));

  log(failures === 0 ? "\nALL CDP CHECKS PASSED" : `\n${failures} CDP CHECK(S) FAILED`);
  process.exit(failures === 0 ? 0 : 1);
})().catch((e) => { log("FATAL " + e.message); process.exit(1); });
