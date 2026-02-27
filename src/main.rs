use clap::{Arg, Command};
use dev_toolbox::app::App;
use dev_toolbox::config::Config;
use dev_toolbox::db::Database;
use dev_toolbox::secrets::Secrets;
use ratatui::{backend::CrosstermBackend, Terminal};
use secrecy::ExposeSecret;
use std::error::Error;
use std::io;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;
    let matches = Command::new("Dev-Toolbox")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A modular CLI toolbox for GitHub and Unicode analysis")
        .arg(
            Arg::new("env")
                .long("env")
                .value_name("ENV_FILE")
                .help("Path to .env file")
                .default_value(".env"),
        )
        .get_matches();

    let env_path = matches.get_one::<String>("env").unwrap();
    let secrets = Secrets::load(env_path)?;
    if secrets.github_token.expose_secret().is_empty() {
        let config_dir = dirs::config_dir()
            .map(|p| p.join("dev-toolbox").join(".env"))
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "~/.config/dev-toolbox/.env".to_string());

        eprintln!("Error: GITHUB_TOKEN not found.");
        eprintln!("\nTo use this tool, please:");
        eprintln!("1. Set the GITHUB_TOKEN environment variable.");
        eprintln!("2. OR create a .env file in the current directory.");
        eprintln!("3. OR create a .env file at: {}", config_dir);
        eprintln!("\nYou can generate a token at: https://github.com/settings/tokens");
        return Err("GitHub token missing".into());
    }

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let db = Arc::new(Mutex::new(Database::new(&config.cache_db_path)?));

    let mut app = App::new(db, secrets, config)?;

    app.run(&mut terminal).await?;

    app.save_cache()?;

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
