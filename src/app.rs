use crate::audio::AudioPlayer;
use crate::config::Config;
use crate::download;
use crate::prompt;
use crate::ui;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

// GitHub repository information for downloading sounds
const GITHUB_USER: &str = "ebithril";
const GITHUB_REPO: &str = "relax-player";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    Rain,
    Thunder,
    Campfire,
    Master,
}

impl Channel {
    /// Get the name of this channel for display
    pub fn name(&self) -> &str {
        match self {
            Channel::Rain => "Rain",
            Channel::Thunder => "Thunder",
            Channel::Campfire => "Campfire",
            Channel::Master => "Master",
        }
    }

    /// Get all channels in order
    pub fn all() -> [Channel; 4] {
        [
            Channel::Rain,
            Channel::Thunder,
            Channel::Campfire,
            Channel::Master,
        ]
    }

    /// Move to the next channel (right)
    pub fn next(&self) -> Self {
        match self {
            Channel::Rain => Channel::Thunder,
            Channel::Thunder => Channel::Campfire,
            Channel::Campfire => Channel::Master,
            Channel::Master => Channel::Master,
        }
    }

    /// Move to the previous channel (left)
    pub fn prev(&self) -> Self {
        match self {
            Channel::Rain => Channel::Rain,
            Channel::Thunder => Channel::Rain,
            Channel::Campfire => Channel::Thunder,
            Channel::Master => Channel::Campfire,
        }
    }
}

pub struct App {
    pub audio: AudioPlayer,
    pub config: Config,
    pub selected_channel: Channel,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let audio = AudioPlayer::new()?;

        Ok(Self {
            audio,
            config,
            selected_channel: Channel::Rain,
            should_quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.handle_sounds()?;

        // Set initial volumes
        self.update_audio_volumes();

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            // Draw UI
            terminal.draw(|f| ui::render(f, &self))?;

            // Handle events
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_event(key)?;
                        self.update_audio_volumes();
                    }
                }
            }

            // Check if we should quit
            if self.should_quit {
                break;
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            // Quit
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.quit();
            }
            // Navigate left
            KeyCode::Left | KeyCode::Char('h') => {
                self.select_prev();
            }
            // Navigate right
            KeyCode::Right | KeyCode::Char('l') => {
                self.select_next();
            }
            // Volume up
            KeyCode::Up | KeyCode::Char('k') => {
                self.increase_volume()?;
            }
            // Volume down
            KeyCode::Down | KeyCode::Char('j') => {
                self.decrease_volume()?;
            }
            // Toggle mute
            KeyCode::Char('m') | KeyCode::Char('M') => {
                self.toggle_mute()?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Check if sounds need downloading and download
    fn handle_sounds(&mut self) -> Result<()> {
        // In debug mode, check CWD first and skip download if found
        if cfg!(debug_assertions) && download::check_cwd_sounds() {
            println!("Debug mode: Using sounds from ./sounds/");
            return Ok(());
        }

        let current_version = env!("CARGO_PKG_VERSION");
        let sounds_exist = download::sounds_exist()?;
        let stored_version = self.config.sounds_version.as_deref();

        // Check if we need to download sounds
        let should_download = if !sounds_exist {
            // First install - auto download
            println!(
                "Sounds not found. Downloading sounds for v{}...",
                current_version
            );
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
            match download::download_sounds(GITHUB_USER, GITHUB_REPO, current_version) {
                Ok(()) => {
                    // Update config with new version
                    self.config.sounds_version = Some(current_version.to_string());
                    self.config.save()?;
                }
                Err(error) => {
                    if !sounds_exist {
                        panic!("{error:?}")
                    }
                }
            }
        }

        Ok(())
    }

    /// Move selection to the next channel (right)
    fn select_next(&mut self) {
        self.selected_channel = self.selected_channel.next();
    }

    /// Move selection to the previous channel (left)
    fn select_prev(&mut self) {
        self.selected_channel = self.selected_channel.prev();
    }

    fn update_audio_volumes(&self) {
        let rain_vol = self.config.effective_volume(Channel::Rain);
        let thunder_vol = self.config.effective_volume(Channel::Thunder);
        let campfire_vol = self.config.effective_volume(Channel::Campfire);

        self.audio
            .update_volumes(rain_vol, thunder_vol, campfire_vol);
    }

    /// Increase volume of selected channel
    fn increase_volume(&mut self) -> Result<()> {
        match self.selected_channel {
            Channel::Rain => {
                self.config.rain.volume = (self.config.rain.volume + 5).min(100);
            }
            Channel::Thunder => {
                self.config.thunder.volume = (self.config.thunder.volume + 5).min(100);
            }
            Channel::Campfire => {
                self.config.campfire.volume = (self.config.campfire.volume + 5).min(100);
            }
            Channel::Master => {
                self.config.master_volume = (self.config.master_volume + 5).min(100);
            }
        }
        self.config.save()?;
        Ok(())
    }

    /// Decrease volume of selected channel
    fn decrease_volume(&mut self) -> Result<()> {
        match self.selected_channel {
            Channel::Rain => {
                self.config.rain.volume = self.config.rain.volume.saturating_sub(5);
            }
            Channel::Thunder => {
                self.config.thunder.volume = self.config.thunder.volume.saturating_sub(5);
            }
            Channel::Campfire => {
                self.config.campfire.volume = self.config.campfire.volume.saturating_sub(5);
            }
            Channel::Master => {
                self.config.master_volume = self.config.master_volume.saturating_sub(5);
            }
        }
        self.config.save()?;
        Ok(())
    }

    /// Toggle mute for selected channel
    fn toggle_mute(&mut self) -> Result<()> {
        match self.selected_channel {
            Channel::Rain => {
                self.config.rain.muted = !self.config.rain.muted;
            }
            Channel::Thunder => {
                self.config.thunder.muted = !self.config.thunder.muted;
            }
            Channel::Campfire => {
                self.config.campfire.muted = !self.config.campfire.muted;
            }
            Channel::Master => {
                // Master doesn't have mute, ignore
                return Ok(());
            }
        }
        self.config.save()?;
        Ok(())
    }

    /// Get the volume for a channel (0-100)
    pub fn get_volume(&self, channel: Channel) -> u8 {
        match channel {
            Channel::Rain => self.config.rain.volume,
            Channel::Thunder => self.config.thunder.volume,
            Channel::Campfire => self.config.campfire.volume,
            Channel::Master => self.config.master_volume,
        }
    }

    /// Check if a channel is muted
    pub fn is_muted(&self, channel: Channel) -> bool {
        match channel {
            Channel::Rain => self.config.rain.muted,
            Channel::Thunder => self.config.thunder.muted,
            Channel::Campfire => self.config.campfire.muted,
            Channel::Master => false,
        }
    }

    /// Quit the application
    fn quit(&mut self) {
        self.should_quit = true;
    }
}
