use crate::config;
use crate::types::{route_color, ColorRgb, LaneId, RouteType, SignalState, Vec2};
use crate::world::World;

/// A car on an inbound lane with a fixed route from spawn to exit.
#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: u32,
    pub lane: LaneId,
    pub route: RouteType,
    route_progress: f32,
    lane_length: f32,
    path_points: Vec<Vec2>,
    path_length: f32,
    finished: bool,
}

impl Vehicle {
    pub fn new(id: u32, lane: LaneId, route: RouteType, world: &World) -> Self {
        let lane_info = world.lane(lane);
        let route_path = world.route(lane, route);
        let (path_points, path_length) = build_full_path(lane_info.spawn, &route_path.waypoints);

        Self {
            id,
            lane,
            route,
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

    pub fn draw_extents(&self) -> (f32, f32) {
        let h = self.heading().abs();
        let narrow = config::LANE_WIDTH * 0.55;
        if (45.0..135.0).contains(&h) || (225.0..315.0).contains(&h) {
            (config::VEHICLE_LENGTH, narrow)
        } else {
            (narrow, config::VEHICLE_LENGTH)
        }
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

fn min_follow_gap() -> f32 {
    config::VEHICLE_LENGTH + config::SAFETY_GAP
}

/// Same-lane queue on the approach; same route once past the stop line.
fn shares_following_group(me: &Vehicle, other: &Vehicle) -> bool {
    if me.lane != other.lane {
        return false;
    }
    if me.route_progress < me.lane_length || other.route_progress < other.lane_length {
        return true;
    }
    me.route == other.route
}

fn leader_progress(me: &Vehicle, vehicles: &[Vehicle], self_index: usize) -> Option<f32> {
    let mut nearest: Option<f32> = None;
    for (i, other) in vehicles.iter().enumerate() {
        if i == self_index || other.finished {
            continue;
        }
        if !shares_following_group(me, other) {
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
    let max_advance = leader - me.route_progress - min_follow_gap();
    advance.min(max_advance.max(0.0))
}

fn cap_for_red_light(me: &Vehicle, signal: SignalState, advance: f32) -> f32 {
    if signal == SignalState::Green || me.route_progress >= me.lane_length {
        return advance;
    }
    let max_at_stop = me.lane_length;
    let allowed = max_at_stop - me.route_progress;
    advance.min(allowed.max(0.0))
}

/// True if another vehicle on this lane is too close to the spawn point.
pub fn lane_spawn_blocked(vehicles: &[Vehicle], lane: LaneId) -> bool {
    let min_gap = min_follow_gap();
    vehicles
        .iter()
        .filter(|v| v.lane == lane && !v.finished)
        .any(|v| v.route_progress < min_gap)
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
        let v = Vehicle::new(1, LaneId::SouthNb, RouteType::Straight, &world);
        let spawn = world.lane(LaneId::SouthNb).spawn;
        let pos = v.position();
        assert!((pos.x - spawn.x).abs() < 0.01);
        assert!((pos.y - spawn.y).abs() < 0.01);
    }

    #[test]
    fn red_light_holds_before_stop_line() {
        let world = World::new();
        let mut v = Vehicle::new(1, LaneId::SouthNb, RouteType::Straight, &world);
        let lane_length = world.lane(LaneId::SouthNb).lane_length;
        v.apply_advance(lane_length - 1.0);
        let vehicles = [v.clone()];
        let advance = v.compute_advance(1.0, SignalState::Red, &vehicles, 0);
        v.apply_advance(advance);
        assert!(v.route_progress() <= lane_length + 0.01);
    }

    #[test]
    fn follower_stops_behind_leader() {
        let world = World::new();
        let mut leader = Vehicle::new(1, LaneId::NorthSb, RouteType::Straight, &world);
        let mut follower = Vehicle::new(2, LaneId::NorthSb, RouteType::Straight, &world);
        let gap = min_follow_gap();
        leader.apply_advance(gap + 20.0);
        let vehicles = vec![leader.clone(), follower.clone()];
        let advance = follower.compute_advance(1.0, SignalState::Green, &vehicles, 1);
        follower.apply_advance(advance);
        assert!(follower.route_progress() <= gap + 0.01);
    }
}
