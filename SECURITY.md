# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Open an issue on the repository with the "security" label. Do not disclose
security vulnerabilities publicly until they have been addressed.

## Scope

This application is fully offline and local-first. No data is transmitted over
the network. Security concerns are limited to:

- Local file system access
- SQLite database integrity
- Tauri webview security boundaries
