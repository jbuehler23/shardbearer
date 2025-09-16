# NEON VEDEY - Neural Shadows

A cyberpunk text-based adventure game built with Bevy.

## Story

New Carthage never sleeps. You are a runner carrying stolen data—fragments of the Vedey Protocol, Helix Corporation's most classified neural mapping project. Navigate the rain-slick streets, make choices that matter, and discover multiple endings in this branching narrative adventure.

## Features

- **Retro Terminal Interface**: Experience the game through a classic terminal aesthetic with cyan and magenta cyberpunk colors
- **Branching Narrative**: Your choices shape the story with 7 unique endings to discover
- **Command System**: Type commands or use number keys for quick choices
- **ASCII Art**: Atmospheric visuals enhance key story moments

## Controls

- **Number Keys (1-4)**: Quick select choices
- **Type Commands**:
  - `help` - Show available commands
  - `back` - Return to previous scene
  - `history` - View visited locations
  - `endings` - Show unlocked endings
  - `quit` - Exit the game
- **ESC**: Pause menu

## Running the Game

### Prerequisites

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

### Build and Run

```bash
# Clone the repository
git clone <repository-url>
cd text-adventure

# Run in development mode
cargo run

# Or build for release
cargo build --release
./target/release/text-adventure
```

### Web Build

```bash
# Install wasm target
rustup target add wasm32-unknown-unknown

# Build for web
cargo build --target wasm32-unknown-unknown --release
```

## System Requirements

- **Linux**: May require Wayland development libraries (`libwayland-dev` on Ubuntu/Debian)
- **Windows**: Should work out of the box
- **macOS**: Should work out of the box

## Troubleshooting

If you encounter Wayland-related build errors on Linux:
```bash
# Ubuntu/Debian
sudo apt-get install libwayland-dev

# Fedora
sudo dnf install wayland-devel

# Arch
sudo pacman -S wayland
```

## License

This project uses the Bevy game engine.