use crate::secrets::Secrets;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use reqwest::Client;
use std::error::Error;

use std::sync::{Arc, Mutex};

pub struct TokenInspectorTool {
    client: Client,
    secrets: Secrets,
    loading: Arc<Mutex<bool>>,
    results: Arc<Mutex<Option<TokenInfo>>>,
    error: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Default, Clone)]
struct TokenInfo {
    owner_login: String,
    owner_name: Option<String>,
    scopes: String,
    scope_warning: Option<String>,
    rate_limit_limit: u32,
    rate_limit_remaining: u32,
    rate_limit_reset: i64,
}

impl TokenInspectorTool {
    pub fn new(client: &Client, secrets: &Secrets) -> Self {
        let tool = TokenInspectorTool {
            client: client.clone(),
            secrets: secrets.clone(),
            loading: Arc::new(Mutex::new(false)),
            results: Arc::new(Mutex::new(None)),
            error: Arc::new(Mutex::new(None)),
        };

        let mut tool_clone = tool.clone_state();
        tokio::spawn(async move {
            let _ = tool_clone.inspect_token().await;
        });

        tool
    }

    fn clone_state(&self) -> Self {
        TokenInspectorTool {
            client: self.client.clone(),
            secrets: self.secrets.clone(),
            loading: Arc::clone(&self.loading),
            results: Arc::clone(&self.results),
            error: Arc::clone(&self.error),
        }
    }

    async fn inspect_token(&mut self) -> Result<String, Box<dyn Error>> {
        use secrecy::ExposeSecret;
        let token = self.secrets.github_token.expose_secret();
        if token.is_empty() {
            *self.error.lock().unwrap() =
                Some("No GitHub token configured. Please check your .env file.".into());
            return Ok("Missing token".into());
        }

        *self.loading.lock().unwrap() = true;
        *self.error.lock().unwrap() = None;

        // Fetch User Info to get scopes and owner details
        let user_url = "https://api.github.com/user";
        let user_resp = self
            .client
            .get(user_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !user_resp.status().is_success() {
            *self.loading.lock().unwrap() = false;
            let err_msg = format!("GitHub API error (User): {}", user_resp.status());
            *self.error.lock().unwrap() = Some(err_msg.clone());
            return Err(err_msg.into());
        }

        let scope_warning = crate::github::check_token_scopes(user_resp.headers());

        let scopes = user_resp
            .headers()
            .get("x-oauth-scopes")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("No scopes returned")
            .to_string();

        let user_json: serde_json::Value = user_resp.json().await?;
        let owner_login = user_json["login"].as_str().unwrap_or("Unknown").to_string();
        let owner_name = user_json["name"].as_str().map(|s| s.to_string());

        // Fetch Rate Limits
        let rate_url = "https://api.github.com/rate_limit";
        let rate_resp = self
            .client
            .get(rate_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !rate_resp.status().is_success() {
            *self.loading.lock().unwrap() = false;
            let err_msg = format!("GitHub API error (Rate Limit): {}", rate_resp.status());
            *self.error.lock().unwrap() = Some(err_msg.clone());
            return Err(err_msg.into());
        }

        let rate_json: serde_json::Value = rate_resp.json().await?;
        let core_limit = rate_json["resources"]["core"]["limit"]
            .as_u64()
            .unwrap_or(0) as u32;
        let core_remaining = rate_json["resources"]["core"]["remaining"]
            .as_u64()
            .unwrap_or(0) as u32;
        let core_reset = rate_json["resources"]["core"]["reset"]
            .as_i64()
            .unwrap_or(0);

        *self.results.lock().unwrap() = Some(TokenInfo {
            owner_login,
            owner_name,
            scopes,
            scope_warning,
            rate_limit_limit: core_limit,
            rate_limit_remaining: core_remaining,
            rate_limit_reset: core_reset,
        });

        *self.loading.lock().unwrap() = false;
        Ok("Token inspected successfully".into())
    }
}

impl super::Tool for TokenInspectorTool {
    fn name(&self) -> &'static str {
        "Token Inspector"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let is_loading = *self.loading.lock().unwrap();
        if is_loading {
            let loading = Paragraph::new("Inspecting token...")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading, area);
            return;
        }

        let error_lock = self.error.lock().unwrap();
        if let Some(err) = &*error_lock {
            let error_para = Paragraph::new(err.as_str())
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error_para, area);
            return;
        }
        drop(error_lock);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let cta = Paragraph::new("Press Enter to re-inspect the current GitHub Token.")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(cta, chunks[0]);

        let mut lines = vec![];
        let results_lock = self.results.lock().unwrap();
        if let Some(info) = &*results_lock {
            lines.push(Line::from(vec![
                Span::styled("Owner Login: ", Style::default().bold()),
                Span::raw(&info.owner_login),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Owner Name:  ", Style::default().bold()),
                Span::raw(info.owner_name.as_deref().unwrap_or("None")),
            ]));
            lines.push(Line::from(""));
            let scope_style = if info.scope_warning.is_some() {
                Style::default().fg(Color::Red).bold()
            } else {
                Style::default().fg(Color::Green)
            };

            lines.push(Line::from(vec![
                Span::styled("Scopes:      ", Style::default().bold()),
                Span::styled(&info.scopes, scope_style),
            ]));
            if let Some(warning) = &info.scope_warning {
                lines.push(Line::from(vec![
                    Span::raw("             "),
                    Span::styled(warning, Style::default().fg(Color::Red)),
                ]));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("Rate Limit:  ", Style::default().bold()),
                Span::raw(format!(
                    "{}/{}",
                    info.rate_limit_remaining, info.rate_limit_limit
                )),
            ]));

            use std::time::{SystemTime, UNIX_EPOCH};
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            let reset_in = info.rate_limit_reset - now;
            let reset_str = if reset_in > 0 {
                format!("{} minutes", reset_in / 60)
            } else {
                "Resetting now...".to_string()
            };

            lines.push(Line::from(vec![
                Span::styled("Reset In:    ", Style::default().bold()),
                Span::raw(reset_str),
            ]));
        } else {
            lines.push(Line::from("No token info loaded yet."));
        }

        let results =
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(Line::from(
                Span::styled("Token Information", Style::default().fg(Color::Cyan)),
            )));
        f.render_widget(results, chunks[1]);
    }

    fn handle_input(&mut self, key: KeyEvent) -> crate::tools::ToolFuture<'_> {
        // We need to clone state for the async move block
        let mut tool_clone = self.clone_state();
        Box::pin(async move {
            match key.code {
                KeyCode::Enter => tool_clone.inspect_token().await,
                _ => Ok(String::new()),
            }
        })
    }
}
