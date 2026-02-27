use async_trait::async_trait;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*};
use std::error::Error;

pub mod org_research;
pub mod repo_explorer;
pub mod unicode_inspector;
pub mod jwt_decoder;

pub use org_research::OrgResearchTool;
pub use repo_explorer::RepoExplorerTool;
pub use unicode_inspector::UnicodeInspectorTool;
pub use jwt_decoder::JwtDecoderTool;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn render(&self, f: &mut Frame, area: Rect);
    async fn handle_input(&mut self, key: KeyEvent) -> Result<String, Box<dyn Error>>;
    fn save_cache(&self) -> Result<(), Box<dyn Error>>;
}
