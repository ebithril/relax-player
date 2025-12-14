use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::download;

pub struct AudioPlayer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    rain_sink: Sink,
    thunder_sink: Sink,
    campfire_sink: Sink,
}

impl AudioPlayer {
    /// Create a new audio player and load all sound files
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()
            .context("Failed to create audio output stream")?;

        let rain_sink = Sink::try_new(&stream_handle)
            .context("Failed to create rain sink")?;
        let thunder_sink = Sink::try_new(&stream_handle)
            .context("Failed to create thunder sink")?;
        let campfire_sink = Sink::try_new(&stream_handle)
            .context("Failed to create campfire sink")?;

        let player = Self {
            _stream: stream,
            _stream_handle: stream_handle,
            rain_sink,
            thunder_sink,
            campfire_sink,
        };

        // Load and start playing all sounds
        let sounds_dir = download::get_sounds_dir()?;
        player.load_sound(&sounds_dir.join("rain.mp3"), &player.rain_sink)?;
        player.load_sound(&sounds_dir.join("thunder.mp3"), &player.thunder_sink)?;
        player.load_sound(&sounds_dir.join("campfire.mp3"), &player.campfire_sink)?;

        // Start all sinks (they'll play at configured volumes)
        player.rain_sink.play();
        player.thunder_sink.play();
        player.campfire_sink.play();

        Ok(player)
    }

    /// Load a sound file into a sink
    fn load_sound(&self, path: &PathBuf, sink: &Sink) -> Result<()> {
        if !path.exists() {
            anyhow::bail!(
                "Sound file not found: {}. Please run the app to download sounds, or check that sounds are properly installed.",
                path.display()
            );
        }

        let file = File::open(path)
            .context(format!("Failed to open sound file: {}", path.display()))?;
        let source = Decoder::new(BufReader::new(file))
            .context(format!("Failed to decode sound file: {}", path.display()))?;

        sink.append(source.repeat_infinite());
        Ok(())
    }

    /// Update rain volume (0.0 to 1.0)
    pub fn set_rain_volume(&self, volume: f32) {
        self.rain_sink.set_volume(volume);
    }

    /// Update thunder volume (0.0 to 1.0)
    pub fn set_thunder_volume(&self, volume: f32) {
        self.thunder_sink.set_volume(volume);
    }

    /// Update campfire volume (0.0 to 1.0)
    pub fn set_campfire_volume(&self, volume: f32) {
        self.campfire_sink.set_volume(volume);
    }

    /// Update all volumes from config
    pub fn update_volumes(&self, rain_vol: f32, thunder_vol: f32, campfire_vol: f32) {
        self.set_rain_volume(rain_vol);
        self.set_thunder_volume(thunder_vol);
        self.set_campfire_volume(campfire_vol);
    }
}
