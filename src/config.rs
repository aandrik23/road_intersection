/// Shared simulation constants (pixels).
pub const WINDOW_WIDTH: u32 = 1152;
pub const WINDOW_HEIGHT: u32 = 864;
pub const HUD_HEIGHT: u32 = 104;

pub const CENTER_X: f32 = WINDOW_WIDTH as f32 * 0.5;
pub const CENTER_Y: f32 = (WINDOW_HEIGHT - HUD_HEIGHT) as f32 * 0.5 + 8.0;

pub const VEHICLE_LENGTH: f32 = 22.0;
pub const MOTORCYCLE_LENGTH: f32 = 15.5;
pub const SAFETY_GAP: f32 = 16.0;
/// Extra bumper space when queued on the approach (before the stop line).
pub const WAITING_EXTRA_GAP: f32 = 14.0;
/// Multiplier on follow gap — minimum space before another car may spawn on the lane.
pub const SPAWN_QUEUE_FACTOR: f32 = 1.75;
/// Small buffer when crossing perpendicular traffic in the box.
pub const INTERSECTION_EXTRA_GAP: f32 = 6.0;
pub const VEHICLE_SPEED: f32 = 90.0;
/// Visual scale for vehicle sprites (collision uses full `VEHICLE_LENGTH`).
pub const VEHICLE_DRAW_SCALE: f32 = 1.22;
/// Extra width for car sprites only (narrow axis).
pub const CAR_DRAW_WIDTH_SCALE: f32 = 1.16;
/// Extra scale for motorcycle sprites (length and width).
pub const MOTORCYCLE_DRAW_SCALE: f32 = 1.38;
/// On-screen scale for traffic-light SVGs (texture is rasterized larger for sharpness).
pub const SIGNAL_DRAW_SCALE: f32 = 0.68;

pub const INTERSECTION_HALF: f32 = 102.0;
pub const LANE_WIDTH: f32 = 46.0;
/// How far into the box (0–1) before a left turn begins (drive straight first).
pub const TURN_LEFT_DEPTH: f32 = 0.66;
/// Shallower entry for right turns.
pub const TURN_RIGHT_DEPTH: f32 = 0.36;
pub const ARM_LENGTH: f32 = 290.0;

/// How far traffic lights sit beside the lane (not on the vehicle path).
pub const TRAFFIC_LIGHT_SIDE_OFFSET: f32 = 28.0;
pub const TRAFFIC_LIGHT_STOP_OFFSET: f32 = 14.0;
/// Crosswalk stripe band depth (must match `draw_crosswalks` in render.rs).
pub const CROSSWALK_DEPTH: f32 = 10.0;
/// Extra space between crosswalk and the front bumper when waiting on red.
pub const RED_LIGHT_STOP_MARGIN: f32 = 10.0;

/// Path-progress hold point on red: keeps the drawn front bumper clear of crosswalks.
pub fn red_light_hold_back() -> f32 {
    let front_bumper = VEHICLE_LENGTH * VEHICLE_DRAW_SCALE * 0.5;
    CROSSWALK_DEPTH + front_bumper + RED_LIGHT_STOP_MARGIN
}

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

/// Road / scenery layout (pixels) — shared by render.
pub const ROAD_MARGIN_X: f32 = 95.0;
pub const ROAD_MARGIN_Y: f32 = 55.0;
pub const CURB_MARGIN_X: f32 = 72.0;
pub const CURB_MARGIN_Y: f32 = 48.0;
pub const ROAD_HALF_WIDTH: f32 = LANE_WIDTH + 10.0;
pub const SIDEWALK_WIDTH: f32 = 10.0;
