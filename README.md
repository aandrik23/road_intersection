# Road Intersection Simulation

Traffic intersection simulation (SDL2). This repository is split across three contributors; **Person 1** delivers the environment, roads, route geometry, and rendering shell.

## Requirements

- C compiler (clang/gcc)
- [SDL2](https://wiki.libsdl.org/SDL2/Installation) development libraries

macOS (Homebrew):

```sh
brew install sdl2
```

## Build & run

```sh
make
make run
```

On start, lane lengths and capacities are printed to the terminal. The window shows the four-way intersection, eight traffic-light placeholders (red by default), stop lines, and direction arrows. Press **Esc** to quit.

## Person 1 scope (done)

- Two crossing roads, one lane per direction (8 lanes)
- Stop lines, spawn points, `lane_length`, route waypoints
- SDL2 main loop and drawing
- Integration hooks for Persons 2 and 3 (see `docs/LANE_DATA.md`)

## Team docs

- `docs/LANE_DATA.md` — lane IDs, spawn mapping, colors, APIs
- `todo.md` — full team task list (unchanged checklist)

## Controls (full project — Person 3)

| Key | Action |
|-----|--------|
| ↑ | Spawn from south |
| ↓ | Spawn from north |
| → | Spawn from west |
| ← | Spawn from east |
| r | Random direction |
| Esc | Exit |

Vehicle spawning is not wired in the Person 1 demo.
