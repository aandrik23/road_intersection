use road_intersection::input::{InputAction, InputHandler};
use road_intersection::render::AppRenderer;
use road_intersection::simulation::Simulation;
use road_intersection::traffic_lights::{print_traffic_light_summary, TrafficLightController};
use road_intersection::world;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let mut sim = Simulation::new();
    let mut traffic_lights = TrafficLightController::new();
    let mut input = InputHandler::new();

    world::print_lane_table(&sim.world);
    print_traffic_light_summary();

    let mut last_frame = Instant::now();

    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video
        .window(
            "Traffic Intersection",
            road_intersection::config::WINDOW_WIDTH,
            road_intersection::config::WINDOW_HEIGHT,
        )
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let mut app = AppRenderer::new(canvas)?;
    let mut event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

    // Apply initial green phase before the first vehicle tick.
    traffic_lights.update(&mut sim, 0.0);

    'running: loop {
        for event in event_pump.poll_iter() {
            if input.handle_event(&mut sim, event) == InputAction::Quit {
                break 'running;
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        // input → lights → vehicles → draw
        traffic_lights.update(&mut sim, dt);
        sim.update_vehicles(dt);

        app.draw_frame(&sim)?;
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
