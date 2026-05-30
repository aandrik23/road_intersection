# Lane & integration data (Person 1, Rust)

Shared contract for Persons 2 (traffic lights) and 3 (vehicles).

## Constants (`src/config.rs`)

| Symbol | Value | Meaning |
|--------|------:|---------|
| `VEHICLE_LENGTH` | 28 px | Car length |
| `SAFETY_GAP` | 12 px | Minimum gap between vehicles |
| `ARM_LENGTH` | 260 px | Spawn to stop line |

**Capacity:** `floor(lane_length / (VEHICLE_LENGTH + SAFETY_GAP))` → **6** per inbound lane.

## Lane IDs (`LaneId` in `src/types.rs`)

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

`lane_for_spawn_direction(0..3)` → ↑ ↓ → ← order.

## Routes & colors

- `RouteType::{Left, Right, Straight}` — fixed at spawn.
- `route_color()` — blue / yellow / green.

## APIs

| Module | Use |
|--------|-----|
| `world::World` | Lanes, spawns, stop lines, `RoutePath` waypoints |
| `simulation::get_lane_queue_count` | Stub `0` (Person 2) |
| `simulation::Simulation::set_lane_signal` | Per-lane red/green (Person 2) |
| `simulation::Simulation::vehicles` | Person 3 pushes `VehicleDraw` for rendering |

## Tick order (integration)

1. Input (Person 3)  
2. Update vehicles (Person 3)  
3. Update lights (Person 2)  
4. `render::AppRenderer::draw_frame` (Person 1)
