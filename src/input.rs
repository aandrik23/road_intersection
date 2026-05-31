use crate::audio::SoundEngine;
use crate::simulation::Simulation;
use crate::types::{
    lane_for_spawn_direction, random_route_uniform, random_vehicle_kind, RouteType, VehicleKind,
};
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

    pub fn handle_event(
        &mut self,
        sim: &mut Simulation,
        audio: &mut SoundEngine,
        event: Event,
    ) -> InputAction {
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
            } if key == Keycode::M => {
                audio.toggle_mute();
                InputAction::None
            }
            Event::KeyDown {
                keycode: Some(key),
                repeat: false,
                ..
            } => {
                try_spawn_from_key(sim, audio, key, &mut self.spawn_seed);
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

fn try_spawn_from_key(
    sim: &mut Simulation,
    audio: &mut SoundEngine,
    key: Keycode,
    seed: &mut u32,
) {
    let (lane, route, kind) = match key {
        Keycode::Up => (
            lane_for_spawn_direction(0),
            next_route(seed),
            next_vehicle_kind(seed),
        ),
        Keycode::Down => (
            lane_for_spawn_direction(1),
            next_route(seed),
            next_vehicle_kind(seed),
        ),
        Keycode::Right => (
            lane_for_spawn_direction(2),
            next_route(seed),
            next_vehicle_kind(seed),
        ),
        Keycode::Left => (
            lane_for_spawn_direction(3),
            next_route(seed),
            next_vehicle_kind(seed),
        ),
        Keycode::R => {
            let roll = mix_spawn_seed(*seed);
            *seed = seed.wrapping_add(1);
            (
                lane_for_spawn_direction(roll as usize % 4),
                random_route_uniform(roll.wrapping_shr(8)),
                random_vehicle_kind(roll.wrapping_shr(16)),
            )
        }
        _ => return,
    };

    if sim.spawn_vehicle(lane, route, kind) {
        audio.play_spawn();
    } else {
        audio.play_spawn_blocked();
    }
}

fn next_route(seed: &mut u32) -> RouteType {
    *seed = seed.wrapping_add(1);
    random_route_uniform(*seed)
}

fn next_vehicle_kind(seed: &mut u32) -> VehicleKind {
    *seed = seed.wrapping_add(1);
    random_vehicle_kind(*seed)
}

/// Scramble the spawn counter so lane / route / kind rolls stay independent per key press.
fn mix_spawn_seed(seed: u32) -> u32 {
    let mut x = seed;
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
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
        let mut audio = SoundEngine::silent();
        try_spawn_from_key(&mut sim, &mut audio, Keycode::Up, &mut seed);
        assert_eq!(sim.vehicles.len(), 1);
        assert_eq!(sim.vehicles[0].lane, LaneId::SouthNb);
    }

    #[test]
    fn anti_spam_blocks_back_to_back_spawn() {
        let mut sim = Simulation::new();
        let mut seed = 0u32;
        let mut audio = SoundEngine::silent();
        try_spawn_from_key(&mut sim, &mut audio, Keycode::Up, &mut seed);
        try_spawn_from_key(&mut sim, &mut audio, Keycode::Up, &mut seed);
        assert_eq!(sim.vehicles.len(), 1);
    }

    #[test]
    fn r_key_randomizes_direction_and_route() {
        use crate::types::RouteType;

        let mut seed = 0u32;
        let mut lanes = Vec::new();
        let mut routes = Vec::new();

        for _ in 0..12 {
            let roll = mix_spawn_seed(seed);
            seed = seed.wrapping_add(1);
            lanes.push(lane_for_spawn_direction(roll as usize % 4));
            routes.push(random_route_uniform(roll.wrapping_shr(8)));
        }

        assert!(lanes.iter().copied().collect::<std::collections::HashSet<_>>().len() > 1);
        assert!(routes.contains(&RouteType::Left));
        assert!(routes.contains(&RouteType::Right));
        assert!(routes.contains(&RouteType::Straight));
    }
}
