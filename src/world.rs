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
    let arm = config::ARM_LENGTH;
    let ix0 = config::IX0;
    let ix1 = config::IX1;
    let iy0 = config::IY0;
    let iy1 = config::IY1;
    let er_nb = config::ENTER_NB_X;
    let er_sb = config::ENTER_SB_X;
    let er_eb = config::ENTER_EB_Y;
    let er_wb = config::ENTER_WB_Y;
    let ex_nb = config::EXIT_NB_X;
    let ex_sb = config::EXIT_SB_X;
    let ex_eb = config::EXIT_EB_Y;
    let ex_wb = config::EXIT_WB_Y;

    // Inbound lanes (spec diagram enter arrows).
    setup_lane(
        &mut world.lanes,
        LaneId::NorthSb,
        Vec2 {
            x: er_sb,
            y: iy0 - arm,
        },
        Vec2 { x: er_sb, y: iy0 },
        Vec2 {
            x: er_sb - config::TRAFFIC_LIGHT_SIDE_OFFSET,
            y: iy0 - config::TRAFFIC_LIGHT_STOP_OFFSET,
        },
        90.0,
        true,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::SouthNb,
        Vec2 {
            x: er_nb,
            y: iy1 + arm,
        },
        Vec2 { x: er_nb, y: iy1 },
        Vec2 {
            x: er_nb + config::TRAFFIC_LIGHT_SIDE_OFFSET,
            y: iy1 + config::TRAFFIC_LIGHT_STOP_OFFSET,
        },
        270.0,
        true,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::WestEb,
        Vec2 {
            x: ix0 - arm,
            y: er_eb,
        },
        Vec2 { x: ix0, y: er_eb },
        Vec2 {
            x: ix0 - config::TRAFFIC_LIGHT_STOP_OFFSET,
            y: er_eb + config::TRAFFIC_LIGHT_SIDE_OFFSET,
        },
        0.0,
        true,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::EastWb,
        Vec2 {
            x: ix1 + arm,
            y: er_wb,
        },
        Vec2 { x: ix1, y: er_wb },
        Vec2 {
            x: ix1 + config::TRAFFIC_LIGHT_STOP_OFFSET,
            y: er_wb - config::TRAFFIC_LIGHT_SIDE_OFFSET,
        },
        180.0,
        true,
    );

    // Outbound lanes (spec diagram departures on the opposite track).
    setup_lane(
        &mut world.lanes,
        LaneId::NorthNb,
        Vec2 {
            x: ex_nb,
            y: iy0 - arm,
        },
        Vec2 { x: ex_nb, y: iy0 },
        Vec2 {
            x: ex_nb,
            y: iy0 - 18.0,
        },
        270.0,
        false,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::SouthSb,
        Vec2 {
            x: ex_sb,
            y: iy1 + arm,
        },
        Vec2 { x: ex_sb, y: iy1 },
        Vec2 {
            x: ex_sb,
            y: iy1 + 18.0,
        },
        90.0,
        false,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::EastEb,
        Vec2 {
            x: ix1 + arm,
            y: ex_eb,
        },
        Vec2 { x: ix1, y: ex_eb },
        Vec2 {
            x: ix1 + 18.0,
            y: ex_eb,
        },
        0.0,
        false,
    );
    setup_lane(
        &mut world.lanes,
        LaneId::WestWb,
        Vec2 {
            x: ix0 - arm,
            y: ex_wb,
        },
        Vec2 { x: ix0, y: ex_wb },
        Vec2 {
            x: ix0 - 18.0,
            y: ex_wb,
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
    let arm = config::ARM_LENGTH;
    let cx = config::CENTER_X;
    let cy = config::CENTER_Y;
    let ix0 = config::IX0;
    let ix1 = config::IX1;
    let iy0 = config::IY0;
    let iy1 = config::IY1;
    let er_nb = config::ENTER_NB_X;
    let er_sb = config::ENTER_SB_X;
    let er_eb = config::ENTER_EB_Y;
    let er_wb = config::ENTER_WB_Y;
    let ex_nb = config::EXIT_NB_X;
    let ex_sb = config::EXIT_SB_X;
    let ex_eb = config::EXIT_EB_Y;
    let ex_wb = config::EXIT_WB_Y;
    let hub = Vec2 { x: cx, y: cy };

    let mut straight = PathBuilder::new(lane, RouteType::Straight);
    let mut left = PathBuilder::new(lane, RouteType::Left);
    let mut right = PathBuilder::new(lane, RouteType::Right);

    match lane {
        // From north, southbound ↓ on west lane.
        LaneId::NorthSb => {
            let stop = Vec2 { x: er_sb, y: iy0 };
            straight.push(stop);
            straight.push(Vec2 { x: er_sb, y: iy1 });
            straight.push(Vec2 {
                x: ex_sb,
                y: iy1 + arm,
            });
            // Left → east: into the box, through center, then east.
            left.push(stop);
            left.push(Vec2 { x: er_sb, y: cy });
            left.push(hub);
            left.push(Vec2 { x: cx, y: er_eb });
            left.push(Vec2 {
                x: ix1 + arm,
                y: ex_eb,
            });
            // Right → west: through center, then west.
            right.push(stop);
            right.push(Vec2 { x: er_sb, y: cy });
            right.push(hub);
            right.push(Vec2 { x: ix0, y: ex_wb });
            right.push(Vec2 {
                x: ix0 - arm,
                y: ex_wb,
            });
        }
        // From south, northbound ↑ on east lane.
        LaneId::SouthNb => {
            let stop = Vec2 { x: er_nb, y: iy1 };
            straight.push(stop);
            straight.push(Vec2 { x: er_nb, y: iy0 });
            straight.push(Vec2 {
                x: ex_nb,
                y: iy0 - arm,
            });
            left.push(stop);
            left.push(Vec2 { x: er_nb, y: cy });
            left.push(hub);
            left.push(Vec2 { x: ix0, y: ex_wb });
            left.push(Vec2 {
                x: ix0 - arm,
                y: ex_wb,
            });
            right.push(stop);
            right.push(Vec2 { x: er_nb, y: cy });
            right.push(hub);
            right.push(Vec2 { x: ix1, y: ex_eb });
            right.push(Vec2 {
                x: ix1 + arm,
                y: ex_eb,
            });
        }
        // From west, eastbound → on south row.
        LaneId::WestEb => {
            let stop = Vec2 { x: ix0, y: er_eb };
            straight.push(stop);
            straight.push(Vec2 { x: ix1, y: er_eb });
            straight.push(Vec2 {
                x: ix1 + arm,
                y: ex_eb,
            });
            left.push(stop);
            left.push(Vec2 { x: cx, y: er_eb });
            left.push(hub);
            left.push(Vec2 { x: er_nb, y: cy });
            left.push(Vec2 {
                x: ex_nb,
                y: iy0 - arm,
            });
            right.push(stop);
            right.push(Vec2 { x: cx, y: er_eb });
            right.push(hub);
            right.push(Vec2 { x: er_sb, y: cy });
            right.push(Vec2 {
                x: ex_sb,
                y: iy1 + arm,
            });
        }
        // From east, westbound ← on north row.
        LaneId::EastWb => {
            let stop = Vec2 { x: ix1, y: er_wb };
            straight.push(stop);
            straight.push(Vec2 { x: ix0, y: er_wb });
            straight.push(Vec2 {
                x: ix0 - arm,
                y: ex_wb,
            });
            left.push(stop);
            left.push(Vec2 { x: cx, y: er_wb });
            left.push(hub);
            left.push(Vec2 { x: er_sb, y: cy });
            left.push(Vec2 {
                x: ex_sb,
                y: iy1 + arm,
            });
            right.push(stop);
            right.push(Vec2 { x: cx, y: er_wb });
            right.push(hub);
            right.push(Vec2 { x: er_nb, y: cy });
            right.push(Vec2 {
                x: ex_nb,
                y: iy0 - arm,
            });
        }
        LaneId::NorthNb => {
            straight.push(Vec2 {
                x: ex_nb,
                y: iy0 - arm * 0.35,
            });
            straight.push(Vec2 {
                x: ex_nb,
                y: iy0 - arm,
            });
        }
        LaneId::SouthSb => {
            straight.push(Vec2 {
                x: ex_sb,
                y: iy1 + arm * 0.35,
            });
            straight.push(Vec2 {
                x: ex_sb,
                y: iy1 + arm,
            });
        }
        LaneId::EastEb => {
            straight.push(Vec2 {
                x: ix1 + arm * 0.35,
                y: ex_eb,
            });
            straight.push(Vec2 {
                x: ix1 + arm,
                y: ex_eb,
            });
        }
        LaneId::WestWb => {
            straight.push(Vec2 {
                x: ix0 - arm * 0.35,
                y: ex_wb,
            });
            straight.push(Vec2 {
                x: ix0 - arm,
                y: ex_wb,
            });
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
