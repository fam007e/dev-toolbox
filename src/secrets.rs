use secrecy::{ExposeSecretMut, SecretBox};
use std::error::Error;

use zeroize::Zeroize;

#[derive(Clone)]
pub struct Secrets {
    pub github_token: SecretBox<str>,
}

impl Secrets {
    pub fn load(env_path: &str) -> Result<Self, Box<dyn Error>> {
        dotenv::from_filename(env_path).ok();
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
