# Dev-Toolbox

A modular and extensible CLI toolbox for developers, built with Rust. This application provides a collection of tools to streamline common development tasks, from analyzing GitHub repositories to inspecting Unicode characters.

## Features

- **Modular Design:** Easily extend the toolbox by adding new tools that implement the `Tool` trait.
- **Cross-Platform:** Builds and runs on both Windows and Linux.
- **Mouse and Touchpad Support:** Navigate between tabs with a simple click or tap.

## Available Tools

- **Org Research:** Get insights into a GitHub organization's public repositories, including language statistics and license information.
- **Repo Explorer:** Explore the contents of a public GitHub repository, view file details, and see a breakdown of the languages used.
- **Unicode Inspector:** Look up Unicode characters by their code point and view detailed information about them.
- **JWT Decoder:** Decode JSON Web Tokens to inspect their header and payload.

## Getting Started

### Prerequisites

- **Rust:** [Install Rust](https://www.rust-lang.org/tools/install)
- **Docker (for Linux builds on Windows):** [Install Docker Desktop](https://www.docker.com/products/docker-desktop/)
- **Git:** [Install Git](https://git-scm.com/downloads/)

### Installation and Usage

1. **Clone the repository:**

   ```bash
   git clone https://github.com/fam007e/dev-toolbox.git
   cd dev-toolbox
   ```

2. **Configuration:**

   Create a `.env` file in the root of the project. This file is used to store your GitHub Personal Access Token, which is required for the `Org Research` and `Repo Explorer` tools.

   ```
   GITHUB_TOKEN=your_github_token
   ```

   You can generate a new token [here](https://github.com/settings/tokens).

3. **Build and run the application:**

   - **Windows:**

     ```bash
     cargo build --release
     ./target/release/dev-toolbox.exe
     ```

   - **Linux (and Linux on Windows with Docker):**

     ```bash
     # Build the Docker image
     docker build -t dev-toolbox-builder .

     # Run the application
     docker run -it dev-toolbox-builder /usr/src/dev-toolbox/target/release/dev-toolbox
     ```

## Navigation

- **Keyboard:**
  - `Ctrl+Q`: Quit the application.
  - `Ctrl+C`: Copy the current status message to clipboard.
  - `Tab`: Switch between tool tabs.
  - **Arrow Keys, Enter, etc.:** Used for interacting with the currently selected tool.

- **Mouse / Touchpad:**
  - **Click / Tap:** Select a tab to switch to that tool.

## Project Maintenance

- **CI/CD:** Automated builds, tests, and linting are performed on every PR via GitHub Actions.
- **Standards:** We adhere to strict Rust coding standards (Clippy) and have a comprehensive [Code of Conduct](CODE_OF_CONDUCT.md).
- **Security:** Our security protocols are detailed in [SECURITY.md](SECURITY.md).
- **Contributing:** See [CONTRIBUTING.md](CONTRIBUTING.md) for how to add new tools.
