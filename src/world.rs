use crate::config;
use crate::types::{LaneId, RouteType, Vec2};

#[derive(Debug, Clone)]
pub struct LaneInfo {
    pub id: LaneId,
    pub spawn: Vec2,
    pub stop_line: Vec2,
    pub light_pos: Vec2,
    pub heading: f32,
    pub lane_length: f32,
    pub inbound: bool,
}

#[derive(Debug, Clone)]
pub struct RoutePath {
    pub lane: LaneId,
    pub route: RouteType,
    pub waypoints: Vec<Vec2>,
    pub path_length: f32,
}

#[derive(Debug, Clone)]
pub struct World {
    pub lanes: [LaneInfo; 8],
    pub routes: [[RoutePath; 3]; 8],
}

impl World {
    pub fn new() -> Self {
        let mut world = World {
            lanes: std::array::from_fn(|_| LaneInfo {
                id: LaneId::NorthSb,
                spawn: Vec2 { x: 0.0, y: 0.0 },
                stop_line: Vec2 { x: 0.0, y: 0.0 },
                light_pos: Vec2 { x: 0.0, y: 0.0 },
                heading: 0.0,
                lane_length: 0.0,
                inbound: false,
            }),
            routes: std::array::from_fn(|_| {
                std::array::from_fn(|_| RoutePath {
                    lane: LaneId::NorthSb,
                    route: RouteType::Straight,
                    waypoints: Vec::new(),
                    path_length: 0.0,
                })
            }),
        };
        init_lanes(&mut world);
        for lane in LaneId::ALL {
            build_paths_for_lane(&mut world, lane);
        }
        world
    }

    pub fn lane(&self, id: LaneId) -> &LaneInfo {
        &self.lanes[lane_index(id)]
    }

    pub fn route(&self, lane: LaneId, route: RouteType) -> &RoutePath {
        &self.routes[lane_index(lane)][route_index(route)]
    }
}

fn lane_index(id: LaneId) -> usize {
    match id {
        LaneId::NorthSb => 0,
        LaneId::NorthNb => 1,
        LaneId::SouthNb => 2,
        LaneId::SouthSb => 3,
        LaneId::EastWb => 4,
        LaneId::EastEb => 5,
        LaneId::WestEb => 6,
        LaneId::WestWb => 7,
    }
}

fn route_index(route: RouteType) -> usize {
    match route {
        RouteType::Left => 0,
        RouteType::Right => 1,
        RouteType::Straight => 2,
    }
}

fn dist(a: Vec2, b: Vec2) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

fn setup_lane(
    lanes: &mut [LaneInfo; 8],
    id: LaneId,
    spawn: Vec2,
    stop: Vec2,
    light: Vec2,
    heading: f32,
    inbound: bool,
) {
    let lane = &mut lanes[lane_index(id)];
    lane.id = id;
    lane.spawn = spawn;
    lane.stop_line = stop;
    lane.light_pos = light;
    lane.heading = heading;
    lane.inbound = inbound;
    lane.lane_length = dist(spawn, stop);
}

fn init_lanes(world: &mut World) {
    let cx = config::CENTER_X;
    let cy = config::CENTER_Y;
    let h = config::INTERSECTION_HALF;
    let lw = config::LANE_WIDTH;
    let arm = config::ARM_LENGTH;

    setup_lane(
        &mut world.lanes,
        LaneId::NorthSb,
        Vec2 {
            x: cx + lw * 0.5,
            y: cy - h - arm,
        },
        Vec2 {
            x: cx + lw * 0.5,
            y: cy - h,
        },
        Vec2 {
            x: cx + lw * 0.5,
            y: cy - h - 18.0,
        },
        90.0,
        true,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::NorthNb,
        Vec2 {
            x: cx - lw * 0.5,
            y: cy - h - arm,
        },
        Vec2 {
            x: cx - lw * 0.5,
            y: cy - h,
        },
        Vec2 {
            x: cx - lw * 0.5,
            y: cy - h - 18.0,
        },
        270.0,
        false,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::SouthNb,
        Vec2 {
            x: cx - lw * 0.5,
            y: cy + h + arm,
        },
        Vec2 {
            x: cx - lw * 0.5,
            y: cy + h,
        },
        Vec2 {
            x: cx - lw * 0.5,
            y: cy + h + 18.0,
        },
        270.0,
        true,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::SouthSb,
        Vec2 {
            x: cx + lw * 0.5,
            y: cy + h + arm,
        },
        Vec2 {
            x: cx + lw * 0.5,
            y: cy + h,
        },
        Vec2 {
            x: cx + lw * 0.5,
            y: cy + h + 18.0,
        },
        90.0,
        false,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::EastWb,
        Vec2 {
            x: cx + h + arm,
            y: cy - lw * 0.5,
        },
        Vec2 {
            x: cx + h,
            y: cy - lw * 0.5,
        },
        Vec2 {
            x: cx + h + 18.0,
            y: cy - lw * 0.5,
        },
        180.0,
        true,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::EastEb,
        Vec2 {
            x: cx + h + arm,
            y: cy + lw * 0.5,
        },
        Vec2 {
            x: cx + h,
            y: cy + lw * 0.5,
        },
        Vec2 {
            x: cx + h + 18.0,
            y: cy + lw * 0.5,
        },
        0.0,
        false,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::WestEb,
        Vec2 {
            x: cx - h - arm,
            y: cy + lw * 0.5,
        },
        Vec2 {
            x: cx - h,
            y: cy + lw * 0.5,
        },
        Vec2 {
            x: cx - h - 18.0,
            y: cy + lw * 0.5,
        },
        0.0,
        true,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::WestWb,
        Vec2 {
            x: cx - h - arm,
            y: cy - lw * 0.5,
        },
        Vec2 {
            x: cx - h,
            y: cy - lw * 0.5,
        },
        Vec2 {
            x: cx - h - 18.0,
            y: cy - lw * 0.5,
        },
        180.0,
        false,
    );
}

struct PathBuilder {
    lane: LaneId,
    route: RouteType,
    points: Vec<Vec2>,
    length: f32,
}

impl PathBuilder {
    fn new(lane: LaneId, route: RouteType) -> Self {
        Self {
            lane,
            route,
            points: Vec::new(),
            length: 0.0,
        }
    }

    fn push(&mut self, p: Vec2) {
        if let Some(last) = self.points.last() {
            self.length += dist(*last, p);
        }
        self.points.push(p);
    }

    fn finish(self) -> RoutePath {
        RoutePath {
            lane: self.lane,
            route: self.route,
            waypoints: self.points,
            path_length: self.length,
        }
    }
}

fn set_route(world: &mut World, lane: LaneId, route: RouteType, path: RoutePath) {
    world.routes[lane_index(lane)][route_index(route)] = path;
}

fn build_paths_for_lane(world: &mut World, lane: LaneId) {
    let cx = config::CENTER_X;
    let cy = config::CENTER_Y;
    let h = config::INTERSECTION_HALF;
    let lw = config::LANE_WIDTH;
    let arm = config::ARM_LENGTH;

    let n_sb_x = cx + lw * 0.5;
    let n_nb_x = cx - lw * 0.5;
    let s_nb_x = cx - lw * 0.5;
    let s_sb_x = cx + lw * 0.5;
    let e_wb_y = cy - lw * 0.5;
    let e_eb_y = cy + lw * 0.5;
    let w_eb_y = cy + lw * 0.5;
    let w_wb_y = cy - lw * 0.5;

    let mut straight = PathBuilder::new(lane, RouteType::Straight);
    let mut left = PathBuilder::new(lane, RouteType::Left);
    let mut right = PathBuilder::new(lane, RouteType::Right);

    match lane {
        LaneId::NorthSb => {
            straight.push(Vec2 { x: n_sb_x, y: cy - h });
            straight.push(Vec2 { x: n_sb_x, y: cy + h });
            straight.push(Vec2 { x: n_sb_x, y: cy + h + arm });
            left.push(Vec2 { x: n_sb_x, y: cy - h });
            left.push(Vec2 { x: cx + h, y: cy - h });
            left.push(Vec2 { x: cx + h, y: e_eb_y });
            left.push(Vec2 { x: cx + h + arm, y: e_eb_y });
            right.push(Vec2 { x: n_sb_x, y: cy - h });
            right.push(Vec2 { x: cx - h, y: cy - h });
            right.push(Vec2 { x: cx - h, y: w_wb_y });
            right.push(Vec2 { x: cx - h - arm, y: w_wb_y });
        }
        LaneId::SouthNb => {
            straight.push(Vec2 { x: s_nb_x, y: cy + h });
            straight.push(Vec2 { x: s_nb_x, y: cy - h });
            straight.push(Vec2 { x: s_nb_x, y: cy - h - arm });
            left.push(Vec2 { x: s_nb_x, y: cy + h });
            left.push(Vec2 { x: cx - h, y: cy + h });
            left.push(Vec2 { x: cx - h, y: w_wb_y });
            left.push(Vec2 { x: cx - h - arm, y: w_wb_y });
            right.push(Vec2 { x: s_nb_x, y: cy + h });
            right.push(Vec2 { x: cx + h, y: cy + h });
            right.push(Vec2 { x: cx + h, y: e_eb_y });
            right.push(Vec2 { x: cx + h + arm, y: e_eb_y });
        }
        LaneId::WestEb => {
            straight.push(Vec2 { x: cx - h, y: w_eb_y });
            straight.push(Vec2 { x: cx + h, y: w_eb_y });
            straight.push(Vec2 { x: cx + h + arm, y: w_eb_y });
            left.push(Vec2 { x: cx - h, y: w_eb_y });
            left.push(Vec2 { x: cx - h, y: cy - h });
            left.push(Vec2 { x: n_nb_x, y: cy - h });
            left.push(Vec2 { x: n_nb_x, y: cy - h - arm });
            right.push(Vec2 { x: cx - h, y: w_eb_y });
            right.push(Vec2 { x: cx - h, y: cy + h });
            right.push(Vec2 { x: s_sb_x, y: cy + h });
            right.push(Vec2 { x: s_sb_x, y: cy + h + arm });
        }
        LaneId::EastWb => {
            straight.push(Vec2 { x: cx + h, y: e_wb_y });
            straight.push(Vec2 { x: cx - h, y: e_wb_y });
            straight.push(Vec2 { x: cx - h - arm, y: e_wb_y });
            left.push(Vec2 { x: cx + h, y: e_wb_y });
            left.push(Vec2 { x: cx + h, y: cy + h });
            left.push(Vec2 { x: s_sb_x, y: cy + h });
            left.push(Vec2 { x: s_sb_x, y: cy + h + arm });
            right.push(Vec2 { x: cx + h, y: e_wb_y });
            right.push(Vec2 { x: cx + h, y: cy - h });
            right.push(Vec2 { x: n_nb_x, y: cy - h });
            right.push(Vec2 { x: n_nb_x, y: cy - h - arm });
        }
        LaneId::NorthNb => {
            straight.push(Vec2 {
                x: n_nb_x,
                y: cy - h - arm * 0.35,
            });
            straight.push(Vec2 {
                x: n_nb_x,
                y: cy - h - arm,
            });
            left.push(Vec2 { x: n_nb_x, y: cy - h });
            left.push(Vec2 { x: cx - h, y: cy - h });
            left.push(Vec2 { x: cx - h, y: w_wb_y });
            right.push(Vec2 { x: n_nb_x, y: cy - h });
            right.push(Vec2 { x: cx + h, y: cy - h });
            right.push(Vec2 { x: cx + h, y: e_eb_y });
        }
        LaneId::SouthSb => {
            straight.push(Vec2 {
                x: s_sb_x,
                y: cy + h + arm * 0.35,
            });
            straight.push(Vec2 {
                x: s_sb_x,
                y: cy + h + arm,
            });
            left.push(Vec2 { x: s_sb_x, y: cy + h });
            left.push(Vec2 { x: cx + h, y: cy + h });
            left.push(Vec2 { x: cx + h, y: e_eb_y });
            right.push(Vec2 { x: s_sb_x, y: cy + h });
            right.push(Vec2 { x: cx - h, y: cy + h });
            right.push(Vec2 { x: cx - h, y: w_wb_y });
        }
        LaneId::EastEb => {
            straight.push(Vec2 {
                x: cx + h + arm * 0.35,
                y: e_eb_y,
            });
            straight.push(Vec2 {
                x: cx + h + arm,
                y: e_eb_y,
            });
            left.push(Vec2 { x: cx + h, y: e_eb_y });
            left.push(Vec2 { x: cx + h, y: cy + h });
            left.push(Vec2 { x: s_sb_x, y: cy + h });
            right.push(Vec2 { x: cx + h, y: e_eb_y });
            right.push(Vec2 { x: cx + h, y: cy - h });
            right.push(Vec2 { x: n_nb_x, y: cy - h });
        }
        LaneId::WestWb => {
            straight.push(Vec2 {
                x: cx - h - arm * 0.35,
                y: w_wb_y,
            });
            straight.push(Vec2 {
                x: cx - h - arm,
                y: w_wb_y,
            });
            left.push(Vec2 { x: cx - h, y: w_wb_y });
            left.push(Vec2 { x: cx - h, y: cy - h });
            left.push(Vec2 { x: n_nb_x, y: cy - h });
            right.push(Vec2 { x: cx - h, y: w_wb_y });
            right.push(Vec2 { x: cx - h, y: cy + h });
            right.push(Vec2 { x: s_sb_x, y: cy + h });
        }
    }

    set_route(world, lane, RouteType::Straight, straight.finish());
    set_route(world, lane, RouteType::Left, left.finish());
    set_route(world, lane, RouteType::Right, right.finish());
}

pub fn print_lane_table(world: &World) {
    println!("\n--- Lane data (Person 1, Rust) ---");
    println!("capacity = floor(lane_length / (vehicle_length + safety_gap))");
    println!(
        "vehicle_length={} safety_gap={} slot={}\n",
        config::VEHICLE_LENGTH,
        config::SAFETY_GAP,
        config::VEHICLE_LENGTH + config::SAFETY_GAP
    );
    for id in LaneId::ALL {
        let lane = world.lane(id);
        println!(
            "{:<8} spawn({:6.0},{:6.0}) stop({:6.0},{:6.0}) len={:6.1} cap={} inbound={}",
            id.name(),
            lane.spawn.x,
            lane.spawn.y,
            lane.stop_line.x,
            lane.stop_line.y,
            lane.lane_length,
            crate::types::lane_capacity(lane.lane_length),
            lane.inbound as u8
        );
    }
    println!("\nRoute colors: LEFT=blue RIGHT=yellow STRAIGHT=green");
    println!("Controls (Person 3): arrows spawn, r random, Esc quit");
    println!("Queue hook get_lane_queue_count() — live when vehicles are queued\n");
}
