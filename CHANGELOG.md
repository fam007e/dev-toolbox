# Changelog

All notable changes to this project will be documented in this file.

## [v2026.02.28] - 2026-02-28

### Features
- **Modular TUI Architecture**: A flexible system for adding developer tools using the `Tool` trait.
- **Org Research**: Get insights into GitHub organizations, language statistics, and public repositories.
- **Repo Explorer**: Explore GitHub users' public repositories and their latest releases.
- **Unicode Inspector**: Instant-start, lazy-loading lookup for Unicode characters, names, and hex points.
- **JWT Decoder**: Decode JSON Web Tokens into structured JSON objects.
- **Asynchronous Engine**: Powered by `tokio` and `async-trait` for non-blocking I/O.
- **Persistent Storage**: SQLite-based caching for faster performance across sessions.
- **Configuration System**: OS-specific config and cache directory support (`config.toml`).

### Security & CI/CD
- **Hardenened CI/CD**: All GitHub Actions pinned to immutable commit SHAs with strict top-level permissions.
- **Automated Releases**: Cross-platform binaries (Linux & Windows) built and uploaded automatically on version tags.
- **CodeQL Integration**: Advanced security scanning enabled for Rust and GitHub Actions.
- **Credential Protection**: Uses `secrecy` and `zeroize` crates to protect GitHub API tokens.
- **Flexible Secret Loading**: Supports `.env` files in both current working directories and OS configuration folders.

### Documentation
- Comprehensive **Wiki** and tool-specific guides.
- Updated README with professional installation and configuration options.
