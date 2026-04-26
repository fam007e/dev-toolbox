use secrecy::{ExposeSecretMut, SecretBox};
use std::error::Error;

use zeroize::Zeroize;

#[derive(Clone)]
pub struct Secrets {
    pub github_token: SecretBox<str>,
}

impl Secrets {
    pub fn load(env_path: Option<&str>, allow_cwd: bool) -> Result<Self, Box<dyn Error>> {
        // 1. Try OS-specific config directory first (canonical location)
        if let Some(mut config_dir) = dirs::config_dir() {
            config_dir.push("dev-toolbox");
            config_dir.push(".env");
            if config_dir.exists() {
                dotenvy::from_filename(config_dir).ok();
            }
        }

        // 2. Try specified path if provided (e.g., via CLI override)
        if let Some(path) = env_path {
            if std::path::Path::new(path).exists() {
                dotenvy::from_filename(path).ok();
            }
        }

        // 3. Optional opt-in: Try .env in current directory
        if allow_cwd && std::path::Path::new(".env").exists() {
            dotenvy::from_filename(".env").ok();
        }

        // 4. Load from environment (populated by the above or set manually)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    #[serial_test::serial]
    fn test_secrets_load_default_ignores_cwd() {
        let temp_dir = tempfile::tempdir().unwrap();
        let env_path = temp_dir.path().join(".env");
        fs::write(&env_path, "GITHUB_TOKEN=malicious_token").unwrap();

        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        env::remove_var("GITHUB_TOKEN");

        let secrets = Secrets::load(None, false).unwrap();

        env::set_current_dir(original_dir).unwrap();

        use secrecy::ExposeSecret;
        assert_ne!(secrets.github_token.expose_secret(), "malicious_token");
    }

    #[test]
    #[serial_test::serial]
    fn test_secrets_load_with_allow_cwd() {
        let temp_dir = tempfile::tempdir().unwrap();
        let env_path = temp_dir.path().join(".env");
        fs::write(&env_path, "GITHUB_TOKEN=test_token_cwd").unwrap();

        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        env::remove_var("GITHUB_TOKEN");

        let secrets = Secrets::load(None, true).unwrap();

        env::set_current_dir(original_dir).unwrap();

        use secrecy::ExposeSecret;
        assert_eq!(secrets.github_token.expose_secret(), "test_token_cwd");
    }
}
