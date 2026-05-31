use crate::types::{LaneId, RouteType, SignalState, VehicleKind};
use crate::vehicle::{lane_spawn_blocked, queue_count_on_lane, Vehicle};
use crate::world::World;
use crate::config;

#[derive(Debug)]
pub struct Simulation {
    pub world: World,
    pub lane_signals: [SignalState; 8],
    pub vehicles: Vec<Vehicle>,
    next_vehicle_id: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            world: World::new(),
            lane_signals: [SignalState::Red; 8],
            vehicles: Vec::new(),
            next_vehicle_id: 1,
        }
    }

    pub fn set_lane_signal(&mut self, lane: LaneId, state: SignalState) {
        self.lane_signals[lane.index()] = state;
    }

    pub fn lane_signal(&self, lane: LaneId) -> SignalState {
        self.lane_signals[lane.index()]
    }

    pub fn is_green(&self, lane: LaneId) -> bool {
        self.lane_signal(lane) == SignalState::Green
    }

    pub fn spawn_vehicle(&mut self, lane: LaneId, route: RouteType, kind: VehicleKind) -> bool {
        if !self.world.lane(lane).inbound {
            return false;
        }
        if self.vehicles.len() >= config::MAX_VEHICLES {
            return false;
        }
        let spawn = self.world.lane(lane).spawn;
        if lane_spawn_blocked(&self.vehicles, lane, spawn) {
            return false;
        }

        let id = self.next_vehicle_id;
        self.next_vehicle_id += 1;
        self.vehicles
            .push(Vehicle::new(id, lane, route, kind, &self.world));
        true
    }

    pub fn update_vehicles(&mut self, dt: f32) {
        let signals = self.lane_signals;
        let advances: Vec<f32> = (0..self.vehicles.len())
            .map(|i| {
                let lane = self.vehicles[i].lane;
                let signal = signals[lane.index()];
                self.vehicles[i].compute_advance(dt, signal, &self.vehicles, i)
            })
            .collect();

        for (i, advance) in advances.into_iter().enumerate() {
            self.vehicles[i].apply_advance(advance);
        }

        self.vehicles.retain(|v| !v.is_finished());
    }
}

pub fn get_lane_queue_count(sim: &Simulation, lane: LaneId) -> i32 {
    let lane_length = sim.world.lane(lane).lane_length;
    queue_count_on_lane(&sim.vehicles, lane, lane_length)
}
