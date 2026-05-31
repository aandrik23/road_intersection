use crate::config;
use crate::simulation::{get_lane_queue_count, Simulation};
use crate::types::{lane_capacity, LaneId, SignalState};

const MIN_GREEN_SECONDS: f32 = 3.0;
const NORMAL_MAX_GREEN_SECONDS: f32 = 8.0;
const EXTENDED_MAX_GREEN_SECONDS: f32 = 14.0;

/// One inbound approach green at a time — any phase order is allowed by the spec;
/// sequential greens avoid path crossings inside the box (e.g. opposing left turns).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficPhase {
    NorthSb,
    SouthNb,
    WestEb,
    EastWb,
}

impl TrafficPhase {
    const ALL: [TrafficPhase; 4] = [
        TrafficPhase::NorthSb,
        TrafficPhase::SouthNb,
        TrafficPhase::WestEb,
        TrafficPhase::EastWb,
    ];

    fn lane(self) -> LaneId {
        match self {
            TrafficPhase::NorthSb => LaneId::NorthSb,
            TrafficPhase::SouthNb => LaneId::SouthNb,
            TrafficPhase::WestEb => LaneId::WestEb,
            TrafficPhase::EastWb => LaneId::EastWb,
        }
    }

    fn next(self) -> TrafficPhase {
        match self {
            TrafficPhase::NorthSb => TrafficPhase::SouthNb,
            TrafficPhase::SouthNb => TrafficPhase::WestEb,
            TrafficPhase::WestEb => TrafficPhase::EastWb,
            TrafficPhase::EastWb => TrafficPhase::NorthSb,
        }
    }
}

#[derive(Debug)]
pub struct TrafficLightController {
    phase: TrafficPhase,
    elapsed_in_phase: f32,
}

impl TrafficLightController {
    pub fn new() -> Self {
        Self {
            phase: TrafficPhase::NorthSb,
            elapsed_in_phase: 0.0,
        }
    }

    /// Returns `true` when the active green phase just changed (play signal SFX).
    pub fn update(&mut self, sim: &mut Simulation, dt_seconds: f32) -> bool {
        self.elapsed_in_phase += dt_seconds;

        let switched = if self.should_switch_phase(sim) {
            self.switch_phase();
            true
        } else {
            false
        };

        self.apply_phase_to_simulation(sim);
        switched
    }

    fn should_switch_phase(&self, sim: &Simulation) -> bool {
        if self.elapsed_in_phase < MIN_GREEN_SECONDS {
            return false;
        }

        let current_lane = self.phase.lane();
        let current_is_full = lane_is_at_capacity(sim, current_lane);
        let other_waiting_full = TrafficPhase::ALL
            .iter()
            .filter(|p| **p != self.phase)
            .any(|p| lane_is_at_capacity(sim, p.lane()));

        // Yield the green to a congested approach that is still waiting.
        if other_waiting_full && !current_is_full {
            return true;
        }

        if self.elapsed_in_phase >= EXTENDED_MAX_GREEN_SECONDS {
            return true;
        }

        if self.elapsed_in_phase >= NORMAL_MAX_GREEN_SECONDS && !current_is_full {
            return true;
        }

        false
    }

    fn switch_phase(&mut self) {
        self.phase = self.phase.next();
        self.elapsed_in_phase = 0.0;
    }

    fn apply_phase_to_simulation(&self, sim: &mut Simulation) {
        for lane in LaneId::ALL {
            sim.set_lane_signal(lane, SignalState::Red);
        }
        sim.set_lane_signal(self.phase.lane(), SignalState::Green);
    }
}

fn lane_is_at_capacity(sim: &Simulation, lane: LaneId) -> bool {
    let lane_info = sim.world.lane(lane);
    let capacity = lane_capacity(lane_info.lane_length);
    let queue_count = get_lane_queue_count(sim, lane);
    capacity > 0 && queue_count >= capacity
}

impl Default for TrafficLightController {
    fn default() -> Self {
        Self::new()
    }
}

pub fn print_traffic_light_summary() {
    println!("\n--- Person 2 traffic-light controller ---");
    println!("Phases (one inbound lane green at a time):");
    println!("  NorthSb → SouthNb → WestEb → EastWb → …");
    println!("Timing:");
    println!("  min green: {MIN_GREEN_SECONDS}s");
    println!("  normal max green: {NORMAL_MAX_GREEN_SECONDS}s");
    println!("  extended max green under congestion: {EXTENDED_MAX_GREEN_SECONDS}s");
    println!(
        "Capacity formula: floor(lane_length / (vehicle_length + safety_gap)) = floor(lane_length / {})\n",
        config::VEHICLE_LENGTH + config::SAFETY_GAP
    );
}