use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use reqwest::{Client, Method};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Copy, PartialEq)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    fn next(&self) -> Self {
        match self {
            HttpMethod::Get => HttpMethod::Post,
            HttpMethod::Post => HttpMethod::Put,
            HttpMethod::Put => HttpMethod::Delete,
            HttpMethod::Delete => HttpMethod::Get,
        }
    }
    
    fn as_reqwest_method(&self) -> Method {
        match self {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Delete => Method::DELETE,
        }
    }
    
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
    }
}

pub struct HttpRequestInspectorTool {
    client: Client,
    url: String,
    method: HttpMethod,
    loading: bool,
    response: Option<String>,
    status: Option<u16>,
}

impl HttpRequestInspectorTool {
    pub fn new(client: &Client) -> Self {
        HttpRequestInspectorTool {
            client: client.clone(),
            url: "https://".to_string(),
            method: HttpMethod::Get,
            loading: false,
            response: None,
            status: None,
        }
    }

    async fn send_request(&mut self) -> Result<String, Box<dyn Error>> {
        if self.url.is_empty() {
            return Ok("URL cannot be empty".into());
        }

        self.loading = true;
        
        let request = self.client.request(self.method.as_reqwest_method(), &self.url);
        let resp = request.send().await;

        self.loading = false;
        
        match resp {
            Ok(r) => {
                self.status = Some(r.status().as_u16());
                let headers = r.headers().iter()
                    .map(|(k, v)| format!("{}: {}", k.as_str(), v.to_str().unwrap_or("[binary]")))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let mut body = r.text().await.unwrap_or_else(|_| "[Unreadable Body]".into());
                if body.chars().count() > 10_000 {
                    body = body.chars().take(10_000).collect::<String>();
                    body.push_str("\n...[truncated]");
                }
                self.response = Some(format!("Headers:\n{}\n\nBody:\n{}", headers, body));
                Ok("Request completed".into())
            }
            Err(e) => {
                self.status = None;
                self.response = Some(format!("Error: {}", e));
                Ok("Request failed".into())
            }
        }
    }
}

impl super::Tool for HttpRequestInspectorTool {
    fn name(&self) -> &'static str {
        "HTTP Inspector"
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        if self.loading {
            let loading = Paragraph::new("Sending Request...")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let input_line = Line::from(vec![
            Span::raw("Method [Ctrl+M]: "),
            Span::styled(self.method.as_str(), Style::default().fg(Color::Yellow).bold()),
            Span::raw(" | URL: "),
            Span::styled(&self.url, Style::default().fg(Color::White)),
        ]);
        
        let input_para = Paragraph::new(input_line).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled("Request", Style::default().fg(Color::Green)))),
        );
        f.render_widget(input_para, chunks[0]);

        let mut res_lines = vec![];
        if let Some(status) = self.status {
            let color = if status >= 200 && status < 300 { Color::Green } else { Color::Red };
            res_lines.push(Line::from(vec![
                Span::styled("Status: ", Style::default().bold()),
                Span::styled(status.to_string(), Style::default().fg(color).bold()),
            ]));
            res_lines.push(Line::from(""));
        }
        
        if let Some(resp) = &self.response {
            for line in resp.lines() {
                res_lines.push(Line::from(line.to_string()));
            }
        } else {
            res_lines.push(Line::from("No request sent yet."));
        }

        let result_para = Paragraph::new(res_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(Span::styled("Response", Style::default().fg(Color::Cyan)))),
        );
        f.render_widget(result_para, chunks[1]);
    }

    fn handle_input(
        &mut self,
        key: KeyEvent,
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            match key.code {
                KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.method = self.method.next();
                    Ok(format!("Method toggled to {}", self.method.as_str()))
                }
                KeyCode::Enter => self.send_request().await,
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.url.push(c);
                    Ok("URL updated".into())
                }
                KeyCode::Backspace => {
                    self.url.pop();
                    Ok("Removed character".into())
                }
                _ => Ok(String::new()),
            }
        })
    }
}
