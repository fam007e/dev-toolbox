use crate::db::Database;
use crate::models::unicode::UnicodeChar;
use rusqlite::params;
use std::error::Error;
use std::fs;
use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use unicode_segmentation::UnicodeSegmentation;

use std::sync::atomic::{AtomicBool, Ordering};

pub struct UnicodeInspectorTool {
    input: InputState,
    results: Vec<UnicodeChar>,
    sequential: bool,
    db: Arc<Mutex<Database>>,
    is_loading: Arc<AtomicBool>,
}

struct InputState {
    text: String,
    codepoint: String,
    name: String,
    current_field: usize,
}

use crate::config::Config;

impl UnicodeInspectorTool {
    pub fn new(db: Arc<Mutex<Database>>, config: &Config) -> Result<Self, Box<dyn Error>> {
        let is_loading = Arc::new(AtomicBool::new(false));
        let db_clone = Arc::clone(&db);
        let loading_clone = Arc::clone(&is_loading);
        let unicode_data_path = config.unicode_data_path.clone();
        let blocks_path = config.blocks_path.clone();

        tokio::spawn(async move {
            loading_clone.store(true, Ordering::SeqCst);
            let _ = Self::import_data_if_needed(db_clone, unicode_data_path, blocks_path);
            loading_clone.store(false, Ordering::SeqCst);
        });

        Ok(UnicodeInspectorTool {
            input: InputState {
                text: String::new(),
                codepoint: String::new(),
                name: String::new(),
                current_field: 0,
            },
            results: Vec::new(),
            sequential: false,
            db,
            is_loading,
        })
    }

    fn import_data_if_needed(
        db: Arc<Mutex<Database>>,
        unicode_path: String,
        _blocks_path: String,
    ) -> Result<(), Box<dyn Error>> {
        let mut locked_db = db.lock().unwrap();

        // Only import if table is empty
        let count: u32 = locked_db
            .conn()
            .query_row("SELECT COUNT(*) FROM unicode_chars", [], |r| r.get(0))
            .unwrap_or(0);
        if count > 0 {
            return Ok(());
        }

        let tx = locked_db.conn().transaction()?;
        if std::path::Path::new(&unicode_path).exists() {
            let chars_data = fs::read_to_string(unicode_path)?;
            for line in chars_data.lines() {
                let fields: Vec<&str> = line.split(';').collect();
                if fields.len() >= 3 {
                    tx.execute(
                        "INSERT OR REPLACE INTO unicode_chars (codepoint, name, block) VALUES (?1, ?2, ?3)",
                        params![fields[0], fields[1], fields[2]],
                    )?;
                }
            }
        }
        tx.commit()?;
        Ok(())
    }

    fn analyze_text(&mut self) -> Result<String, Box<dyn Error>> {
        self.results.clear();
        let graphemes = self.input.text.graphemes(true).collect::<Vec<&str>>();
        for g in graphemes {
            let codepoints = g
                .chars()
                .map(|c| format!("{:04X}", c as u32))
                .collect::<Vec<String>>();
            if let Some(codepoint) = codepoints.first() {
                let mut db = self.db.lock().unwrap();
                let mut rows = db.conn().prepare(
                    "SELECT codepoint, name, block FROM unicode_chars WHERE codepoint = ?1",
                )?;
                let chars = rows.query_map(params![codepoint], |row| {
                    Ok(UnicodeChar {
                        codepoint: row.get(0)?,
                        name: row.get(1)?,
                        block: row.get(2)?,
                    })
                })?;
                for c in chars {
                    self.results.push(c?);
                }
            }
        }
        Ok(format!("Analyzed {} graphemes", self.results.len()))
    }

    fn lookup_codepoint(&mut self) -> Result<String, Box<dyn Error>> {
        self.results.clear();
        let cp = self.input.codepoint.trim_start_matches("U+").to_uppercase();
        let mut db = self.db.lock().unwrap();
        let mut rows = db
            .conn()
            .prepare("SELECT codepoint, name, block FROM unicode_chars WHERE codepoint = ?1")?;
        let chars = rows.query_map(
            params![format!("{:04X}", u32::from_str_radix(&cp, 16)?)],
            |row| {
                Ok(UnicodeChar {
                    codepoint: row.get(0)?,
                    name: row.get(1)?,
                    block: row.get(2)?,
                })
            },
        )?;
        for c in chars {
            self.results.push(c?);
        }
        Ok(format!("Found {} characters", self.results.len()))
    }

    fn lookup_name(&mut self) -> Result<String, Box<dyn Error>> {
        self.results.clear();
        let mut db = self.db.lock().unwrap();
        let mut rows = db
            .conn()
            .prepare("SELECT codepoint, name, block FROM unicode_chars WHERE name LIKE ?1")?;
        let chars = rows.query_map(params![format!("%{}%", self.input.name)], |row| {
            Ok(UnicodeChar {
                codepoint: row.get(0)?,
                name: row.get(1)?,
                block: row.get(2)?,
            })
        })?;
        for c in chars {
            self.results.push(c?);
        }
        Ok(format!("Found {} characters", self.results.len()))
    }
}

use async_trait::async_trait;

#[async_trait]
impl super::Tool for UnicodeInspectorTool {
    fn name(&self) -> &'static str {
        "Unicode Inspector"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        if self.is_loading.load(Ordering::SeqCst) {
            let area = f.area();
            let loading = Paragraph::new("Loading Unicode Database...")
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
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let text_input = Paragraph::new(self.input.text.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Unicode Input",
                    Style::default().fg(Color::Green),
                ))),
        );
        f.render_widget(text_input, chunks[0]);

        let codepoint_input = Paragraph::new(self.input.codepoint.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Codepoint",
                    Style::default().fg(Color::Green),
                ))),
        );
        f.render_widget(codepoint_input, chunks[1]);

        let name_input = Paragraph::new(self.input.name.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Name",
                    Style::default().fg(Color::Green),
                ))),
        );
        f.render_widget(name_input, chunks[2]);

        let sequential_toggle = Paragraph::new(if self.sequential { "Yes" } else { "No" }).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Sequential Mode",
                    Style::default().fg(Color::Green),
                ))),
        );
        f.render_widget(sequential_toggle, chunks[3]);

        let results =
            Paragraph::new(
                self.results
                    .iter()
                    .map(|c| {
                        Line::from(vec![
                            Span::raw(format!("U+{} ", c.codepoint)),
                            Span::raw(c.name.clone()),
                            Span::raw(format!(" ({})", c.block)),
                        ])
                    })
                    .collect::<Vec<_>>(),
            )
            .block(Block::default().borders(Borders::ALL).title(Line::from(
                Span::styled("Unicode Results", Style::default().fg(Color::Green)),
            )));
        f.render_widget(results, chunks[4]);
    }

    async fn handle_input(&mut self, key: KeyEvent) -> Result<String, Box<dyn Error>> {
        match key.code {
            KeyCode::Up => {
                self.input.current_field = self.input.current_field.saturating_sub(1);
                Ok("Switched field".into())
            }
            KeyCode::Down => {
                self.input.current_field = (self.input.current_field + 1).min(3);
                Ok("Switched field".into())
            }
            KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.sequential = !self.sequential;
                Ok(format!("Sequential Mode: {}", self.sequential))
            }
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if !self.input.codepoint.is_empty() {
                    self.lookup_codepoint()
                } else if !self.input.name.is_empty() {
                    self.lookup_name()
                } else {
                    Ok("No lookup input".into())
                }
            }
            KeyCode::Enter => self.analyze_text(),
            KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                serde_json::to_writer(
                    std::fs::File::create("unicode_results.json")?,
                    &self.results,
                )?;
                Ok("Exported to unicode_results.json".into())
            }
            KeyCode::Char(c) => {
                match self.input.current_field {
                    0 => self.input.text.push(c),
                    1 => self.input.codepoint.push(c),
                    2 => self.input.name.push(c),
                    _ => {}
                }
                Ok("Input updated".into())
            }
            KeyCode::Backspace => match self.input.current_field {
                0 => {
                    self.input.text.pop();
                    Ok("Removed character".into())
                }
                1 => {
                    self.input.codepoint.pop();
                    Ok("Removed character".into())
                }
                2 => {
                    self.input.name.pop();
                    Ok("Removed character".into())
                }
                _ => Ok("No action".into()),
            },
            _ => Ok(String::new()),
        }
    }

    fn save_cache(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
