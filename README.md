# Road Intersection

Rust + SDL2 simulation of a four-way intersection: two crossing roads, traffic lights, and vehicles that follow left / right / straight routes.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- SDL2

**macOS (Homebrew):**

```sh
brew install sdl2
```

**Debian / Ubuntu:**

```sh
sudo apt install libsdl2-dev
```

On macOS, `.cargo/config.toml` links against Homebrew libraries.

## Run

```sh
cargo run
```

Close the window or press **Esc** to exit.

## Controls (full simulation)

| Key | Action |
|-----|--------|
| ↑ | Vehicle from the south |
| ↓ | Vehicle from the north |
| → | Vehicle from the west |
| ← | Vehicle from the east |
| r | Random approach |
| Esc | Quit |

## Project layout

- `src/` — simulation code
- `docs/LANE_DATA.md` — lane IDs, spawns, integration notes for the team
- `todo.md` — task split for the group
