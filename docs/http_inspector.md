# HTTP Inspector
 
 The HTTP Inspector tool allows you to perform arbitrary HTTP requests and inspect the response status, headers, and body.
 
 ## Usage
 
 1. Switch to the **HTTP Inspector** tab using `Tab` or the search palette (`Ctrl+F`).
 2. Select the **HTTP Method** (GET, POST, PUT, DELETE).
 3. Type the **URL** you wish to request.
 4. Press `Enter` to send the request.
 
 ## Response Details
 
 - **Status:** The HTTP status code of the response (e.g., 200 OK, 404 Not Found).
 - **Headers:** A list of headers returned by the server.
 - **Body:** The response body text. For performance and memory safety, bodies longer than 10,000 characters are truncated.
 
 ## Keyboard Shortcuts
 
 - `Ctrl+M`: Cycle through HTTP methods (**GET**, **POST**, **PUT**, **DELETE**).
 - `Enter`: Send the HTTP request.
 - `Backspace`: Remove the last character from the URL.
 - `Any Character`: Append the character to the URL.
 
 ---
 [Back to Wiki](WIKI.md) | [Back to README](../README.md)
