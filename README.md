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

#### Option 1: Download Binary (Recommended for Users)
Download the latest pre-built binary for your OS from the [Releases](https://github.com/fam007e/dev-toolbox/releases) page.

#### Option 2: Build from Source
1. **Clone the repository:**
   ```bash
   git clone https://github.com/fam007e/dev-toolbox.git
   cd dev-toolbox
   ```
2. **Build and run:**
   ```bash
   cargo run --release
   ```

## Configuration

### 1. Secrets (.env)
The application requires a GitHub Personal Access Token to fetch repository and organization data. Create a `.env` file containing:
```
GITHUB_TOKEN=your_github_token
```
You can place this file in:
- The **current working directory** where you run the app.
- The **OS-specific config directory** (see below).

Generate a token [here](https://github.com/settings/tokens).

### 2. Application Config (config.toml)
On the first run, the app generates a `config.toml` in your OS-specific config directory:
- **Linux:** `~/.config/dev-toolbox/`
- **Windows:** `%AppData%\Roaming\dev-toolbox\`

You can modify this file to change database paths or API URLs.

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
