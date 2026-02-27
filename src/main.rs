use std::sync::{Arc, Mutex};
use dev_toolbox::db::Database;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use dev_toolbox::app::App;
use dev_toolbox::secrets::Secrets;
use dev_toolbox::config::Config;
use std::io;
use clap::{Arg, Command};
use secrecy::ExposeSecret;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;
    let matches = Command::new("Dev-Toolbox")
        .version("1.0")
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
        eprintln!("Error: GitHub token missing in .env file");
        return Err("GitHub token missing in .env file".into());
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

