# Road Intersection Simulation (Rust)

SDL2 traffic intersection simulation. **Person 1** provides the environment, roads, route geometry, and rendering shell.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
Install SDL2, then build:

```sh
brew install sdl2          # macOS
# sudo apt install libsdl2-dev   # Debian/Ubuntu
```

On macOS, `.cargo/config.toml` points the linker at Homebrew’s SDL2.

## Build & run

```sh
cargo build
cargo run
```

Lane capacities are printed on startup. The window shows the intersection and eight traffic-light placeholders (red). Press **Esc** or close the window to quit.

## Layout

```
src/
  main.rs        — SDL event loop
  lib.rs
  config.rs      — shared constants
  types.rs       — LaneId, RouteType, colors
  world.rs       — lanes, spawns, route waypoints
  simulation.rs  — state + integration hooks
  render.rs      — drawing
docs/LANE_DATA.md — team integration contract
todo.md           — full team checklist (unchanged)
```

## Person 1 (done)

- Four-way intersection, 8 lanes, stop lines, spawn points
- Route polylines for left / right / straight
- SDL2 rendering shell and hooks for teammates

## Full project controls (Person 3)

| Key | Action |
|-----|--------|
| ↑ | Spawn from south |
| ↓ | Spawn from north |
| → | Spawn from west |
| ← | Spawn from east |
| r | Random direction |
| Esc | Exit |

Spawning is not implemented in the Person 1 demo.
