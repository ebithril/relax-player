use crate::config::Config;
use anyhow::Result;

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
    pub config: Config,
    pub selected_channel: Channel,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        Ok(Self {
            config,
            selected_channel: Channel::Rain,
            should_quit: false,
        })
    }

    /// Move selection to the next channel (right)
    pub fn select_next(&mut self) {
        self.selected_channel = self.selected_channel.next();
    }

    /// Move selection to the previous channel (left)
    pub fn select_prev(&mut self) {
        self.selected_channel = self.selected_channel.prev();
    }

    /// Increase volume of selected channel
    pub fn increase_volume(&mut self) -> Result<()> {
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
    pub fn decrease_volume(&mut self) -> Result<()> {
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
    pub fn toggle_mute(&mut self) -> Result<()> {
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
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
