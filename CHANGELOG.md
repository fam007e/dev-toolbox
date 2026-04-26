# Changelog

All notable changes to this project will be documented in this file.

## [v2026.04.26] - 2026-04-26
 
 ### Features (Phase 3)
 - **Global Search Palette**: Integrated `Ctrl+F` search interface for instant navigation between tools.
 - **Token Inspector**: New tool for viewing authenticated GitHub token details, owner information, and rate limits.
 - **Encoder/Decoder**: New utility supporting Base64, Hex, and URL encoding/decoding.
 - **HTTP Request Inspector**: Generic TUI for making arbitrary HTTP requests with header and body inspection.
 - **Dynamic Tool Loading**: Tools now support asynchronous background initialization (e.g., Token Inspector auto-check).
 
 ### Modernization & Security
 - **Security Hardening**: Centralized GitHub scope validation with strict exact-match logic.
 - **UI Safety**: Replaced brittle layout indexing with named dynamic constraints to prevent panics.
 - **Connection Pooling**: Shifted to a shared `reqwest::Client` with per-request authentication scheme (`Bearer`).
 - **Test Robustness**: Resolved race conditions in secret-loading tests using `serial_test`.
 - **CI/CD Pinning**: All GitHub actions pinned to immutable SHAs for maximum supply-chain security.
 - **Strict Code Quality**: Achieved 100% Clippy compliance across the entire codebase.
 
 ### Performance
 - **Optimized Response Handling**: Added 10,000-character truncation to HTTP bodies for memory and rendering efficiency.
 - **Dependency Pruning**: Leaner `Tokio` feature set to reduce binary size and compilation time.
 
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
