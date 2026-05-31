use crate::config;
use crate::types::{route_color, ColorRgb, LaneId, RouteType, SignalState, VehicleKind, Vec2};
use crate::world::World;

/// A car on an inbound lane with a fixed route from spawn to exit.
#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: u32,
    pub lane: LaneId,
    pub route: RouteType,
    pub kind: VehicleKind,
    route_progress: f32,
    lane_length: f32,
    path_points: Vec<Vec2>,
    path_length: f32,
    finished: bool,
}

impl Vehicle {
    pub fn new(id: u32, lane: LaneId, route: RouteType, kind: VehicleKind, world: &World) -> Self {
        let lane_info = world.lane(lane);
        let route_path = world.route(lane, route);
        let (path_points, path_length) = build_full_path(lane_info.spawn, &route_path.waypoints);

        Self {
            id,
            lane,
            route,
            kind,
            route_progress: 0.0,
            lane_length: lane_info.lane_length,
            path_points,
            path_length,
            finished: false,
        }
    }

    pub fn route_progress(&self) -> f32 {
        self.route_progress
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn color(&self) -> ColorRgb {
        route_color(self.route)
    }

    pub fn position(&self) -> Vec2 {
        sample_path(&self.path_points, self.route_progress).0
    }

    pub fn heading(&self) -> f32 {
        sample_path(&self.path_points, self.route_progress).1
    }

    /// Vehicle-local sprite size: (lateral width, forward length). Heading is applied via rotation.
    pub fn draw_sprite_size(&self) -> (f32, f32) {
        let length = match self.kind {
            VehicleKind::Car => config::VEHICLE_LENGTH,
            VehicleKind::Motorcycle => config::MOTORCYCLE_LENGTH,
        };
        let lateral = match self.kind {
            VehicleKind::Car => config::LANE_WIDTH * 0.48 * config::CAR_DRAW_WIDTH_SCALE,
            VehicleKind::Motorcycle => config::LANE_WIDTH * 0.30,
        };

        let mut w = lateral * config::VEHICLE_DRAW_SCALE;
        let mut h = length * config::VEHICLE_DRAW_SCALE;

        if self.kind == VehicleKind::Motorcycle {
            w *= config::MOTORCYCLE_DRAW_SCALE;
            h *= config::MOTORCYCLE_DRAW_SCALE;
        }

        (w, h)
    }

    pub fn compute_advance(
        &self,
        dt: f32,
        signal: SignalState,
        vehicles: &[Vehicle],
        self_index: usize,
    ) -> f32 {
        if self.finished {
            return 0.0;
        }

        let mut advance = config::VEHICLE_SPEED * dt;
        advance = cap_for_leader(self, vehicles, self_index, advance);
        advance = cap_for_lane_proximity(self, vehicles, self_index, advance);
        advance = cap_for_intersection(self, vehicles, self_index, advance);
        cap_for_red_light(self, signal, advance)
    }

    pub fn apply_advance(&mut self, advance: f32) {
        if self.finished || advance <= 0.0 {
            return;
        }
        self.route_progress += advance;
        if self.route_progress >= self.path_length {
            self.finished = true;
        }
    }
}

fn build_full_path(spawn: Vec2, route_waypoints: &[Vec2]) -> (Vec<Vec2>, f32) {
    let mut points = Vec::with_capacity(1 + route_waypoints.len());
    points.push(spawn);
    points.extend_from_slice(route_waypoints);

    let mut length = 0.0;
    for w in points.windows(2) {
        length += segment_length(w[0], w[1]);
    }
    (points, length)
}

fn segment_length(a: Vec2, b: Vec2) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

fn sample_path(path: &[Vec2], mut distance: f32) -> (Vec2, f32) {
    if path.is_empty() {
        return (Vec2 { x: 0.0, y: 0.0 }, 0.0);
    }
    if path.len() == 1 {
        return (path[0], 0.0);
    }

    for i in 0..path.len() - 1 {
        let a = path[i];
        let b = path[i + 1];
        let seg_len = segment_length(a, b);
        if distance <= seg_len || i == path.len() - 2 {
            let t = if seg_len > 0.0 {
                (distance / seg_len).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let pos = Vec2 {
                x: a.x + t * (b.x - a.x),
                y: a.y + t * (b.y - a.y),
            };
            let heading = (b.y - a.y).atan2(b.x - a.x).to_degrees();
            return (pos, heading);
        }
        distance -= seg_len;
    }

    let last = path[path.len() - 1];
    let prev = path[path.len() - 2];
    let heading = (last.y - prev.y).atan2(last.x - prev.x).to_degrees();
    (last, heading)
}

fn follow_gap(me: &Vehicle) -> f32 {
    let base = config::VEHICLE_LENGTH + config::SAFETY_GAP;
    if me.route_progress < me.lane_length {
        base + config::WAITING_EXTRA_GAP
    } else {
        base
    }
}

fn spawn_queue_gap() -> f32 {
    (config::VEHICLE_LENGTH + config::SAFETY_GAP + config::WAITING_EXTRA_GAP)
        * config::SPAWN_QUEUE_FACTOR
}

fn travel_forward(heading_deg: f32) -> (f32, f32) {
    let h = heading_deg.to_radians();
    (h.cos(), h.sin())
}

fn spatially_ahead(me: &Vehicle, other: &Vehicle) -> bool {
    let (fx, fy) = travel_forward(me.heading());
    let dx = other.position().x - me.position().x;
    let dy = other.position().y - me.position().y;
    dx * fx + dy * fy > 6.0
}

fn intersection_min_sep(me: &Vehicle) -> f32 {
    follow_gap(me) + config::INTERSECTION_EXTRA_GAP
}

fn in_intersection_zone(pos: Vec2) -> bool {
    pos.x >= config::IX0
        && pos.x <= config::IX1
        && pos.y >= config::IY0
        && pos.y <= config::IY1
}

fn dist(a: Vec2, b: Vec2) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

/// How far past the stop line (same units as `route_progress`).
fn intersection_commitment(v: &Vehicle) -> f32 {
    (v.route_progress - v.lane_length).max(0.0)
}

/// Any two different inbound approaches can cross paths in the box (turns included).
fn lanes_conflict(a: LaneId, b: LaneId) -> bool {
    use LaneId::{EastWb, NorthSb, SouthNb, WestEb};
    if a == b {
        return false;
    }
    let inbound = |l: LaneId| matches!(l, NorthSb | SouthNb | WestEb | EastWb);
    inbound(a) && inbound(b)
}

/// True if `me` must yield to `other` at `pos` (prevents mutual deadlock).
fn must_yield_to(
    me: &Vehicle,
    me_index: usize,
    pos: Vec2,
    other: &Vehicle,
    other_index: usize,
    min_sep: f32,
) -> bool {
    if !lanes_conflict(me.lane, other.lane) {
        return false;
    }

    let other_pos = other.position();
    if dist(pos, other_pos) >= min_sep {
        return false;
    }

    if me.lane == other.lane {
        return spatially_ahead(me, other);
    }

    let my_commit = intersection_commitment(me);
    let other_commit = intersection_commitment(other);

    if my_commit > other_commit + 10.0 {
        return false;
    }
    if my_commit < other_commit - 10.0 {
        return true;
    }

    me_index > other_index
}

fn position_clear_at(
    pos: Vec2,
    me: &Vehicle,
    me_index: usize,
    vehicles: &[Vehicle],
    min_sep: f32,
) -> bool {
    if !in_intersection_zone(pos) {
        return true;
    }

    for (i, other) in vehicles.iter().enumerate() {
        if i == me_index || other.finished {
            continue;
        }
        let other_pos = other.position();
        if !in_intersection_zone(other_pos) {
            continue;
        }
        if must_yield_to(me, me_index, pos, other, i, min_sep) {
            return false;
        }
    }
    true
}

fn intersection_move_ok(
    me: &Vehicle,
    vehicles: &[Vehicle],
    self_index: usize,
    advance: f32,
    min_sep: f32,
) -> bool {
    if advance <= 0.0 {
        return true;
    }

    const SAMPLES: usize = 6;
    for step in 1..=SAMPLES {
        let delta = advance * step as f32 / SAMPLES as f32;
        let pos = sample_path(&me.path_points, me.route_progress + delta).0;
        if !position_clear_at(pos, me, self_index, vehicles, min_sep) {
            return false;
        }
    }
    true
}

fn cap_for_intersection(
    me: &Vehicle,
    vehicles: &[Vehicle],
    self_index: usize,
    advance: f32,
) -> f32 {
    if advance <= 0.0 {
        return 0.0;
    }

    let min_sep = intersection_min_sep(me);
    if intersection_move_ok(me, vehicles, self_index, advance, min_sep) {
        return advance;
    }

    let mut lo = 0.0f32;
    let mut hi = advance;
    for _ in 0..10 {
        let mid = (lo + hi) * 0.5;
        if intersection_move_ok(me, vehicles, self_index, mid, min_sep) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

/// All cars on the same approach queue behind each other.
fn shares_following_group(me: &Vehicle, other: &Vehicle) -> bool {
    me.lane == other.lane
}

fn leader_progress(me: &Vehicle, vehicles: &[Vehicle], self_index: usize) -> Option<f32> {
    let mut nearest: Option<f32> = None;
    for (i, other) in vehicles.iter().enumerate() {
        if i == self_index || other.finished {
            continue;
        }
        if !shares_following_group(me, other) || me.route != other.route {
            continue;
        }
        if other.route_progress <= me.route_progress {
            continue;
        }
        nearest = Some(match nearest {
            Some(n) => n.min(other.route_progress),
            None => other.route_progress,
        });
    }
    nearest
}

fn cap_for_leader(me: &Vehicle, vehicles: &[Vehicle], self_index: usize, advance: f32) -> f32 {
    let Some(leader) = leader_progress(me, vehicles, self_index) else {
        return advance;
    };
    let max_advance = leader - me.route_progress - follow_gap(me);
    advance.min(max_advance.max(0.0))
}

/// Same-lane spacing by position (covers different routes in the intersection).
fn cap_for_lane_proximity(
    me: &Vehicle,
    vehicles: &[Vehicle],
    self_index: usize,
    advance: f32,
) -> f32 {
    let gap = follow_gap(me);
    for (i, other) in vehicles.iter().enumerate() {
        if i == self_index || other.finished || other.lane != me.lane {
            continue;
        }
        if dist(me.position(), other.position()) >= gap {
            continue;
        }
        if spatially_ahead(me, other) {
            return 0.0;
        }
    }
    advance
}

/// Path progress limit on red — front bumper stays before the crosswalk band.
fn red_light_hold_progress(lane_length: f32) -> f32 {
    (lane_length - config::red_light_hold_back()).max(0.0)
}

fn cap_for_red_light(me: &Vehicle, signal: SignalState, advance: f32) -> f32 {
    if signal == SignalState::Green {
        return advance;
    }
    // Already committed past the stop line — keep clearing the intersection.
    if me.route_progress > me.lane_length {
        return advance;
    }
    // Red: hold before the crossing/stop line markings.
    let hold = red_light_hold_progress(me.lane_length);
    let allowed = hold - me.route_progress;
    advance.min(allowed.max(0.0))
}

/// True if the lane queue is too close to the spawn point for another car.
pub fn lane_spawn_blocked(vehicles: &[Vehicle], lane: LaneId, spawn: Vec2) -> bool {
    let clearance = spawn_queue_gap();
    vehicles.iter().any(|v| {
        v.lane == lane && !v.finished && dist(v.position(), spawn) < clearance
    })
}

/// Vehicles waiting on the approach (before the stop line).
pub fn queue_count_on_lane(vehicles: &[Vehicle], lane: LaneId, lane_length: f32) -> i32 {
    vehicles
        .iter()
        .filter(|v| {
            v.lane == lane
                && !v.finished
                && v.route_progress < lane_length
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SignalState;

    #[test]
    fn path_starts_at_spawn() {
        let world = World::new();
        let v = Vehicle::new(1, LaneId::SouthNb, RouteType::Straight, VehicleKind::Car, &world);
        let spawn = world.lane(LaneId::SouthNb).spawn;
        let pos = v.position();
        assert!((pos.x - spawn.x).abs() < 0.01);
        assert!((pos.y - spawn.y).abs() < 0.01);
    }

    #[test]
    fn south_spawn_uses_right_hand_enter_lane() {
        let world = World::new();
        let spawn = world.lane(LaneId::SouthNb).spawn;
        assert!((spawn.x - config::ENTER_NB_X).abs() < 0.01);
    }

    #[test]
    fn north_spawn_uses_right_hand_enter_lane() {
        let world = World::new();
        let spawn = world.lane(LaneId::NorthSb).spawn;
        assert!((spawn.x - config::ENTER_SB_X).abs() < 0.01);
    }

    #[test]
    fn red_light_holds_before_stop_line() {
        let world = World::new();
        let mut v = Vehicle::new(1, LaneId::SouthNb, RouteType::Straight, VehicleKind::Car, &world);
        let lane_length = world.lane(LaneId::SouthNb).lane_length;
        v.apply_advance(lane_length - 1.0);
        let vehicles = [v.clone()];
        let advance = v.compute_advance(1.0, SignalState::Red, &vehicles, 0);
        v.apply_advance(advance);
        assert!(v.route_progress() <= lane_length + 0.01);
    }

    #[test]
    fn red_light_holds_before_crossing_markings() {
        let world = World::new();
        let mut v = Vehicle::new(1, LaneId::SouthNb, RouteType::Straight, VehicleKind::Car, &world);
        let lane_length = world.lane(LaneId::SouthNb).lane_length;
        let hold = red_light_hold_progress(lane_length);
        v.apply_advance(hold);
        let vehicles = [v.clone()];
        let advance = v.compute_advance(1.0, SignalState::Red, &vehicles, 0);
        assert_eq!(advance, 0.0);
        assert!(v.route_progress() < lane_length - 1.0);
    }

    #[test]
    fn red_light_does_not_drive_up_to_stop_line() {
        let world = World::new();
        let mut v = Vehicle::new(1, LaneId::SouthNb, RouteType::Straight, VehicleKind::Car, &world);
        let lane_length = world.lane(LaneId::SouthNb).lane_length;
        let hold = red_light_hold_progress(lane_length);
        v.apply_advance(hold - 5.0);
        let vehicles = [v.clone()];
        let advance = v.compute_advance(1.0, SignalState::Red, &vehicles, 0);
        v.apply_advance(advance);
        assert!(v.route_progress() <= hold + 0.01);
    }

    #[test]
    fn green_light_passes_stop_line() {
        let world = World::new();
        let mut v = Vehicle::new(1, LaneId::SouthNb, RouteType::Straight, VehicleKind::Car, &world);
        let lane_length = world.lane(LaneId::SouthNb).lane_length;
        v.apply_advance(lane_length);
        let vehicles = [v.clone()];
        let advance = v.compute_advance(1.0, SignalState::Green, &vehicles, 0);
        assert!(advance > 0.0);
    }

    #[test]
    fn intersection_zone_contains_center() {
        let center = Vec2 {
            x: config::CENTER_X,
            y: config::CENTER_Y,
        };
        assert!(in_intersection_zone(center));
    }

    #[test]
    fn intersection_blocks_advance_when_too_close() {
        let world = World::new();
        let mut a = Vehicle::new(1, LaneId::NorthSb, RouteType::Straight, VehicleKind::Car, &world);
        let mut b = Vehicle::new(2, LaneId::WestEb, RouteType::Straight, VehicleKind::Car, &world);
        let lane_a = a.lane_length;
        let lane_b = b.lane_length;
        a.apply_advance(lane_a + 40.0);
        b.apply_advance(lane_b + 40.0);
        let dist_now = dist(a.position(), b.position());
        if dist_now >= intersection_min_sep(&a) {
            return;
        }
        let vehicles = vec![a.clone(), b.clone()];
        let advance = a.compute_advance(1.0, SignalState::Green, &vehicles, 0);
        assert!(advance < config::VEHICLE_SPEED * 0.5);
    }

    #[test]
    fn follower_stops_behind_leader() {
        let world = World::new();
        let mut leader = Vehicle::new(1, LaneId::NorthSb, RouteType::Straight, VehicleKind::Car, &world);
        let mut follower = Vehicle::new(2, LaneId::NorthSb, RouteType::Straight, VehicleKind::Car, &world);
        let gap = follow_gap(&follower);
        leader.apply_advance(gap + 20.0);
        let vehicles = vec![leader.clone(), follower.clone()];
        let advance = follower.compute_advance(1.0, SignalState::Green, &vehicles, 1);
        follower.apply_advance(advance);
        assert!(follower.route_progress() <= gap + 0.01);
    }
}
