use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

pub mod encoder_decoder;
pub mod http_inspector;
pub mod jwt_decoder;
pub mod org_research;
pub mod repo_explorer;
pub mod token_inspector;
pub mod unicode_inspector;

pub use encoder_decoder::EncoderDecoderTool;
pub use http_inspector::HttpRequestInspectorTool;
pub use jwt_decoder::JwtDecoderTool;
pub use org_research::OrgResearchTool;
pub use repo_explorer::RepoExplorerTool;
pub use token_inspector::TokenInspectorTool;
pub use unicode_inspector::UnicodeInspectorTool;

pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn render(&self, f: &mut Frame, area: Rect);
    fn handle_input(
        &mut self,
        key: KeyEvent,
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error>>> + Send + '_>>;

    fn as_persistable(&self) -> Option<&dyn Persistable> {
        None
    }
}

pub trait Persistable {
    fn save_cache(&self) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Clone, Default)]
pub enum LoadState {
    #[default]
    Loading,
    Ready,
    Failed(String),
}
