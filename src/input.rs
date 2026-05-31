use crate::simulation::Simulation;
use crate::types::{lane_for_spawn_direction, random_route_uniform, random_vehicle_kind};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Result of processing one frame's input events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    None,
    Quit,
}

pub struct InputHandler {
    spawn_seed: u32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self { spawn_seed: 0 }
    }

    pub fn handle_event(&mut self, sim: &mut Simulation, event: Event) -> InputAction {
        match event {
            Event::Quit { .. } => InputAction::Quit,
            Event::KeyDown {
                keycode: Some(key),
                repeat: false,
                ..
            } if key == Keycode::Escape => InputAction::Quit,
            Event::KeyDown {
                keycode: Some(key),
                repeat: false,
                ..
            } => {
                try_spawn_from_key(sim, key, &mut self.spawn_seed);
                InputAction::None
            }
            _ => InputAction::None,
        }
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

fn try_spawn_from_key(sim: &mut Simulation, key: Keycode, seed: &mut u32) {
    let lane = match key {
        Keycode::Up => Some(lane_for_spawn_direction(0)),
        Keycode::Down => Some(lane_for_spawn_direction(1)),
        Keycode::Right => Some(lane_for_spawn_direction(2)),
        Keycode::Left => Some(lane_for_spawn_direction(3)),
        Keycode::R => {
            *seed = seed.wrapping_add(1);
            Some(lane_for_spawn_direction(*seed as usize % 4))
        }
        _ => None,
    };

    let Some(lane) = lane else {
        return;
    };

    *seed = seed.wrapping_add(1);
    let route = random_route_uniform(*seed);
    *seed = seed.wrapping_add(1);
    let kind = random_vehicle_kind(*seed);
    let _ = sim.spawn_vehicle(lane, route, kind);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::Simulation;
    use crate::types::LaneId;

    #[test]
    fn arrow_keys_map_to_inbound_lanes() {
        assert_eq!(lane_for_key(Keycode::Up), Some(LaneId::SouthNb));
        assert_eq!(lane_for_key(Keycode::Down), Some(LaneId::NorthSb));
        assert_eq!(lane_for_key(Keycode::Right), Some(LaneId::WestEb));
        assert_eq!(lane_for_key(Keycode::Left), Some(LaneId::EastWb));
    }

    fn lane_for_key(key: Keycode) -> Option<LaneId> {
        match key {
            Keycode::Up => Some(lane_for_spawn_direction(0)),
            Keycode::Down => Some(lane_for_spawn_direction(1)),
            Keycode::Right => Some(lane_for_spawn_direction(2)),
            Keycode::Left => Some(lane_for_spawn_direction(3)),
            _ => None,
        }
    }

    #[test]
    fn spawn_from_key_adds_vehicle() {
        let mut sim = Simulation::new();
        let mut seed = 0u32;
        try_spawn_from_key(&mut sim, Keycode::Up, &mut seed);
        assert_eq!(sim.vehicles.len(), 1);
        assert_eq!(sim.vehicles[0].lane, LaneId::SouthNb);
    }

    #[test]
    fn anti_spam_blocks_back_to_back_spawn() {
        let mut sim = Simulation::new();
        let mut seed = 0u32;
        try_spawn_from_key(&mut sim, Keycode::Up, &mut seed);
        try_spawn_from_key(&mut sim, Keycode::Up, &mut seed);
        assert_eq!(sim.vehicles.len(), 1);
    }
}
