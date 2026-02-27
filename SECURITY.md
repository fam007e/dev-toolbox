# Security Policy

## Supported Versions

Currently, only the latest version of Dev-Toolbox is supported with security updates.

## Reporting a Vulnerability

We take the security of this project seriously. If you believe you have found a security vulnerability, please do not open a public issue. Instead, please report it via [email](mailto:faisalmoshiur+devtoolbox@gmail.com) to the project maintainers.

## Security Features in Use

This project employs several security-focused Rust crates to protect sensitive information:

- **`secrecy`**: Used to wrap sensitive tokens (like GitHub API tokens) to prevent accidental logging or exposure.
- **`zeroize`**: Ensures that sensitive data is securely wiped from memory when it is no longer needed.
- **`https-only`**: All networking through `reqwest` is configured to enforce HTTPS.

## Credentials

Never commit your `.env` file or any other files containing real secrets. This project includes a `.gitignore` that excludes common secret files.
