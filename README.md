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
