use crate::db::Database;
use crate::models::github::Organization;
use crate::secrets::Secrets;
use reqwest::Client;
use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use serde_json;
use std::error::Error;

pub struct OrgResearchTool {
    input: InputState,
    results: Vec<Organization>,
    #[allow(dead_code)]
    db: Arc<Mutex<Database>>,
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    secrets: Secrets,
    loading: bool,
}

struct InputState {
    parent_org: String,
    search_term: String,
    current_field: usize,
    allow_no_parent: bool,
}

use crate::models::github::SearchResponse;
use secrecy::ExposeSecret;

impl OrgResearchTool {
    pub fn new(
        db: Arc<Mutex<Database>>,
        client: &Client,
        secrets: &Secrets,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(OrgResearchTool {
            input: InputState {
                parent_org: String::new(),
                search_term: String::new(),
                current_field: 0,
                allow_no_parent: false,
            },
            results: Vec::new(),
            db,
            client: client.clone(),
            secrets: secrets.clone(),
            loading: false,
        })
    }

    async fn fetch_orgs(&mut self) -> Result<String, Box<dyn Error>> {
        self.loading = true;
        let query = if self.input.parent_org.is_empty() {
            self.input.search_term.clone()
        } else {
            format!("org:{} {}", self.input.parent_org, self.input.search_term)
        };

        let url = format!("https://api.github.com/search/users?q={}+type:org", query);
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

        let search_results: SearchResponse = resp.json().await?;
        self.results = search_results.items;
        self.loading = false;
        Ok(format!("Found {} organizations", self.results.len()))
    }
}

use async_trait::async_trait;

#[async_trait]
impl super::Tool for OrgResearchTool {
    fn name(&self) -> &'static str {
        "Org Research"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        if self.loading {
            let area = f.area();
            let loading = Paragraph::new("Searching GitHub Organizations...")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let parent_input = Paragraph::new(self.input.parent_org.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Parent Org",
                    Style::default().fg(Color::Green),
                ))),
        );
        f.render_widget(parent_input, chunks[0]);

        let search_input = Paragraph::new(self.input.search_term.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Search Term",
                    Style::default().fg(Color::Green),
                ))),
        );
        f.render_widget(search_input, chunks[1]);

        let allow_toggle =
            Paragraph::new(if self.input.allow_no_parent {
                "Yes"
            } else {
                "No"
            })
            .block(Block::default().borders(Borders::ALL).title(Line::from(
                Span::styled("Allow No Parent", Style::default().fg(Color::Green)),
            )));
        f.render_widget(allow_toggle, chunks[2]);

        let results =
            Paragraph::new(
                self.results
                    .iter()
                    .map(|org| Line::from(org.login.clone()))
                    .collect::<Vec<_>>(),
            )
            .block(Block::default().borders(Borders::ALL).title(Line::from(
                Span::styled("Org Results", Style::default().fg(Color::Green)),
            )));
        f.render_widget(results, chunks[3]);
    }

    async fn handle_input(&mut self, key: KeyEvent) -> Result<String, Box<dyn Error>> {
        match key.code {
            KeyCode::Up => {
                self.input.current_field = self.input.current_field.saturating_sub(1);
                Ok("Switched field".into())
            }
            KeyCode::Down => {
                self.input.current_field = (self.input.current_field + 1).min(2);
                Ok("Switched field".into())
            }
            KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.input.allow_no_parent = !self.input.allow_no_parent;
                Ok(format!("Allow No Parent: {}", self.input.allow_no_parent))
            }
            KeyCode::Enter => self.fetch_orgs().await,
            KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                serde_json::to_writer(std::fs::File::create("org_results.json")?, &self.results)?;
                Ok("Exported to org_results.json".into())
            }
            KeyCode::Char(c) => {
                match self.input.current_field {
                    0 => self.input.parent_org.push(c),
                    1 => self.input.search_term.push(c),
                    _ => {}
                }
                Ok("Input updated".into())
            }
            KeyCode::Backspace => match self.input.current_field {
                0 => {
                    self.input.parent_org.pop();
                    Ok("Removed character".into())
                }
                1 => {
                    self.input.search_term.pop();
                    Ok("Removed character".into())
                }
                _ => Ok("No action".into()),
            },
            _ => Ok(String::new()),
        }
    }

    fn save_cache(&self) -> Result<(), Box<dyn Error>> {
        // OrgResearchTool does not save cache to DB
        Ok(())
    }
}
