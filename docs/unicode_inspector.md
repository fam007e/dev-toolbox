# Unicode Inspector Tool

Look up detailed Unicode character data with instant-start performance.

## How to Use

The tool features **lazy loading**. On the first run, it imports the Unicode database in the background. A "Loading..." screen will appear while this process is active, but subsequent starts are instantaneous.

1. **Enter Text/Input:** Type directly to analyze individual graphemes.
2. **Enter Codepoint:** Search by a specific hexadecimal code (e.g., `1F60A`).
3. **Enter Name:** Search by the official Unicode name (e.g., `SMILE`).

## Keybindings

- `Up / Down`: Switch between input fields (Input, Codepoint, Name).
- `Enter`: Analyze the text field.
- `Ctrl+L`: Perform a database lookup using the Codepoint or Name field.
- `Ctrl+A`: Toggle "Sequential Mode" for text analysis.
- `Ctrl+E`: Export current character results to `unicode_results.json`.
