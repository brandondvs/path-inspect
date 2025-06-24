# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust TUI (Terminal User Interface) application for browsing PATH directories with search functionality. Built with `ratatui` for terminal UI and `crossterm` for terminal interaction.

## Development Commands

### Building and Running
```bash
# Build the project
cargo build

# Run the application
cargo run

# Build in release mode
cargo build --release

# Run with optimizations
cargo run --release
```

### Testing and Quality
```bash
# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

## Architecture

### Core Components

- **`main.rs`**: Entry point that initializes the terminal and starts the app
- **`app.rs`**: Main application logic with state management and event handling
- **`types.rs`**: Data structures for application state (`View`, `DirectoryView`, `FileEntry`, `EntryInfo`)
- **`ui.rs`**: Terminal UI rendering functions for different views
- **`lib.rs`**: Module declarations and public API exports

### Application Flow

1. **PATH List View**: Shows numbered list of directories from `$PATH` environment variable
2. **Directory Contents View**: Displays files in selected PATH directory with search capability
3. **Search Mode**: Real-time filtering of directory contents

### Key Data Structures

- `App`: Main application state with current view, selection, and directory data
- `DirectoryView`: Manages directory contents, search state, and filtered results
- `FileEntry`: Represents a file/directory with symlink detection and metadata
- `View` enum: Tracks current UI state (PathList vs DirectoryContents)

### UI Architecture

Two main rendering functions in `ui.rs`:
- `draw_path_list()`: Renders the initial PATH directory list
- `draw_directory_contents()`: Renders selected directory with search interface

Uses `ratatui::Table` widgets with stateful selection and dynamic content filtering.