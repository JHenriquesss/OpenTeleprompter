# CLAUDE.md — OpenTeleprompter project rules

Local-first offline desktop teleprompter. Tauri v2 + Rust (rusqlite) backend, Leptos CSR (WASM) frontend. No cloud, auth, telemetry, AI, paid features, or code signing.

## Git / shell

- **Multi-line git messages: use bash heredoc, never PowerShell here-strings.** Commits run through the Bash tool. `git commit -m @'...'@` is PowerShell syntax — under bash it injects literal `@` into the subject (produced `@ (#2)` on main once). Always:
  ```bash
  git commit -F - <<'EOF'
  subject line
  
  body
  EOF
  ```
- **Never push directly to `main`.** Default branch is PR-only (auto-mode blocks direct push). Branch → commit → PR → wait CI `validate` green → squash-merge.
- End commit messages with `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`; end PR bodies with the Claude Code footer.
- Don't commit build-regenerated `src-tauri/gen/schemas/*.json` — `git checkout -- src-tauri/gen/schemas/` before staging.

## CI gates (`.github/workflows/ci.yml`)

`validate` job is the PR gate: fmt · `cargo check -p openprompter-rs-tauri --all-targets` · `cargo test -p openprompter-rs-tauri` · `cargo clippy -p openprompter-rs-tauri --all-targets --all-features -- -D warnings` · `trunk build` · `wasm-pack test --headless --chrome`.

- **Frontend clippy is NOT gated** — wasm crate has baseline warnings. Don't chase them; just don't add new ones gratuitously.
- `build-linux/macos/windows` jobs come from `release.yml` (any `v*` tag, incl. phase tags) — noise on PRs, not gates.
- Local full check: `.\scripts\check.ps1`. Windows release: `cargo tauri build` (MSI + NSIS).

## Frontend testing (Phase 11+)

- Components depend on `AppApi` trait via `use_context::<ApiCtx>()` (`ApiCtx = Rc<dyn AppApi>`). Production = `RealTauriApi`; tests = `MockApi` (`#[cfg(test)]`). Never call `tauri_api::*` directly from a component.
- Component tests are inline in `src/component_tests.rs` (so `cfg(test)` is true and `MockApi` is visible — a `tests/` integration crate cannot see it).
- Mounted-component tests settle async with the bounded `tick`/`settle` poll — never a fixed `sleep`, never depend on toast 4 s auto-dismiss timing.
- `Rc<dyn AppApi>` is not `Copy`: handlers used >once / inside reactive `Fn` closures / list `.map` must be Leptos `Callback`; single-use handlers clone the `Rc` inline.

## Pipeline / docs

- Dev uses the `/dv` phase pipeline. `.dv-state.json` is **local orchestration** — its phase-close edits are not pushed (PR-only policy); leave modified locally.
- This repo has **no `sessions/` dir**; `/wiki` consolidates from the active conversation, not session logs.
- Wiki lives in `wiki/` (LLM-consumer docs). Update via PR.
