use crate::config;
use crate::simulation::{get_lane_queue_count, Simulation};
use crate::types::{lane_capacity, LaneId, SignalState};

const MIN_GREEN_SECONDS: f32 = 3.0;
const NORMAL_MAX_GREEN_SECONDS: f32 = 8.0;
const EXTENDED_MAX_GREEN_SECONDS: f32 = 14.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficPhase {
    NorthSouth,
    EastWest,
}

#[derive(Debug)]
pub struct TrafficLightController {
    phase: TrafficPhase,
    elapsed_in_phase: f32,
}

impl TrafficLightController {
    pub fn new() -> Self {
        Self {
            phase: TrafficPhase::NorthSouth,
            elapsed_in_phase: 0.0,
        }
    }

    pub fn update(&mut self, sim: &mut Simulation, dt_seconds: f32) {
        self.elapsed_in_phase += dt_seconds;

        if self.should_switch_phase(sim) {
            self.switch_phase();
        }

        self.apply_phase_to_simulation(sim);
    }

    fn should_switch_phase(&self, sim: &Simulation) -> bool {
        if self.elapsed_in_phase < MIN_GREEN_SECONDS {
            return false;
        }

        let current_is_full = self.phase_has_full_lane(sim, self.phase);
        let opposite_is_full = self.phase_has_full_lane(sim, self.opposite_phase());

        if opposite_is_full {
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
        self.phase = self.opposite_phase();
        self.elapsed_in_phase = 0.0;
    }

    fn opposite_phase(&self) -> TrafficPhase {
        match self.phase {
            TrafficPhase::NorthSouth => TrafficPhase::EastWest,
            TrafficPhase::EastWest => TrafficPhase::NorthSouth,
        }
    }

    fn apply_phase_to_simulation(&self, sim: &mut Simulation) {
        for lane in LaneId::ALL {
            sim.set_lane_signal(lane, SignalState::Red);
        }

        for lane in Self::lanes_for_phase(self.phase) {
            sim.set_lane_signal(lane, SignalState::Green);
        }
    }

    fn lanes_for_phase(phase: TrafficPhase) -> [LaneId; 2] {
        match phase {
            TrafficPhase::NorthSouth => [LaneId::NorthSb, LaneId::SouthNb],
            TrafficPhase::EastWest => [LaneId::EastWb, LaneId::WestEb],
        }
    }

    fn phase_has_full_lane(&self, sim: &Simulation, phase: TrafficPhase) -> bool {
        for lane in Self::lanes_for_phase(phase) {
            let lane_info = sim.world.lane(lane);
            let capacity = lane_capacity(lane_info.lane_length);
            let queue_count = get_lane_queue_count(sim, lane);

            if capacity > 0 && queue_count >= capacity {
                return true;
            }
        }

        false
    }
}

impl Default for TrafficLightController {
    fn default() -> Self {
        Self::new()
    }
}

pub fn print_traffic_light_summary() {
    println!("\n--- Person 2 traffic-light controller ---");
    println!("Phases:");
    println!("  NorthSouth: NorthSb + SouthNb green");
    println!("  EastWest:   EastWb + WestEb green");
    println!("Timing:");
    println!("  min green: {MIN_GREEN_SECONDS}s");
    println!("  normal max green: {NORMAL_MAX_GREEN_SECONDS}s");
    println!("  extended max green under congestion: {EXTENDED_MAX_GREEN_SECONDS}s");
    println!(
        "Capacity formula: floor(lane_length / (vehicle_length + safety_gap)) = floor(lane_length / {})\n",
        config::VEHICLE_LENGTH + config::SAFETY_GAP
    );
}