# Repo Explorer Tool

Explore repositories owned by a specific GitHub user.

## How to Use

1. **Enter GitHub Username:** Type the username of the account you want to explore.
2. **Fetch Data:** Press **Enter** to retrieve a list of public repositories.

The tool displays repository details and automatically fetches the **latest 5 releases** for each repository (if available).
 
 ## Security Note
 
 This tool automatically validates the scopes of your configured GitHub token. If broad permissions (like `repo`) are detected, a warning will be displayed recommending the use of fine-grained, read-only tokens for better security.

## Keybindings

- `Enter`: Fetch repositories for the entered user.
- `Ctrl+E`: Export current repository list and release data to `repo_results.json`.
