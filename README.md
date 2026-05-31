# Road Intersection

SDL2 simulation of a four-way intersection: two crossing roads, red/green traffic lights, and vehicles that follow fixed left / right / straight routes.

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

## Controls

| Key | Action |
|-----|--------|
| ↑ | Spawn from the **south**, toward the intersection |
| ↓ | Spawn from the **north**, toward the intersection |
| → | Spawn from the **west**, toward the intersection |
| ← | Spawn from the **east**, toward the intersection |
| r | Spawn from a **random** direction |
| Esc | Quit |

Each spawn picks a random route (left / right / straight). Rapid key presses are blocked if the lane already has a vehicle too close to the spawn point (`vehicle_length + safety_gap`).

## How this maps to the project brief

### Roads

- Two roads cross at a four-way intersection (North / South / East / West).
- Each arm has **one lane per direction** (right-hand traffic, as in the spec diagram).
- Matches the spec diagram: **west column ↓**, **east column ↑**; **north row ←**, **south row →**.
- Direction arrows on entering lanes only; lane centerlines on departing half of each arm.
- Paths through the box support left, right, and straight for each approach.

### Traffic lights

- **Four** signals — one per approach where traffic **enters** the intersection (`NorthSb`, `SouthNb`, `EastWb`, `WestEb`).
- **Red** and **green** only.
- Two-phase controller: North–South green, then East–West green (conflicting movements never green together).
- **Congestion:** `capacity = floor(lane_length / (vehicle_length + safety_gap))`. If a lane’s queue reaches capacity, green time can extend (up to 14s) before switching phase.

Constants: `vehicle_length = 28`, `safety_gap = 12` (pixels) in `src/config.rs`.

### Vehicles

- Fixed speed, fixed route and color at spawn.
- Stop at red; follow with a safe gap behind the car ahead.
- Despawn after completing the exit path.

### Route colors (audit)

| Route | Color |
|-------|--------|
| Left | Blue |
| Right | Yellow |
| Straight | Green |

See [docs/ROUTE_COLORS.md](docs/ROUTE_COLORS.md).

## Project layout

| Path | Purpose |
|------|---------|
| `src/world.rs` | Lanes, spawns, stop lines, paths |
| `src/traffic_lights.rs` | Phase timing and congestion |
| `src/vehicle.rs` | Movement, following, lights |
| `src/input.rs` | Keyboard spawn |
| `src/render.rs` | Drawing |
| `docs/LANE_DATA.md` | Lane IDs and integration |
| `todo.md` | Team task split |

## Tests

```sh
cargo test
```
