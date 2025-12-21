use crate::app::Channel;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundConfig {
    pub volume: u8, // 0-100
    pub muted: bool,
}

impl Default for SoundConfig {
    fn default() -> Self {
        Self {
            volume: 70,
            muted: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rain: SoundConfig,
    pub thunder: SoundConfig,
    pub campfire: SoundConfig,
    pub master_volume: u8, // 0-100
    #[serde(default)]
    pub sounds_version: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rain: SoundConfig::default(),
            thunder: SoundConfig::default(),
            campfire: SoundConfig::default(),
            master_volume: 70,
            sounds_version: None,
        }
    }
}

impl Config {
    /// Get the config file path (cross-platform)
    fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "relax-player", "relax-player")
            .context("Failed to determine config directory")?;

        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir).context("Failed to create config directory")?;

        Ok(config_dir.join("config.json"))
    }

    /// Load config from file, or create default if it doesn't exist
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if path.exists() {
            let contents = fs::read_to_string(&path).context("Failed to read config file")?;
            let config: Config =
                serde_json::from_str(&contents).context("Failed to parse config file")?;
            Ok(config)
        } else {
            // Create default config and save it
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let contents = serde_json::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(&path, contents).context("Failed to write config file")?;
        Ok(())
    }

    /// Get the effective volume for a sound (individual * master / 100)
    pub fn effective_volume(&self, channel: Channel) -> f32 {
        let sound_config = match channel {
            Channel::Rain => &self.rain,
            Channel::Thunder => &self.thunder,
            Channel::Campfire => &self.campfire,
            Channel::Master => panic!("Not allowed to use master"),
        };

        if sound_config.muted {
            0.0
        } else {
            (sound_config.volume as f32 / 100.0) * (self.master_volume as f32 / 100.0)
        }
    }

    /// Get the sounds directory path (cross-platform)
    pub fn sounds_dir() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "relax-player", "relax-player")
            .context("Failed to determine data directory")?;

        let data_dir = proj_dirs.data_dir();
        let sounds_dir = data_dir.join("sounds");

        fs::create_dir_all(&sounds_dir).context("Failed to create sounds directory")?;

        Ok(sounds_dir)
    }
}
