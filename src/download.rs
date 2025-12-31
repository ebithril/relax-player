use anyhow::{Context, Result};
use directories::ProjectDirs;
use flate2::read::GzDecoder;
use ratatui::DefaultTerminal;
use std::fs;
use std::path::{Path, PathBuf};
use tar::Archive;

use crate::config::Config;
use crate::prompt::{run_prompt, PromptType};

const REQUIRED_SOUNDS: &[&str] = &["rain.mp3", "thunder.mp3", "campfire.mp3"];

/// Check if all required sound files exist in the CWD's sounds/ directory
pub fn check_cwd_sounds() -> bool {
    let cwd_sounds = Path::new("sounds");

    if !cwd_sounds.exists() || !cwd_sounds.is_dir() {
        return false;
    }

    for sound in REQUIRED_SOUNDS {
        let sound_path = cwd_sounds.join(sound);
        if !sound_path.exists() {
            return false;
        }
    }

    true
}

/// Get the sounds directory path
/// In debug mode: Check CWD first, fall back to data directory
/// In release mode: Use data directory only
pub fn get_sounds_dir() -> Result<PathBuf> {
    if cfg!(debug_assertions) && check_cwd_sounds() {
        Ok(PathBuf::from("sounds"))
    } else {
        Config::sounds_dir()
    }
}

/// Check if all required sound files exist in the sounds directory
/// In debug mode: Checks CWD first, then data directory
/// In release mode: Only checks data directory
pub fn sounds_exist() -> Result<bool> {
    let sounds_dir = get_sounds_dir()?;

    for sound in REQUIRED_SOUNDS {
        let sound_path = sounds_dir.join(sound);
        if !sound_path.exists() {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Download and extract sounds from GitHub release
pub fn download_sounds(
    terminal: &mut DefaultTerminal,
    github_user: &str,
    github_repo: &str,
    version: &str,
) -> Result<()> {
    let url = format!(
        "https://github.com/{}/{}/releases/download/v{}/sounds.tar.gz",
        github_user, github_repo, version
    );

    run_prompt(
        terminal,
        "Downloading",
        &format!("Downloading sounds from GitHub release v{}...", version),
        PromptType::Info,
    )?;

    // Download the file
    let response = reqwest::blocking::get(&url).context("Failed to download sounds")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download sounds: HTTP status {}. Make sure the release exists with sounds.tar.gz attached.",
            response.status()
        );
    }

    let bytes = response
        .bytes()
        .context("Failed to read download response")?;

    run_prompt(
        terminal,
        "Download Complete",
        &format!("Downloaded {} KB", bytes.len() / 1024),
        PromptType::Info,
    )?;

    // Extract to data directory (archive contains sounds/ folder)
    let proj_dirs = ProjectDirs::from("com", "relax-player", "relax-player")
        .context("Failed to determine data directory")?;
    let data_dir = proj_dirs.data_dir();

    fs::create_dir_all(data_dir).context("Failed to create data directory")?;

    let decoder = GzDecoder::new(&bytes[..]);
    let mut archive = Archive::new(decoder);

    run_prompt(
        terminal,
        "Extracting",
        "Extracting sound files...",
        PromptType::Info,
    )?;

    archive
        .unpack(data_dir)
        .context("Failed to extract sounds archive")?;

    // Verify all sounds were extracted
    if !sounds_exist()? {
        anyhow::bail!(
            "Sound extraction completed but some files are missing. Expected: {:?}",
            REQUIRED_SOUNDS
        );
    }

    run_prompt(
        terminal,
        "Success",
        "Sounds downloaded and extracted successfully!",
        PromptType::Info,
    )?;

    Ok(())
}

/// Check if version needs update (returns true if update needed)
pub fn needs_update(current_version: &str, stored_version: Option<&str>) -> bool {
    match stored_version {
        None => true, // No version stored means first install or needs update
        Some(stored) => stored != current_version,
    }
}
