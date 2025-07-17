use crate::db::Database;
use crate::secrets::Secrets;
use crate::tools::{Tool, OrgResearchTool, RepoExplorerTool, UnicodeInspectorTool, JwtDecoderTool};
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use arboard::Clipboard;
use std::sync::{Arc, Mutex};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Tabs, Paragraph},
};
use std::io;
use reqwest::Client;

pub struct App {
    tab_index: usize,
    tools: Vec<Box<dyn Tool>>,
    message: String,
    #[allow(dead_code)]
    db: Arc<Mutex<Database>>,
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    secrets: Secrets,
}

impl App {
    pub fn new(db: Arc<Mutex<Database>>, secrets: Secrets) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .user_agent("Dev-Toolbox/1.0")
            .https_only(true)
            .build()?;
        let mut tools: Vec<Box<dyn Tool>> = Vec::new();

        tools.push(Box::new(OrgResearchTool::new(Arc::clone(&db), &client, &secrets)?));

        tools.push(Box::new(RepoExplorerTool::new(Arc::clone(&db), &client, &secrets)?));

        tools.push(Box::new(UnicodeInspectorTool::new(Arc::clone(&db))?));

        tools.push(Box::new(JwtDecoderTool::new()));

        Ok(App {
            tab_index: 0,
            tools,
            message: String::from("Welcome to Dev-Toolbox! Press 'q' to quit, 'Tab' to switch tools."),
            db,
            client,
            secrets,
        })
    }

    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
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
            })?;
            let event = crossterm::event::read()?;
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break,
                    KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                        let mut clipboard = Clipboard::new().unwrap();
                        clipboard.set_text(self.message.clone()).unwrap();
                        self.message = "Copied to clipboard!".to_string();
                    },
                    KeyCode::Tab => self.tab_index = (self.tab_index + 1) % self.tools.len(),
                    _ => {
                        self.message = self.tools[self.tab_index].handle_input(key).unwrap_or_default();
                    }
                },
                Event::Mouse(mouse) => {
                    if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                        if mouse.row == 1 {
                            let tab_width = terminal.size()?.width / self.tools.len() as u16;
                            self.tab_index = (mouse.column / tab_width) as usize;
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn save_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        for tool in &self.tools {
            tool.save_cache()?;
        }
        Ok(())
    }
}
