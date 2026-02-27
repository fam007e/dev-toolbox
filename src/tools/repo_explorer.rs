use crate::db::Database;
use crate::models::github::Repository;
use crate::secrets::Secrets;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use reqwest::Client;
use rusqlite::params;
use serde_json;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct RepoExplorerTool {
    input: String,
    results: Vec<Repository>,
    #[allow(dead_code)]
    db: Arc<Mutex<Database>>,
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    secrets: Secrets,
    loading: bool,
}

use secrecy::ExposeSecret;

impl RepoExplorerTool {
    pub fn new(
        db: Arc<Mutex<Database>>,
        client: &Client,
        secrets: &Secrets,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(RepoExplorerTool {
            input: String::new(),
            results: Vec::new(),
            db,
            client: client.clone(),
            secrets: secrets.clone(),
            loading: false,
        })
    }

    async fn fetch_repos(&mut self) -> Result<String, Box<dyn Error>> {
        self.loading = true;
        let url = format!("https://api.github.com/users/{}/repos", self.input);
        let resp = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("token {}", self.secrets.github_token.expose_secret()),
            )
            .send()
            .await?;

        if !resp.status().is_success() {
            self.loading = false;
            return Err(format!("GitHub API error: {}", resp.status()).into());
        }

        let mut repos: Vec<Repository> = resp.json().await?;

        // Fetch releases for each repo (limited to first 5 for performance)
        for repo in repos.iter_mut().take(5) {
            let release_url = format!(
                "https://api.github.com/repos/{}/{}/releases",
                self.input, repo.name
            );
            let release_resp = self
                .client
                .get(&release_url)
                .header(
                    "Authorization",
                    format!("token {}", self.secrets.github_token.expose_secret()),
                )
                .send()
                .await?;
            if release_resp.status().is_success() {
                repo.releases = release_resp.json().await?;
            }
        }

        self.results = repos;
        self.loading = false;
        Ok(format!("Fetched {} repositories", self.results.len()))
    }
}

use async_trait::async_trait;

#[async_trait]
impl super::Tool for RepoExplorerTool {
    fn name(&self) -> &'static str {
        "Repo Explorer"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        if self.loading {
            let area = f.area();
            let loading = Paragraph::new("Fetching Repositories...")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let input = Paragraph::new(self.input.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Repo Input",
                    Style::default().fg(Color::Green),
                ))),
        );
        f.render_widget(input, chunks[0]);

        let results =
            Paragraph::new(
                self.results
                    .iter()
                    .map(|repo| Line::from(repo.name.clone()))
                    .collect::<Vec<_>>(),
            )
            .block(Block::default().borders(Borders::ALL).title(Line::from(
                Span::styled("Repo Results", Style::default().fg(Color::Green)),
            )));
        f.render_widget(results, chunks[1]);
    }

    async fn handle_input(&mut self, key: KeyEvent) -> Result<String, Box<dyn Error>> {
        match key.code {
            KeyCode::Enter => self.fetch_repos().await,
            KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                serde_json::to_writer(std::fs::File::create("repo_results.json")?, &self.results)?;
                Ok("Exported to repo_results.json".into())
            }
            KeyCode::Char(c) => {
                self.input.push(c);
                Ok("Input updated".into())
            }
            KeyCode::Backspace => {
                self.input.pop();
                Ok("Removed character".into())
            }
            _ => Ok(String::new()),
        }
    }

    fn save_cache(&self) -> Result<(), Box<dyn Error>> {
        let mut db = self.db.lock().unwrap();
        for repo in &self.results {
            db.conn().execute(
                "INSERT OR REPLACE INTO repos (username, name, data) VALUES (?1, ?2, ?3)",
                params![&self.input, &repo.name, serde_json::to_string(&repo)?],
            )?;
        }
        Ok(())
    }
}
