/// Shared simulation constants (pixels).
pub const WINDOW_WIDTH: u32 = 1024;
pub const WINDOW_HEIGHT: u32 = 768;
pub const HUD_HEIGHT: u32 = 88;

pub const CENTER_X: f32 = WINDOW_WIDTH as f32 * 0.5;
pub const CENTER_Y: f32 = (WINDOW_HEIGHT - HUD_HEIGHT) as f32 * 0.5 + 8.0;

pub const VEHICLE_LENGTH: f32 = 22.0;
pub const SAFETY_GAP: f32 = 10.0;
pub const VEHICLE_SPEED: f32 = 90.0;
/// Visual scale for car sprites (collision uses full `VEHICLE_LENGTH`).
pub const VEHICLE_DRAW_SCALE: f32 = 0.88;

pub const INTERSECTION_HALF: f32 = 84.0;
pub const LANE_WIDTH: f32 = 40.0;
/// How far into the box (0–1) before a left turn begins (drive straight first).
pub const TURN_LEFT_DEPTH: f32 = 0.58;
/// Shallower entry for right turns.
pub const TURN_RIGHT_DEPTH: f32 = 0.36;
/// When sweeping left, how much to pull toward the intersection center (0–1).
pub const TURN_CROSS_BLEND: f32 = 0.75;
pub const ARM_LENGTH: f32 = 260.0;

/// How far traffic lights sit beside the lane (not on the vehicle path).
pub const TRAFFIC_LIGHT_SIDE_OFFSET: f32 = 28.0;
pub const TRAFFIC_LIGHT_STOP_OFFSET: f32 = 14.0;

/// Intersection box edges (pixels).
pub const IX0: f32 = CENTER_X - INTERSECTION_HALF;
pub const IX1: f32 = CENTER_X + INTERSECTION_HALF;
pub const IY0: f32 = CENTER_Y - INTERSECTION_HALF;
pub const IY1: f32 = CENTER_Y + INTERSECTION_HALF;

/// Spec diagram (N at top): west column ↓ southbound, east column ↑ northbound.
pub const LANE_WEST_X: f32 = CENTER_X - LANE_WIDTH * 0.5;
pub const LANE_EAST_X: f32 = CENTER_X + LANE_WIDTH * 0.5;
/// East arm: north row ← westbound, south row → eastbound.
pub const LANE_NORTH_Y: f32 = CENTER_Y - LANE_WIDTH * 0.5;
pub const LANE_SOUTH_Y: f32 = CENTER_Y + LANE_WIDTH * 0.5;

/// Entering the intersection (per spec arrows).
pub const ENTER_NB_X: f32 = LANE_EAST_X; // from south, northbound ↑
pub const ENTER_SB_X: f32 = LANE_WEST_X; // from north, southbound ↓
pub const ENTER_EB_Y: f32 = LANE_SOUTH_Y; // from west, eastbound →
pub const ENTER_WB_Y: f32 = LANE_NORTH_Y; // from east, westbound ←

/// Leaving the intersection (opposite track on each arm).
pub const EXIT_NB_X: f32 = LANE_EAST_X; // depart north ↑
pub const EXIT_SB_X: f32 = LANE_WEST_X; // depart south ↓
pub const EXIT_EB_Y: f32 = LANE_SOUTH_Y; // depart east →
pub const EXIT_WB_Y: f32 = LANE_NORTH_Y; // depart west ←

pub const MAX_ROUTE_WAYPOINTS: usize = 16;
pub const MAX_VEHICLES: usize = 64;
