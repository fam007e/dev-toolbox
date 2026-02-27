# Changelog

All notable changes to this project will be documented in this file.

## [1.1.0] - 2026-02-27

### Added
- **Asynchronous Tool Architecture**: Implemented `async-trait` for the `Tool` trait, enabling non-blocking I/O operations for all tools.
- **GitHub API Integration**: 
  - `Repo Explorer` now fetches real user repositories and latest releases.
  - `Org Research` now implements GitHub organization search.
- **Configuration System**: Added `config.toml` support via `toml` and `dirs` crates for cross-platform portability.
- **Lazy Loading**: Implemented background data import for `Unicode Inspector` with TUI loading indicators.
- **Unit Tests**: Added initial test suite for JWT decoding and GitHub model parsing.
- **CI/CD**: Integrated GitHub Actions for automated building, testing, linting (`clippy`), and spellchecking (`typos`).
- **Community Standards**: Added `CODE_OF_CONDUCT.md`, `SECURITY.md`, and updated `CONTRIBUTING.md`.

### Changed
- **Navigation Shortcuts**: 
  - `Ctrl+Q` to quit.
  - `Ctrl+C` to copy status to clipboard.
  - Updated tool-specific shortcuts to use `Ctrl+` modifiers for input safety.
- **Storage Paths**: Moved `cache.db` and `config.toml` to OS-specific standard directories (e.g., `~/.config/dev-toolbox`).

### Fixed
- Fixed UI "freezing" during startup while importing Unicode data.
- Resolved several compiler warnings related to unused imports and dead code.
- Hardcoded file paths replaced with configurable settings.

## [1.0.0] - Initial Release

- Basic TUI application with modular tool structure.
- Placeholder implementations for Unicode Inspector, JWT Decoder, Repo Explorer, and Org Research.
