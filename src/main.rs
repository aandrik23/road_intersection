use road_intersection::render::AppRenderer;
use road_intersection::simulation::Simulation;
use road_intersection::world;
use road_intersection::traffic_lights::{print_traffic_light_summary, TrafficLightController};
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;


fn main() -> Result<(), String> {
    let mut sim = Simulation::new();
    let mut traffic_lights = TrafficLightController::new();

    world::print_lane_table(&sim.world);
    print_traffic_light_summary();

    let mut last_frame = Instant::now();

    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video
        .window(
            "Road Intersection",
            road_intersection::config::WINDOW_WIDTH,
            road_intersection::config::WINDOW_HEIGHT,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let mut app = AppRenderer::new(canvas);
    let mut event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        traffic_lights.update(&mut sim, dt);

        app.draw_frame(&sim)?;
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
