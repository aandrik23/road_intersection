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

Retro sound effects (traffic-light phase change, spawn blip) use SDL2 audio — no extra libraries. If audio fails to open (e.g. headless), the sim still runs silently.

## Controls

| Key | Action |
|-----|--------|
| ↑ | Spawn from the **south**, toward the intersection |
| ↓ | Spawn from the **north**, toward the intersection |
| → | Spawn from the **west**, toward the intersection |
| ← | Spawn from the **east**, toward the intersection |
| r | Spawn from a **random** direction |
| m | Toggle sound **mute** |
| Esc | Quit |

Each spawn picks a random route (left / right / straight) and vehicle type (car or motorcycle). Rapid key presses are blocked if the lane already has a vehicle too close to the spawn point (`vehicle_length + safety_gap`).

## How this maps to the project brief

### Roads

- Two roads cross at a four-way intersection (North / South / East / West).
- Each arm has **one lane per direction** (right-hand traffic, as in the spec diagram).
- Matches the spec diagram: **west column ↓**, **east column ↑**; **north row ←**, **south row →**.
- Direction arrows on entering lanes only; lane centerlines on departing half of each arm.
- Paths through the box support left, right, and straight for each approach.

### Traffic lights

- **Four** signals — one per approach where traffic **enters** the intersection (`NorthSb`, `SouthNb`, `EastWb`, `WestEb`).
- **Colors:** red and green only (no yellow).
- Four-phase sequential controller: one inbound approach green at a time (`NorthSb` → `SouthNb` → `WestEb` → `EastWb`). Conflicting movements never green together.
- **Timing:** minimum green 3s, normal max 8s. **Congestion:** `capacity = floor(lane_length / (vehicle_length + safety_gap))`. If a lane’s queue reaches capacity, green extends up to 14s before switching phase.

### Vehicles

- Fixed speed, fixed route and color at spawn.
- Car or motorcycle at spawn (`random_vehicle_kind` — roughly 25% motorcycles).
- Stop at red; follow with a safe gap behind the vehicle ahead.
- Despawn after completing the exit path.
- No emergency or priority vehicle types.

## Constants

Defined in `src/config.rs`:

| Symbol | Value | Meaning |
|--------|------:|---------|
| `VEHICLE_LENGTH` | 22 px | Car length |
| `MOTORCYCLE_LENGTH` | 15.5 px | Motorcycle length |
| `SAFETY_GAP` | 16 px | Minimum gap between vehicles |
| `ARM_LENGTH` | 290 px | Spawn to stop line |

**Capacity per inbound lane:** `floor(lane_length / (VEHICLE_LENGTH + SAFETY_GAP))` → **7** (uses car length for the slot size).

## Lane IDs

Eight lanes total: four approaches × two directions (4 enter + 4 exit). Four **inbound** lanes where traffic enters the intersection.

| Variant | Name | Inbound | Keyboard spawn |
|---------|------|---------|----------------|
| `NorthSb` | N_SB | yes | ↓ Down |
| `NorthNb` | N_NB | no | — |
| `SouthNb` | S_NB | yes | ↑ Up |
| `SouthSb` | S_SB | no | — |
| `EastWb` | E_WB | yes | ← Left |
| `EastEb` | E_EB | no | — |
| `WestEb` | W_EB | yes | → Right |
| `WestWb` | W_WB | no | — |

`LaneId` is defined in `src/types.rs`. `lane_for_spawn_direction(0..3)` maps to ↑ ↓ → ← order.

## Routes & colors

- `RouteType::{Left, Right, Straight}` — fixed at spawn, never changes.
- Vehicles are painted by route via `route_color()` in `src/types.rs`.

| Route | Color | RGB |
|-------|-------|-----|
| Left | Blue | `(0, 112, 236)` |
| Right | Yellow | `(236, 188, 0)` |
| Straight | Green | `(0, 168, 0)` |

Random route at spawn: uniform over left / right / straight (`random_route_uniform` in `types.rs`).

## Architecture

### Module layout

| Path | Purpose |
|------|---------|
| `src/config.rs` | Shared constants (dimensions, speeds, layout) |
| `src/types.rs` | Lane IDs, routes, colors, spawn helpers |
| `src/world.rs` | Lanes, spawns, stop lines, paths |
| `src/simulation.rs` | Vehicle list, spawning, queue counts |
| `src/traffic_lights.rs` | Phase timing and congestion |
| `src/vehicle.rs` | Movement, following, lights |
| `src/input.rs` | Keyboard spawn |
| `src/render.rs` | Drawing |
| `src/sprites.rs` | Vehicle and traffic-light textures |
| `src/audio.rs` | Phase-change and spawn sound effects |

### Integration APIs

| Module | Use |
|--------|-----|
| `world::World` | Lanes, spawns, stop lines, `RoutePath` waypoints |
| `simulation::get_lane_queue_count` | Queue length per lane (for congestion control) |
| `simulation::Simulation::set_lane_signal` | Per-lane red/green state |
| `simulation::Simulation::vehicles` | Vehicle list for rendering |

### Tick order

Each frame runs in this order (`main.rs`):

1. Input
2. Update vehicles
3. Update lights
4. `render::AppRenderer::draw_frame`

## Project status

**Complete.** Three-person split: world/rendering, traffic lights, vehicles & input. All core requirements are implemented (roads, four-phase signals with congestion extension, vehicle behavior, keyboard spawn, sprites, sound).

## Tests

```sh
cargo test
```
