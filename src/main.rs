mod app;
mod audio;
mod config;
mod download;
mod prompt;
mod ui;

use anyhow::Result;
use app::App;
use audio::AudioPlayer;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

// GitHub repository information for downloading sounds
// TODO: Update these with your actual GitHub username and repository name
const GITHUB_USER: &str = "ebithril";
const GITHUB_REPO: &str = "relax-player";

fn main() -> Result<()> {
    // Initialize app
    let mut app = App::new()?;

    // Check and download sounds if needed
    handle_sounds(&mut app)?;

    // Initialize audio player
    let audio = AudioPlayer::new()?;

    // Set initial volumes
    update_audio_volumes(&audio, &app);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let result = run_app(&mut terminal, &mut app, &audio);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Print any errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

fn handle_sounds(app: &mut App) -> Result<()> {
    // In debug mode, check CWD first and skip download if found
    if cfg!(debug_assertions) && download::check_cwd_sounds() {
        println!("Debug mode: Using sounds from ./sounds/");
        return Ok(());
    }

    let current_version = env!("CARGO_PKG_VERSION");
    let sounds_exist = download::sounds_exist()?;
    let stored_version = app.config.sounds_version.as_deref();

    // Check if we need to download sounds
    let should_download = if !sounds_exist {
        // First install - auto download
        println!("Sounds not found. Downloading sounds for v{}...", current_version);
        true
    } else if download::needs_update(current_version, stored_version) {
        // Version mismatch - prompt user
        let message = format!(
            "New sounds available for v{} (current: {}). Download now?",
            current_version,
            stored_version.unwrap_or("unknown")
        );
        prompt::prompt_yes_no(&message)?
    } else {
        // All good, sounds exist and version matches
        false
    };

    if should_download {
        download::download_sounds(GITHUB_USER, GITHUB_REPO, current_version)?;

        // Update config with new version
        app.config.sounds_version = Some(current_version.to_string());
        app.config.save()?;
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    audio: &AudioPlayer,
) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| ui::render(f, app))?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key_event(app, key)?;
                    update_audio_volumes(audio, app);
                }
            }
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
        }
        // Navigate left
        KeyCode::Left | KeyCode::Char('h') => {
            app.select_prev();
        }
        // Navigate right
        KeyCode::Right | KeyCode::Char('l') => {
            app.select_next();
        }
        // Volume up
        KeyCode::Up | KeyCode::Char('k') => {
            app.increase_volume()?;
        }
        // Volume down
        KeyCode::Down | KeyCode::Char('j') => {
            app.decrease_volume()?;
        }
        // Toggle mute
        KeyCode::Char('m') | KeyCode::Char('M') => {
            app.toggle_mute()?;
        }
        _ => {}
    }

    Ok(())
}

fn update_audio_volumes(audio: &AudioPlayer, app: &App) {
    let rain_vol = app.config.effective_volume(&app.config.rain);
    let thunder_vol = app.config.effective_volume(&app.config.thunder);
    let campfire_vol = app.config.effective_volume(&app.config.campfire);

    audio.update_volumes(rain_vol, thunder_vol, campfire_vol);
}
