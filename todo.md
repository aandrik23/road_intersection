# Road Intersection Simulation — Team Split (3 people)

Greenfield project: SDL2 recommended. Split below minimizes blocking: **Person 1** owns the world model and rendering shell; **Person 2** owns traffic-light logic; **Person 3** owns vehicles and input. Agree on shared types early (lane IDs, directions, coordinates).

---

## Shared (all three — agree in a short kickoff)

- [ ] Pick language, build system, and SDL2 setup (window, loop, delta time).
- [ ] Define constants: `vehicle_length`, `safety_gap`, simulation units (pixels or meters).
- [ ] Agree on lane identifiers: four approaches × two directions = **8 entry lanes** (one lane per direction per road arm).
- [ ] Agree on route enum: `LEFT`, `RIGHT`, `STRAIGHT` (fixed at spawn, never changes).
- [ ] Document color → route mapping for audit (e.g. yellow = right, blue = left, green = straight).
- [ ] Integration contract (headers/APIs): lane geometry, spawn points, stop lines, traffic-light state per lane, vehicle list updates.

---

## Person 1 — Environment & roads (foundation)

**Goal:** Build the intersection world and visual shell so others can plug in lights and cars.

### Roads & intersection

- [ ] Two roads crossing at a four-way intersection (per spec diagram: North / South / East / West).
- [ ] Each road: **one lane per direction** (right-hand flow as in diagram).
- [ ] Mark lane centerlines, edges, and intersection box (display style is flexible).
- [ ] Place **stop lines** where each lane enters the intersection.
- [ ] Define `lane_length` per lane: distance from stop line to spawn point (needed for capacity formula).
- [ ] Define spawn points per approach (used by Person 3 keyboard commands).

### Routing geometry (data only — movement implemented by Person 3)

- [ ] For each entry lane, define valid paths through the intersection: left turn, right turn, straight (waypoints or spline segments).
- [ ] Expose route paths so vehicles can follow them without changing route mid-trip.

### Rendering shell

- [ ] SDL2 window, main loop, clear/draw/update frame.
- [ ] Draw roads, lanes, direction arrows, and labels (N/S/E/W).
- [ ] Draw traffic-light **positions** at each lane’s entry (bulbs can be static placeholders until Person 2 drives state).
- [ ] Hook for Person 2: `get_lane_queue_count(lane_id)` or equivalent (may return stub `0` until Person 3 exists).
- [ ] Hook for Person 3: render vehicle list passed from simulation state.

### Deliverables

- [ ] Runnable app showing the intersection (no cars or logic required for first demo).
- [ ] Documented lane/spawn/stop-line data structures shared with the team.

### Acceptance criteria

- [ ] All 8 entry lanes identifiable with consistent IDs.
- [ ] `lane_length` measurable and documented for each lane.
- [ ] Route paths exist for all spawn lane × {left, right, straight} combinations that are physically valid.

---

## Person 2 — Traffic lights & congestion control

**Goal:** Red/green signals at every lane entry, safe intersection timing, dynamic response to queue length.

**Depends on:** Person 1 lane IDs, stop lines, and `lane_length`. Can start with mock queue counts, then wire to real data.

### Traffic lights

- [ ] One signal per lane where traffic **enters** the intersection (8 lights).
- [ ] Colors: **red** and **green** only (no yellow required).
- [ ] Render state (coordinate with Person 1’s light positions).
- [ ] Vehicles must see per-lane signal state (API: e.g. `is_green(lane_id)`).

### Control algorithm

- [ ] Choose a scheduling strategy (fixed phases, actuated, round-robin, etc.) that **prevents collisions** between conflicting movements.
- [ ] Define conflicting lane groups (e.g. N-S straight vs E-W straight) and enforce mutual exclusion on green.
- [ ] Implement phase timing (minimum green, optional red clearance if needed for safety).

### Dynamic congestion rule

- [ ] Implement capacity per lane:
  ```
  capacity = floor(lane_length / (vehicle_length + safety_gap))
  ```
- [ ] Read queue length per lane (vehicles waiting before stop line — coordinate counting rules with Person 3).
- [ ] When queue count **reaches capacity**, adapt logic (e.g. extend green for that lane / prioritize that phase) to avoid overflow past spawn.
- [ ] Primary objective: no intersection collisions; secondary: keep queues below capacity when possible.

### Deliverables

- [ ] Traffic-light controller module with tests or logged scenarios for phase changes.
- [ ] Short write-up of algorithm and conflict matrix (for audit).

### Acceptance criteria

- [ ] No two conflicting lanes green at the same time.
- [ ] Green extension (or equivalent) triggers when a lane hits capacity.
- [ ] All 8 entry lanes have a controllable red/green state visible in the sim.

---

## Person 3 — Vehicles, behavior & keyboard commands

**Goal:** Spawn cars safely, follow routes and rules, respect lights and following distance.

**Depends on:** Person 1 paths/spawn/lanes; Person 2 signal state API.

### Vehicle model

- [ ] Vehicle entity: position, heading, route (fixed at spawn), color by route, constant speed.
- [ ] Follow assigned path from Person 1; **route cannot change** after spawn.
- [ ] Stop at **red**; proceed on **green** (per lane’s entry signal).
- [ ] Maintain **safety gap** behind the vehicle ahead on the same lane/path; if leader stops, follower stops in time.
- [ ] No emergency or priority vehicle types.

### Spawning & keyboard

- [ ] `↑` — spawn from **south**, toward intersection.
- [ ] `↓` — spawn from **north**, toward intersection.
- [ ] `→` — spawn from **west**, toward intersection.
- [ ] `←` — spawn from **east**, toward intersection.
- [ ] `r` — spawn from **random** direction (among the four).
- [ ] `Esc` — exit simulation cleanly.
- [ ] **Anti-spam:** cannot spawn if the new vehicle would violate safe distance from the last vehicle on that lane (use `vehicle_length + safety_gap` or equivalent).
- [ ] **Random route** at spawn: left / right / straight (uniform or weighted — document choice).

### Simulation integration

- [ ] Update vehicle positions each frame at fixed velocity (when allowed to move).
- [ ] Queue detection for Person 2: count vehicles in lane between spawn and stop line.
- [ ] Remove or despawn vehicles after they exit the intersection area (define exit criteria with Person 1).
- [ ] Provide vehicle list + state to Person 1 for drawing (simple rects/sprites OK for v1).

### Deliverables

- [ ] Vehicle module + input handler.
- [ ] Color–route legend (for audit).

### Acceptance criteria

- [ ] Spawn keys work for all four directions + random; Esc quits.
- [ ] Rapid key presses do not create overlapping or unsafe spawns.
- [ ] Cars stop for red and for a stopped car ahead.
- [ ] Cars complete left/right/straight paths without cutting through invalid areas.

---

## Integration checklist (when all three merge)

- [ ] Person 1 draws world + cars; Person 2 updates lights each tick; Person 3 updates cars each tick — order documented (e.g. input → vehicles → lights → draw).
- [ ] End-to-end: spawn from each direction, random routes, lights cycle, queues grow, green extends under congestion.
- [ ] No crashes at intersection under normal load.
- [ ] README: build/run, controls, color legend, light algorithm summary.

---

## Bonus (optional — any owner after core is done)

- [ ] Vehicle / traffic-light animations.
- [ ] Sprite or image assets (e.g. limezu, finalbossblue, spriters-resource).
- [ ] Polish: sounds, FPS counter, pause, config file for timings.

---

## Reference

- Example project mentioned in brief: `road_intersection` (use as inspiration for layout/controls, not necessarily code to copy).
