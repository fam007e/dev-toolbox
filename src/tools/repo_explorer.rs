use crate::models::github::Repository;
use crate::db::Database;
use std::sync::{Arc, Mutex};
use crate::secrets::Secrets;
use reqwest::Client;
use rusqlite::params;
use serde_json;
use std::error::Error;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct RepoExplorerTool {
    input: String,
    results: Vec<Repository>,
    #[allow(dead_code)]
    db: Arc<Mutex<Database>>,
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    secrets: Secrets,
}

impl RepoExplorerTool {
    pub fn new(db: Arc<Mutex<Database>>, client: &Client, secrets: &Secrets) -> Result<Self, Box<dyn Error>> {
        Ok(RepoExplorerTool {
            input: String::new(),
            results: Vec::new(),
            db: db,
            client: client.clone(),
            secrets: secrets.clone(),
        })
    }

    fn fetch_repos(&mut self) -> Result<String, Box<dyn Error>> {
        Ok("Repository fetching not implemented".into())
    }
}

impl super::Tool for RepoExplorerTool {
    fn name(&self) -> &'static str {
        "Repo Explorer"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let input = Paragraph::new(self.input.as_str())
            .block(Block::default().borders(Borders::ALL).title(Line::from(Span::styled("Repo Input", Style::default().fg(Color::Green)))));
        f.render_widget(input, chunks[0]);

        let results = Paragraph::new(self.results.iter().map(|repo| Line::from(repo.name.clone())).collect::<Vec<_>>())
            .block(Block::default().borders(Borders::ALL).title(Line::from(Span::styled("Repo Results", Style::default().fg(Color::Green)))));
        f.render_widget(results, chunks[1]);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<String, Box<dyn Error>> {
        match key.code {
            KeyCode::Enter => self.fetch_repos(),
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
            _ => Ok(String::new())
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
