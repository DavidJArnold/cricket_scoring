# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Development
- `cargo build` - Build the project
- `cargo run --example cricsheet` - Run the cricsheet example that processes cricket match data
- `cargo test` - Run tests
- `cargo clippy` - Run Clippy linter
- `cargo fmt` - Format code

### Running Examples
- The main example is `cricsheet.rs` which processes cricket match data from JSON files
- Example data is stored in `examples/all_matches/` and `examples/test_matches/` directories
- Run with: `cargo run --example cricsheet`

## Architecture

This is a Rust library for cricket scoring and match analysis, particularly focused on processing Cricsheet data format.

### Core Structure

- **`src/lib.rs`** - Main library entry point exposing error and scoring modules
- **`src/scoring/`** - Core scoring logic:
  - `game.rs` - Main Game struct with teams, innings, meta data, and outcome calculation
  - `innings.rs` - Individual innings tracking 
  - `ball.rs` - Individual ball/delivery tracking
  - `player.rs` - Player and team definitions
  - `score.rs` - Score tracking (runs, wickets, etc.)

### Key Components

- **Game**: Central struct containing teams, innings list, metadata, and final outcome
- **Innings**: Represents a single innings with batting/bowling teams and score
- **Outcome**: Determines match result (win/loss/draw/tie) with margins and methods
- **Cricsheet Integration**: Examples demonstrate parsing real cricket data from JSON format

### Data Processing

The cricsheet example (`examples/cricsheet.rs`) processes thousands of cricket match files from the `examples/all_matches/` directory, comparing calculated results against actual match outcomes for validation.

### Dependencies

- `thiserror` - Error handling
- `chrono`, `serde`, `serde_json` - For dev/example data processing