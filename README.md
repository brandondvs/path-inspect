# path-inspect

A terminal user interface (TUI) application for browsing and inspecting directories in your system's PATH environment variable.

## Features

- **Interactive PATH browsing**: Navigate through all directories in your PATH with arrow keys
- **Directory inspection**: View contents of any PATH directory with file type detection
- **Symlink resolution**: Automatically detects and shows symlink targets
- **Real-time search**: Filter directory contents with `/` search functionality
- **Error handling**: Gracefully handles unreadable directories and broken symlinks
- **Keyboard navigation**: Intuitive controls for terminal-based workflow

## Installation

### Prerequisites

- Rust 1.70+ (with Cargo)

### Build from source

```bash
git clone https://github.com/brandondvs/path-inspect
cd path-inspect
cargo build --release
```

The binary will be available at `target/release/path-inspect`.

## Usage

Run the application:

```bash
cargo run
```

Or if installed:

```bash
path-inspect
```

### Controls

**PATH List View:**
- `↑/↓` - Navigate between PATH directories
- `Enter` - Inspect selected directory
- `q` - Quit application

**Directory Contents View:**
- `↑/↓` - Navigate between files
- `/` - Start search mode
- `Esc` - Clear search or return to PATH list
- `q` - Quit application

**Search Mode:**
- Type to filter files in real-time
- `Enter` - Exit search mode (keep filter)
- `Esc` - Clear search and exit search mode
- `Backspace` - Remove characters from search

## Examples

### PATH List View
```
┌ PATH Entries ─────────────────────────────────────────────────────────────┐
│ #  │ Path                                                                  │
├────┼───────────────────────────────────────────────────────────────────────┤
│ 1  │ /usr/local/bin                                                        │
│ 2  │ /usr/bin                                                              │
│ 3  │ /bin                                                                  │
│ 4  │ /usr/sbin                                                             │
└────┴─────────── Navigate <↑↓> Select <Enter> Quit <Q> ────────────────────┘
```

### Directory Contents View
```
┌ Directory: /usr/local/bin ───────────────────────────────────────────────┐
│ #  │ Name           │ Type    │ Link Target                              │
├────┼────────────────┼─────────┼──────────────────────────────────────────┤
│ 1  │ cargo          │ symlink │ -> /Users/user/.cargo/bin/cargo          │
│ 2  │ git            │ file    │                                          │
│ 3  │ node           │ symlink │ -> /usr/local/Cellar/node/18.0.0/bin/node│
└────┴──── Navigate <↑↓> Search </> Back <Esc> Quit <Q> ──────────────────┘
```

### Error Handling Example
```
┌ Directory: /restricted/path ─────────────────────────────────────────────┐
│ #  │ Name                        │ Type    │ Error                       │
├────┼─────────────────────────────┼─────────┼─────────────────────────────┤
│ 1  │ ⚠️  Unable to read directory │ file    │ Error: Permission denied    │
└────┴──── Navigate <↑↓> Search </> Back <Esc> Quit <Q> ──────────────────┘
```

### Search Mode
```
┌ Search: git ─────────────────────────────────────────────────────────────┐
│ #  │ Name           │ Type    │ Link Target                              │
├────┼────────────────┼─────────┼──────────────────────────────────────────┤
│ 1  │ git            │ file    │                                          │
│ 2  │ git-lfs        │ file    │                                          │
└── Type to search, <Enter> to confirm, <Esc> to cancel ──────────────────┘
```

## Technical Details

Built with:
- **[ratatui](https://ratatui.rs/)** - Terminal user interface framework
- **[crossterm](https://github.com/crossterm-rs/crossterm)** - Cross-platform terminal manipulation

The application reads your `PATH` environment variable, presents each directory as a selectable item, and allows deep inspection of contents with file type detection and symlink resolution.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.