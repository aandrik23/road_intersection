# Lane & integration data (Person 1)

Shared contract for Persons 2 (traffic lights) and 3 (vehicles).

## Constants (`include/sim_config.h`)

| Symbol | Value | Meaning |
|--------|------:|---------|
| `VEHICLE_LENGTH` | 28 px | Car length in simulation units |
| `SAFETY_GAP` | 12 px | Minimum gap between vehicles |
| `ARM_LENGTH` | 260 px | Spawn point to stop line (per inbound lane) |
| `LANE_WIDTH` | 34 px | Single-lane width |

**Capacity** (used by Person 2):

```
capacity = floor(lane_length / (vehicle_length + safety_gap))
```

With defaults: slot = 40 px → capacity = `floor(260 / 40) = 6` per inbound lane.

## Lane IDs (`LaneId` in `include/types.h`)

Eight lanes = four arms × two directions (right-hand flow).

| ID | Name | Arm | Direction | Inbound | Keyboard spawn |
|----|------|-----|-----------|---------|----------------|
| `LANE_NORTH_SB` | N_SB | North | Southbound ↓ | yes | ↓ Down |
| `LANE_NORTH_NB` | N_NB | North | Northbound ↑ | no | — |
| `LANE_SOUTH_NB` | S_NB | South | Northbound ↑ | yes | ↑ Up |
| `LANE_SOUTH_SB` | S_SB | South | Southbound ↓ | no | — |
| `LANE_EAST_WB` | E_WB | East | Westbound ← | yes | ← Left |
| `LANE_EAST_EB` | E_EB | East | Eastbound → | no | — |
| `LANE_WEST_EB` | W_EB | West | Eastbound → | yes | → Right |
| `LANE_WEST_WB` | W_WB | West | Westbound ← | no | — |

`world_lane_for_spawn_direction(0..3)` maps: 0=↑ south spawn, 1=↓ north, 2=→ west, 3=← east.

## Route enum (`RouteType`)

- `ROUTE_LEFT`, `ROUTE_RIGHT`, `ROUTE_STRAIGHT` — fixed at spawn, never changes.

## Color → route (audit legend)

| Route | RGB | Color |
|-------|-----|-------|
| LEFT | (60, 140, 255) | Blue |
| RIGHT | (255, 210, 40) | Yellow |
| STRAIGHT | (50, 200, 90) | Green |

Use `route_color(route)` from `types.h`.

## Geometry API (`include/world.h`)

- `world_init(World *)` — lanes, spawn, stop lines, lights, route waypoints.
- `world_lane(world, lane_id)` → `LaneInfo` (`spawn`, `stop_line`, `light_pos`, `lane_length`, `heading`, `inbound`).
- `world_route(world, lane_id, route)` → `RoutePath` (`waypoints[]`, `path_length`).

Paths are polylines through the intersection (left / right / straight) for every lane × route combination.

## Person 2 hooks (`include/simulation.h`)

- `get_lane_queue_count(sim, lane_id)` — **stub returns 0** until Person 3 counts queues.
- `simulation_set_lane_signal` / `simulation_lane_signal` — per-lane red/green.
- Light draw positions: `LaneInfo.light_pos` (just before each stop line).

## Person 3 hooks

- Spawn at `LaneInfo.spawn` for the lane matching the arrow key.
- Follow `RoutePath.waypoints` for the chosen route.
- Render via `Simulation.vehicles[]` + `vehicle_count`; Person 1 draws them in `renderer_draw_frame`.
- Anti-spam: do not spawn if distance to tail vehicle &lt; `VEHICLE_LENGTH + SAFETY_GAP`.

## Suggested tick order (integration)

1. Input (Person 3)
2. Update vehicles (Person 3)
3. Update traffic lights (Person 2)
4. Draw (Person 1 renderer)
