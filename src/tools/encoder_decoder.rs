use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
use hex;
use urlencoding;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Format {
    Base64,
    Hex,
    Url,
}

impl Format {
    fn name(&self) -> &'static str {
        match self {
            Format::Base64 => "Base64",
            Format::Hex => "Hex",
            Format::Url => "URL",
        }
    }

    fn next(&self) -> Self {
        match self {
            Format::Base64 => Format::Hex,
            Format::Hex => Format::Url,
            Format::Url => Format::Base64,
        }
    }
}

pub struct EncoderDecoderTool {
    input: String,
    format: Format,
    is_encode: bool,
    result: Option<Result<String, String>>,
}

impl Default for EncoderDecoderTool {
    fn default() -> Self {
        Self::new()
    }
}

impl EncoderDecoderTool {
    pub fn new() -> Self {
        EncoderDecoderTool {
            input: String::new(),
            format: Format::Base64,
            is_encode: true,
            result: None,
        }
    }

    fn process(&mut self) {
        if self.input.is_empty() {
            self.result = None;
            return;
        }

        let res = match (self.format, self.is_encode) {
            (Format::Base64, true) => Ok(b64.encode(&self.input)),
            (Format::Base64, false) => b64
                .decode(&self.input)
                .map_err(|e| e.to_string())
                .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string())),
            (Format::Hex, true) => Ok(hex::encode(&self.input)),
            (Format::Hex, false) => hex::decode(&self.input)
                .map_err(|e| e.to_string())
                .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string())),
            (Format::Url, true) => Ok(urlencoding::encode(&self.input).into_owned()),
            (Format::Url, false) => urlencoding::decode(&self.input)
                .map_err(|e| e.to_string())
                .map(|s| s.into_owned()),
        };
        self.result = Some(res);
    }
}

impl super::Tool for EncoderDecoderTool {
    fn name(&self) -> &'static str {
        "Encoder/Decoder"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let config_line = Line::from(vec![
            Span::raw("Mode: "),
            Span::styled(
                if self.is_encode { "Encode" } else { "Decode" },
                Style::default().fg(Color::Yellow).bold(),
            ),
            Span::raw(" | Format: "),
            Span::styled(
                self.format.name(),
                Style::default().fg(Color::Yellow).bold(),
            ),
            Span::raw(" | [Ctrl+M] Toggle Mode | [Ctrl+T] Toggle Format"),
        ]);
        let config_para = Paragraph::new(config_line).block(Block::default().borders(Borders::ALL));
        f.render_widget(config_para, chunks[0]);

        let input_para = Paragraph::new(self.input.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Input",
                    Style::default().fg(Color::Green),
                ))),
        );
        f.render_widget(input_para, chunks[1]);

        let result_content = match &self.result {
            Some(Ok(res)) => res.clone(),
            Some(Err(err)) => format!("Error: {}", err),
            None => String::new(),
        };

        let result_style = match &self.result {
            Some(Err(_)) => Style::default().fg(Color::Red),
            _ => Style::default(),
        };

        let result_para = Paragraph::new(result_content).style(result_style).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled(
                    "Result",
                    Style::default().fg(Color::Cyan),
                ))),
        );
        f.render_widget(result_para, chunks[2]);
    }

    fn handle_input(
        &mut self,
        key: KeyEvent,
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            match key.code {
                KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.is_encode = !self.is_encode;
                    self.process();
                    Ok(format!(
                        "Mode toggled to {}",
                        if self.is_encode { "Encode" } else { "Decode" }
                    ))
                }
                KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.format = self.format.next();
                    self.process();
                    Ok(format!("Format toggled to {}", self.format.name()))
                }
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.input.push(c);
                    self.process();
                    Ok("Input updated".into())
                }
                KeyCode::Backspace => {
                    self.input.pop();
                    self.process();
                    Ok("Removed character".into())
                }
                _ => Ok(String::new()),
            }
        })
    }
}
