# Token Inspector
 
 The Token Inspector tool allows you to view detailed information about the GitHub Personal Access Token (PAT) configured in your `.env` file.
 
 ## Usage
 
 1. Switch to the **Token Inspector** tab using `Tab` or the search palette (`Ctrl+F`).
 2. The tool automatically inspects your token on startup and displays the results.
 3. To re-inspect the token (e.g., after updating your `.env` file), press `Enter`.
 
 ## Information Displayed
 
 - **Owner Login:** The GitHub username associated with the token.
 - **Owner Name:** The full name of the user (if public).
 - **Scopes:** A list of permission scopes assigned to the token.
   - **Security Warning:** If the token has broad scopes (like `repo`), a warning will be displayed recommending the use of fine-grained tokens with read-only access.
 - **Rate Limit:** The number of remaining core API requests for the current hour.
 - **Reset In:** The time remaining until the rate limit resets.
 
 ## Keyboard Shortcuts
 
 - `Enter`: Re-inspect the configured GitHub token.
 
 ---
 [Back to Wiki](WIKI.md) | [Back to README](../README.md)
