# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build          # Build the project
cargo run            # Run the game
cargo check          # Type-check without building
cargo clippy         # Run linter
cargo test           # Run tests
```

## Architecture

Worldfall is a terminal-based roguelike dungeon crawler written in Rust. It uses crossterm for terminal rendering and input handling.

### Core Game Loop (main.rs)

The `Game` struct orchestrates everything:
1. Initializes renderer, generates map, spawns player and enemies
2. Main loop: render → get input → handle player action → enemy turns → check win/lose
3. Player moves or attacks (bumping into enemies triggers combat)
4. Enemies chase the player within `ENEMY_CHASE_RANGE` and attack when adjacent

### Module Responsibilities

- **map.rs**: Procedural dungeon generation using room placement with corridors. `Map` contains `Tile` grid and `Room` list. Rooms are non-overlapping rectangles connected by L-shaped corridors.
- **player.rs**: Player entity with position, HP, and power stats. Simple movement via `move_by(dx, dy)`.
- **enemy.rs**: Enemy entities with `EnemyType` enum for different monster types (currently only Goblin). Enemies use simple chase AI (`move_toward`) with collision avoidance.
- **combat.rs**: Turn-based combat with randomized damage variance. Returns `CombatResult` with damage dealt and message string.
- **render.rs**: Crossterm-based terminal renderer. Handles raw mode, cursor hiding, map/entity drawing, status bar, and message log (last 5 messages, displays 3).
- **input.rs**: Keyboard input mapping to `Action` enum (Move, Quit, None). Supports arrow keys, WASD, and quit keys (Q, Esc, Ctrl+C).

### Key Constants (main.rs)

- `MAP_WIDTH`/`MAP_HEIGHT`: Dungeon dimensions (60x20)
- `NUM_ROOMS`: Target room count (6)
- `MIN_ROOM_SIZE`/`MAX_ROOM_SIZE`: Room size bounds (4-8)
- `ENEMY_CHASE_RANGE`: Distance at which enemies pursue player (8)

### Entity Representation

- Player: `@` character
- Goblin: `g` character
- Walls: `#`, Floors: `.`, Doors: `+`
