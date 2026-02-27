use secrecy::{ExposeSecretMut, SecretBox};
use std::error::Error;

use zeroize::Zeroize;

#[derive(Clone)]
pub struct Secrets {
    pub github_token: SecretBox<str>,
}

impl Secrets {
    pub fn load(env_path: &str) -> Result<Self, Box<dyn Error>> {
        // 1. Try specified path (defaults to .env in CWD)
        if std::path::Path::new(env_path).exists() {
            dotenv::from_filename(env_path).ok();
        } else {
            // 2. Fallback to OS-specific config directory
            if let Some(mut config_dir) = dirs::config_dir() {
                config_dir.push("dev-toolbox");
                config_dir.push(".env");
                if config_dir.exists() {
                    dotenv::from_filename(config_dir).ok();
                }
            }
        }

        // 3. Load from environment (populated by dotenv or set manually)
        let github_token = std::env::var("GITHUB_TOKEN")
            .map(|s| SecretBox::new(s.into_boxed_str()))
            .unwrap_or_else(|_| SecretBox::new(String::new().into_boxed_str()));
        Ok(Secrets { github_token })
    }
}

impl Drop for Secrets {
    fn drop(&mut self) {
        self.github_token.expose_secret_mut().zeroize();
    }
}
