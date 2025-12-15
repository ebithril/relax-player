# Relax Player

A terminal-based sound player for creating relaxing ambient soundscapes. Features individual volume control for Rain, Thunder, and Campfire sounds with an alsamixer-style TUI interface.

![relax-player](images/relax-player.gif)

## Features

- Play multiple looping ambient sounds simultaneously
- Individual volume control for each sound (0-100%)
- Master volume control
- Mute/unmute individual sounds
- Alsamixer-style vertical bar UI
- Persistent configuration (volumes and mute states saved automatically)
- Cross-platform (Linux, Windows, macOS)
- Vim-style keybindings

## Installation

### From crates.io (recommended)

```bash
cargo install relax-player
```

On first run, the application will automatically download the sound files for you.

### From source

```bash
cargo build --release
./target/release/relax-player
```

**For development** (debug builds):
```bash
# Add sound files to ./sounds/ directory first
mkdir -p sounds
# Copy rain.mp3, thunder.mp3, campfire.mp3 into sounds/

# Then run in debug mode
cargo run
```

Debug builds check `./sounds/` in the current directory first, making local development easier. If sounds aren't found in CWD, they fall back to downloading from GitHub like release builds.

**Release builds**: Sound files are automatically downloaded when you first run the application. They are stored in a platform-specific data directory and will be reused between sessions.

## Sound Management

### Automatic Downloads

- **First install**: Sounds are automatically downloaded on first run
- **Version updates**: When you update to a new version, you'll be prompted to download updated sounds
- **Storage location**:
  - **Linux**: `~/.local/share/relax-player/sounds/`
  - **Windows**: `%APPDATA%\relax-player\sounds\`
  - **macOS**: `~/Library/Application Support/relax-player/sounds/`

The application downloads three sound files:
- `rain.mp3` - Rain ambient sound
- `thunder.mp3` - Thunder ambient sound
- `campfire.mp3` - Campfire crackling sound

## Controls

### Navigation
- `←` / `→` or `h` / `l` - Select previous/next channel
- Channels: Rain → Thunder → Campfire → Master

### Volume Control
- `↑` / `↓` or `k` / `j` - Increase/decrease volume (±5%)
- Volume range: 0-100%

### Mute
- `m` - Toggle mute for selected sound (not available for Master)

### Other
- `q` - Quit application

## UI Layout

```
┌─────── Relax Player ───────────────────────┐
│  Rain     Thunder   Campfire    Master     │
│   ┃         ┃         ┃           ┃        │
│   ┃         ┃         ▓▓▓         ┃        │
│   ┃         ┃         ▓▓▓         ▓▓▓      │
│   ▓▓▓       ┃         ▓▓▓         ▓▓▓      │
│   ▓▓▓       ░░░       ▓▓▓         ▓▓▓      │
│   ▓▓▓       ░░░       ▓▓▓         ▓▓▓      │
│   ▓▓▓       ░░░       ▓▓▓         ▓▓▓      │
│  [80%]     [30%]🔇   [100%]       [70%]     │
│    ^                                        │
├────────────────────────────────────────────┤
│ ←/→ h/l: Select  ↑/↓ j/k: Volume  m: Mute │
│ q: Quit                                    │
└────────────────────────────────────────────┘
```

- `▓▓▓` - Active volume bars (green for selected, cyan for others)
- `░░░` - Muted sound
- `┃` - Empty bar space
- `^` - Selected channel indicator
- `🔇` - Mute indicator

## Configuration

Settings are automatically saved to a configuration file when changed:

- **Linux**: `~/.config/relax-player/config.json`
- **Windows**: `%APPDATA%\relax-player\config.json`
- **macOS**: `~/Library/Application Support/relax-player/config.json`

The config file stores:
- Individual volume levels for each sound
- Mute states
- Master volume
- Downloaded sounds version (for update tracking)

Settings persist between sessions, so your preferred volumes are restored when you restart the application.

## How It Works

- **Volume Calculation**: Each sound's effective volume = (individual volume × master volume) / 100
- **Looping**: All sounds loop continuously when not muted
- **Master Volume**: Affects all sounds proportionally without changing their individual settings

## Dependencies

- [rodio](https://github.com/RustAudio/rodio) - Audio playback
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal control
- [serde](https://serde.rs/) - Configuration serialization
- [directories](https://github.com/dirs-dev/directories-rs) - Cross-platform config paths
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP downloads
- [tar](https://github.com/alexcrichton/tar-rs) - Archive extraction
- [flate2](https://github.com/rust-lang/flate2-rs) - Gzip compression

## For Maintainers

### Automated Release Process

This project uses GitHub Actions to automate the release process. Everything is handled automatically - just push a tag!

## License

This project uses the MIT license

