# Dev-Toolbox

A modular and extensible CLI toolbox for developers, built with Rust. This application provides a collection of tools to streamline common development tasks, from analyzing GitHub repositories to inspecting Unicode characters.

## Features

- **Modular Design:** Easily extend the toolbox by adding new tools that implement the `Tool` trait.
- **Cross-Platform:** Builds and runs on both Windows and Linux.
- **Mouse and Touchpad Support:** Navigate between tabs with a simple click or tap.

## Available Tools

Detailed documentation for each tool can be found in the [Wiki](docs/WIKI.md).

- **[Org Research](docs/org_research.md):** Get insights into a GitHub organization's public repositories, including language statistics and license information.
- **[Repo Explorer](docs/repo_explorer.md):** Explore the contents of a public GitHub repository, view file details, and see a breakdown of the languages used.
- **[Unicode Inspector](docs/unicode_inspector.md):** Look up Unicode characters by their code point and view detailed information about them.
- **[JWT Decoder](docs/jwt_decoder.md):** Decode JSON Web Tokens to inspect their header and payload.

## Getting Started

### Prerequisites

- **Rust:** [Install Rust](https://www.rust-lang.org/tools/install)
- **Git:** [Install Git](https://git-scm.com/downloads/)

### Installation and Usage

1. **Clone the repository:**

   ```bash
   git clone https://github.com/fam007e/dev-toolbox.git
   cd dev-toolbox
   ```

2. **Secrets Configuration:**

   Create a `.env` file in the root of the project to store your GitHub Personal Access Token.

   ```
   GITHUB_TOKEN=your_github_token
   ```

   Generate a token [here](https://github.com/settings/tokens).

3. **Application Configuration:**

   On the first run, the app will generate a `config.toml` in your OS-specific config directory:
   - **Linux:** `~/.config/dev-toolbox/config.toml`
   - **Windows:** `%AppData%\Roaming\dev-toolbox\config.toml`

   You can modify this file to change database paths or API URLs.

4. **Build and run:**

   ```bash
   cargo run --release
   ```

## Navigation

- **Keyboard:**
  - `Tab`: Switch between tool tabs.
  - **Arrow Keys, Enter, etc.:** Used for interacting with the currently selected tool.
- **Mouse / Touchpad:**
  - **Click / Tap:** Select a tab to switch to that tool.

## Project Maintenance

- **CI/CD:** Automated builds, tests, and linting are performed on every PR via GitHub Actions.
- **Standards:** We adhere to strict Rust coding standards (Clippy) and have a comprehensive [Code of Conduct](CODE_OF_CONDUCT.md).
- **Security:** Our security protocols are detailed in [SECURITY.md](SECURITY.md).
- **Contributing:** See [CONTRIBUTING.md](CONTRIBUTING.md) for how to add new tools.

## Contact

For support or reporting issues, contact [email](mailto:faisalmoshiur+devtoolbox@gmail.com).
