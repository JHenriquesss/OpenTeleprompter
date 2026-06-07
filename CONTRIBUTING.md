# Contributing to OpenPrompter RS

Thank you for your interest in contributing! This project is in its early stages and welcomes all forms of contribution.

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

1. Fork the repository.
2. Clone your fork: `git clone https://github.com/your-username/openprompter-rs.git`
3. Create a feature branch: `git checkout -b feat/my-feature`
4. Make your changes.
5. Run tests: `cargo test`
6. Commit with a clear message.
7. Push and open a Pull Request.

## Development Setup

See [README.md](README.md#development) for setup instructions.

## Architecture Guidelines

- **No cloud dependencies** — everything must work offline.
- **Frontend animation** — teleprompter scrolling uses `requestAnimationFrame`, not Tauri commands.
- **Tauri commands** — for persistence, settings, and system integration only.
- **No JavaScript** — use Rust/WASM for all frontend logic.
- **Error handling** — use `thiserror` for backend errors, `Result<T, String>` for Tauri commands.

## Pull Request Process

1. Update the README or documentation if needed.
2. Add tests for new functionality.
3. Ensure all tests pass.
4. Keep PRs focused — one feature or fix per PR.
5. Use Conventional Commits for commit messages.

## Commit Conventions

```
feat: add horizontal mirror mode
fix: prevent scroll jump on prompter restart
docs: update README with keyboard shortcuts
refactor: extract animation loop into engine module
test: add persistence roundtrip tests
```

## Questions?

Open an issue for discussion before starting significant work.
