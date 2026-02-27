# Contributing to Dev-Toolbox

We welcome contributions to the Dev-Toolbox! This guide will help you get started with adding new tools or improving existing ones.

## Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally.
3. **Create a new branch** for your feature or bugfix.

## Development Environment

- **Rust:** Ensure you have the latest stable Rust version installed.
- **SQLite:** The project uses `rusqlite` with the `bundled` feature, but you may need `libsqlite3-dev` on Linux for CI consistency.
- **Format & Lint:**
  - Run `cargo fmt` to format code.
  - Run `cargo clippy -- -D warnings` to check for common mistakes.

## Creating a New Tool

All tools must implement the `async_trait` version of the `Tool` trait.

1. **Implement the Trait:**
   ```rust
   #[async_trait]
   impl Tool for MyTool {
       fn name(&self) -> &'static str { "My Tool" }
       fn render(&self, f: &mut Frame, area: Rect) { ... }
       async fn handle_input(&mut self, key: KeyEvent) -> Result<String, Box<dyn Error>> { ... }
       fn save_cache(&self) -> Result<(), Box<dyn Error>> { ... }
   }
   ```
2. **Configuration:** Use the `Config` struct to avoid hardcoding paths.
3. **Lazy Loading:** If your tool requires heavy data loading, use `tokio::spawn` to load data in the background and show a loading state in `render`.

## CI Pipeline

Every Pull Request triggers a CI pipeline that runs:
- `cargo build`
- `cargo test`
- `cargo fmt --check`
- `cargo clippy`
- Spellcheck (`typos`)

Please ensure all checks pass before requesting a review.

## Submitting Your Contribution

1. **Commit your changes** with a clear and descriptive message.
2. **Push to your fork.**
3. **Create a Pull Request** against the `main` branch.
