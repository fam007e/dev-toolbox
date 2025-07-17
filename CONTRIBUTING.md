# Contributing to Dev-Toolbox

We welcome contributions to the Dev-Toolbox! This guide will help you get started with adding new tools to the application.

## Getting Started

1. **Fork the repository:**

   Fork the `dev-toolbox` repository to your own GitHub account.

2. **Clone your fork:**

   ```
   git clone https://github.com/your-username/dev-toolbox.git
   ```

3. **Create a new branch:**

   ```
   git checkout -b my-new-tool
   ```

## Creating a New Tool

To add a new tool, you need to create a new Rust module and implement the `Tool` trait.

1. **Create a new file:**

   Create a new file for your tool in the `src/tools` directory (e.g., `src/tools/my_tool.rs`).

2. **Implement the `Tool` trait:**

   Your new tool must implement the `Tool` trait, which has the following methods:

   - `name(&self) -> &str`: Returns the name of the tool, which will be displayed in the tab.
   - `render(&self, f: &mut Frame, area: Rect)`: Renders the tool's UI.
   - `handle_input(&mut self, key: KeyEvent) -> Result<String, Box<dyn std::error::Error>>`: Handles user input for the tool.
   - `save_cache(&self) -> Result<(), Box<dyn std::error::Error>>`: Saves any cached data for the tool.

3. **Add your tool to `src/app.rs`:**

   In `src/app.rs`, add your new tool to the `tools` vector in the `App::new` function.

## Submitting Your Contribution

1. **Commit your changes:**

   ```
   git commit -m "Add my new tool"
   ```

2. **Push to your fork:**

   ```
   git push origin my-new-tool
   ```

3. **Create a pull request:**

   Open a pull request from your fork to the `main` branch of the `fam007e/dev-toolbox` repository.
