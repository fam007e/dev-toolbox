use crate::db::Database;
use crate::secrets::Secrets;
use crate::tools::{
    EncoderDecoderTool, HttpRequestInspectorTool, JwtDecoderTool, OrgResearchTool,
    RepoExplorerTool, TokenInspectorTool, Tool, UnicodeInspectorTool,
};
use arboard::Clipboard;
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
};
use reqwest::Client;
use std::io;
use std::sync::{Arc, Mutex};

use crate::config::Config;

pub struct App {
    tab_index: usize,
    tools: Vec<Box<dyn Tool>>,
    message: String,
    search_mode: bool,
    search_query: String,
    search_results: Vec<usize>,
    search_selected: usize,
    #[allow(dead_code)]
    db: Arc<Mutex<Database>>,
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    secrets: Secrets,
    #[allow(dead_code)]
    config: Config,
}

impl App {
    pub fn new(
        db: Arc<Mutex<Database>>,
        secrets: Secrets,
        config: Config,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .user_agent("Dev-Toolbox/1.0")
            .https_only(true)
            .build()?;

        let tools: Vec<Box<dyn Tool>> = vec![
            Box::new(OrgResearchTool::new(&client, &secrets)?),
            Box::new(RepoExplorerTool::new(Arc::clone(&db), &client, &secrets)?),
            Box::new(UnicodeInspectorTool::new(Arc::clone(&db), &config)?),
            Box::new(TokenInspectorTool::new(&client, &secrets)),
            Box::new(EncoderDecoderTool::new()),
            Box::new(HttpRequestInspectorTool::new(&client)),
            Box::new(JwtDecoderTool::new()),
        ];

        Ok(App {
            tab_index: 0,
            tools,
            message: String::from("Welcome to Dev-Toolbox! Use shortcuts below to navigate."),
            search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: 0,
            db,
            client,
            secrets,
            config,
        })
    }

    fn update_search(&mut self) {
        let query = self.search_query.to_lowercase();
        self.search_results = self
            .tools
            .iter()
            .enumerate()
            .filter(|(_, t)| t.name().to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect();
        self.search_selected = 0;
    }

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Tabs
                        Constraint::Min(0),    // Tool Content
                        Constraint::Length(3), // Status
                        Constraint::Length(3), // Hints
                    ])
                    .split(f.area());

                let tab_titles = self.tools.iter()
                    .map(|t| Line::from(Span::styled(t.name(), Style::default().fg(Color::Cyan))))
                    .collect::<Vec<Line>>();
                let tabs = Tabs::new(tab_titles)
                    .block(Block::default().borders(Borders::ALL).title(Span::styled("Dev-Toolbox", Style::default().fg(Color::Green))))
                    .select(self.tab_index)
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Yellow).bold());
                f.render_widget(tabs, chunks[0]);

                self.tools[self.tab_index].render(f, chunks[1]);

                let message = Paragraph::new(self.message.as_str())
                    .block(Block::default().borders(Borders::ALL).title(Span::styled("Status", Style::default().fg(Color::Magenta))))
                    .style(Style::default().fg(Color::White));
                f.render_widget(message, chunks[2]);

                let hints = Paragraph::new("Ctrl+F: Search | Ctrl+Q: Quit | Tab: Next Tool | Ctrl+C: Copy Status | Enter: Run/Action | Up/Down: Switch Fields | Ctrl+E: Export")
                    .block(Block::default().borders(Borders::ALL).title(Span::styled("Hints", Style::default().fg(Color::Yellow))))
                    .style(Style::default().fg(Color::Gray));
                f.render_widget(hints, chunks[3]);

                if self.search_mode {
                    let area = f.area();
                    let popup_width = 50;
                    let popup_height = 10;
                    let popup_area = Rect::new(
                        area.width.saturating_sub(popup_width) / 2,
                        area.height.saturating_sub(popup_height) / 2,
                        popup_width.min(area.width),
                        popup_height.min(area.height),
                    );
                    f.render_widget(Clear, popup_area);

                    let search_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(3), Constraint::Min(0)])
                        .split(popup_area);

                    let search_input = Paragraph::new(self.search_query.as_str())
                        .block(Block::default().borders(Borders::ALL).title(Span::styled("Global Search (Esc: Cancel, Enter: Select)", Style::default().fg(Color::Green))));
                    f.render_widget(search_input, search_chunks[0]);

                    let mut items = vec![];
                    for (i, &idx) in self.search_results.iter().enumerate() {
                        let style = if i == self.search_selected {
                            Style::default().bg(Color::Blue).fg(Color::White).bold()
                        } else {
                            Style::default()
                        };
                        items.push(Line::from(Span::styled(self.tools[idx].name(), style)));
                    }
                    if items.is_empty() {
                        items.push(Line::from("No tools found."));
                    }
                    let results_list = Paragraph::new(items).block(Block::default().borders(Borders::ALL));
                    f.render_widget(results_list, search_chunks[1]);
                }
            })?;
            let event = crossterm::event::read()?;
            match event {
                Event::Key(key) => {
                    if self.search_mode {
                        match key.code {
                            KeyCode::Esc => {
                                self.search_mode = false;
                            }
                            KeyCode::Enter => {
                                if !self.search_results.is_empty() {
                                    self.tab_index = self.search_results[self.search_selected];
                                }
                                self.search_mode = false;
                            }
                            KeyCode::Up => {
                                self.search_selected = self.search_selected.saturating_sub(1);
                            }
                            KeyCode::Down if !self.search_results.is_empty() => {
                                self.search_selected = (self.search_selected + 1)
                                    .min(self.search_results.len().saturating_sub(1));
                            }
                            KeyCode::Char(c)
                                if !key
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
                            {
                                self.search_query.push(c);
                                self.update_search();
                            }
                            KeyCode::Backspace => {
                                self.search_query.pop();
                                self.update_search();
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('f')
                                if key
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
                            {
                                self.search_mode = true;
                                self.search_query.clear();
                                self.update_search();
                            }
                            KeyCode::Char('q')
                                if key
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
                            {
                                break
                            }
                            KeyCode::Char('c')
                                if key
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
                            {
                                if let Ok(mut clipboard) = Clipboard::new() {
                                    let _ = clipboard.set_text(self.message.clone());
                                    self.message =
                                        "Copied to clipboard! (Cleared in 30s)".to_string();

                                    tokio::spawn(async move {
                                        tokio::time::sleep(tokio::time::Duration::from_secs(30))
                                            .await;
                                        if let Ok(mut cb) = Clipboard::new() {
                                            let _ = cb.set_text("".to_string());
                                        }
                                    });
                                }
                            }
                            KeyCode::Tab => {
                                self.tab_index = (self.tab_index + 1) % self.tools.len()
                            }
                            _ => {
                                self.message = self.tools[self.tab_index]
                                    .handle_input(key)
                                    .await
                                    .unwrap_or_else(|e| e.to_string());
                            }
                        }
                    }
                }
                Event::Mouse(mouse)
                    if mouse.kind == MouseEventKind::Down(MouseButton::Left) && mouse.row == 1 =>
                {
                    let tab_width = terminal.size()?.width / self.tools.len() as u16;
                    self.tab_index = (mouse.column / tab_width) as usize;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn save_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        for tool in &self.tools {
            if let Some(persistable) = tool.as_persistable() {
                persistable.save_cache()?;
            }
        }
        Ok(())
    }
}
