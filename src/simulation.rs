use crate::types::{LaneId, SignalState};
use crate::world::World;

#[derive(Debug, Clone)]
pub struct VehicleDraw {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: crate::types::ColorRgb,
}

#[derive(Debug)]
pub struct Simulation {
    pub world: World,
    pub lane_signals: [SignalState; 8],
    pub vehicles: Vec<VehicleDraw>,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            world: World::new(),
            lane_signals: [SignalState::Red; 8],
            vehicles: Vec::new(),
        }
    }

    pub fn set_lane_signal(&mut self, lane: LaneId, state: SignalState) {
        self.lane_signals[lane.index()] = state;
    }

    pub fn lane_signal(&self, lane: LaneId) -> SignalState {
        self.lane_signals[lane.index()]
    }
}

/// Person 2 hook — stub until Person 3 counts queues.
pub fn get_lane_queue_count(_sim: &Simulation, _lane: LaneId) -> i32 {
    0
}
