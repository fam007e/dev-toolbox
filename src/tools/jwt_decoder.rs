use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use std::error::Error;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use crossterm::event::{KeyCode, KeyEvent};

pub struct JwtDecoderTool {
    input: String,
    header: Option<Value>,
    payload: Option<Value>,
}

impl JwtDecoderTool {
    pub fn new() -> Self {
        JwtDecoderTool {
            input: String::new(),
            header: None,
            payload: None,
        }
    }

    fn decode_jwt(&mut self) -> Result<String, Box<dyn Error>> {
        let parts: Vec<&str> = self.input.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid JWT format".into());
        }

        let decode_part = |part: &str| -> Result<Value, Box<dyn Error>> {
            let decoded = general_purpose::STANDARD_NO_PAD.decode(part)?;
            let json = serde_json::from_slice(&decoded)?;
            Ok(json)
        };

        self.header = Some(decode_part(parts[0])?);
        self.payload = Some(decode_part(parts[1])?);
        Ok("Decoded JWT".into())
    }
}

use async_trait::async_trait;

#[async_trait]
impl super::Tool for JwtDecoderTool {
    fn name(&self) -> &'static str {
        "JWT Decoder"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let input = Paragraph::new(self.input.as_str())
            .block(Block::default().borders(Borders::ALL).title(Line::from(Span::styled("JWT Input", Style::default().fg(Color::Green)))));
        f.render_widget(input, chunks[0]);

        let results = Paragraph::new(vec![
            Line::from(format!("Header: {}", self.header.as_ref().map_or("None".to_string(), |v| v.to_string()))),
            Line::from(format!("Payload: {}", self.payload.as_ref().map_or("None".to_string(), |v| v.to_string()))),
        ])
            .block(Block::default().borders(Borders::ALL).title(Line::from(Span::styled("JWT Results", Style::default().fg(Color::Green)))));
        f.render_widget(results, chunks[1]);
    }

    async fn handle_input(&mut self, key: KeyEvent) -> Result<String, Box<dyn Error>> {
        match key.code {
            KeyCode::Char(c) => {
                self.input.push(c);
                Ok("Input updated".into())
            }
            KeyCode::Backspace => {
                self.input.pop();
                Ok("Removed character".into())
            }
            KeyCode::Enter => self.decode_jwt(),
            _ => Ok(String::new())
        }
    }

    fn save_cache(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_jwt_invalid_format() {
        let mut tool = JwtDecoderTool::new();
        tool.input = "invalid.jwt".to_string();
        assert!(tool.decode_jwt().is_err());
    }

    #[test]
    fn test_decode_jwt_valid() {
        // Base64 for {"alg":"HS256"} and {"sub":"1234567890","name":"John Doe","iat":1516239022}
        let header = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let payload = "eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ";
        let jwt = format!("{}.{}.signature", header, payload);
        
        let mut tool = JwtDecoderTool::new();
        tool.input = jwt;
        assert!(tool.decode_jwt().is_ok());
        assert_eq!(tool.header.unwrap()["alg"], "HS256");
        assert_eq!(tool.payload.unwrap()["name"], "John Doe");
    }
}
