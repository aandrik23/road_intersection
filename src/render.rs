use crate::config;
use crate::simulation::Simulation;
use crate::types::{LaneId, RouteType, SignalState};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct AppRenderer {
    pub canvas: Canvas<Window>,
}

impl AppRenderer {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self { canvas }
    }

    pub fn draw_frame(&mut self, sim: &Simulation) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        draw_roads(&mut self.canvas);
        draw_route_preview(&mut self.canvas, sim);
        draw_intersection_box(&mut self.canvas);
        draw_stop_lines(&mut self.canvas, sim);
        draw_direction_arrows(&mut self.canvas);
        draw_cardinal_labels(&mut self.canvas);
        draw_traffic_lights(&mut self.canvas, sim);
        draw_vehicles(&mut self.canvas, sim);

        self.canvas.present();
        Ok(())
    }
}

fn set_color(canvas: &mut Canvas<Window>, r: u8, g: u8, b: u8) {
    canvas.set_draw_color(Color::RGB(r, g, b));
}

fn fill_rect(canvas: &mut Canvas<Window>, x: i32, y: i32, w: u32, h: u32) {
    let _ = canvas.fill_rect(Rect::new(x, y, w, h));
}

fn draw_roads(canvas: &mut Canvas<Window>) {
    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;
    let h = config::INTERSECTION_HALF as i32;
    let road_half = (config::LANE_WIDTH as i32) + 6;

    set_color(canvas, 45, 48, 55);
    fill_rect(
        canvas,
        0,
        0,
        config::WINDOW_WIDTH,
        config::WINDOW_HEIGHT,
    );

    set_color(canvas, 62, 66, 74);
    fill_rect(
        canvas,
        cx - road_half,
        0,
        (road_half * 2) as u32,
        config::WINDOW_HEIGHT,
    );
    fill_rect(
        canvas,
        0,
        cy - road_half,
        config::WINDOW_WIDTH,
        (road_half * 2) as u32,
    );

    set_color(canvas, 45, 48, 55);
    fill_rect(canvas, cx - h, cy - h, (h * 2) as u32, (h * 2) as u32);

    set_color(canvas, 90, 95, 105);
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(cx, 0),
        sdl2::rect::Point::new(cx, cy - h),
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(cx, cy + h),
        sdl2::rect::Point::new(cx, config::WINDOW_HEIGHT as i32),
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(0, cy),
        sdl2::rect::Point::new(cx - h, cy),
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(cx + h, cy),
        sdl2::rect::Point::new(config::WINDOW_WIDTH as i32, cy),
    );

    set_color(canvas, 40, 42, 48);
    fill_rect(canvas, cx - 2, 0, 4, config::WINDOW_HEIGHT);
    fill_rect(canvas, 0, cy - 2, config::WINDOW_WIDTH, 4);
}

fn draw_stop_lines(canvas: &mut Canvas<Window>, sim: &Simulation) {
    set_color(canvas, 240, 240, 240);
    for lane_id in LaneId::ALL {
        let lane = sim.world.lane(lane_id);
        let sx = lane.stop_line.x as i32;
        let sy = lane.stop_line.y as i32;
        if lane.heading == 90.0 || lane.heading == 270.0 {
            let _ = canvas.draw_line(
                sdl2::rect::Point::new(sx - 14, sy),
                sdl2::rect::Point::new(sx + 14, sy),
            );
        } else {
            let _ = canvas.draw_line(
                sdl2::rect::Point::new(sx, sy - 14),
                sdl2::rect::Point::new(sx, sy + 14),
            );
        }
    }
}

fn draw_route_preview(canvas: &mut Canvas<Window>, sim: &Simulation) {
    set_color(canvas, 80, 85, 95);
    for lane_id in LaneId::ALL {
        if !sim.world.lane(lane_id).inbound {
            continue;
        }
        let path = sim.world.route(lane_id, RouteType::Straight);
        for w in path.waypoints.windows(2) {
            let _ = canvas.draw_line(
                sdl2::rect::Point::new(w[0].x as i32, w[0].y as i32),
                sdl2::rect::Point::new(w[1].x as i32, w[1].y as i32),
            );
        }
    }
}

fn draw_intersection_box(canvas: &mut Canvas<Window>) {
    set_color(canvas, 75, 80, 90);
    let x = (config::CENTER_X - config::INTERSECTION_HALF) as i32;
    let y = (config::CENTER_Y - config::INTERSECTION_HALF) as i32;
    let s = (config::INTERSECTION_HALF * 2.0) as u32;
    let _ = canvas.draw_rect(Rect::new(x, y, s, s));
}

fn draw_traffic_light(canvas: &mut Canvas<Window>, x: i32, y: i32, state: SignalState) {
    set_color(canvas, 30, 30, 35);
    fill_rect(canvas, x - 8, y - 18, 16, 36);

    if state == SignalState::Green {
        set_color(canvas, 40, 200, 70);
    } else {
        set_color(canvas, 40, 40, 45);
    }
    fill_rect(canvas, x - 5, y + 2, 10, 10);

    if state == SignalState::Red {
        set_color(canvas, 220, 50, 50);
    } else {
        set_color(canvas, 55, 55, 60);
    }
    fill_rect(canvas, x - 5, y - 14, 10, 10);
}

fn draw_traffic_lights(canvas: &mut Canvas<Window>, sim: &Simulation) {
    for lane_id in LaneId::ALL {
        let lane = sim.world.lane(lane_id);
        let state = sim.lane_signal(lane_id);
        draw_traffic_light(
            canvas,
            lane.light_pos.x as i32,
            lane.light_pos.y as i32,
            state,
        );
    }
}

fn draw_arrow(canvas: &mut Canvas<Window>, x: i32, y: i32, dx: i32, dy: i32) {
    set_color(canvas, 230, 230, 230);
    let tip_x = x + dx;
    let tip_y = y + dy;
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(x, y),
        sdl2::rect::Point::new(tip_x, tip_y),
    );
    if dy > 0 {
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x - 5, tip_y - 8),
        );
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x + 5, tip_y - 8),
        );
    } else if dy < 0 {
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x - 5, tip_y + 8),
        );
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x + 5, tip_y + 8),
        );
    } else if dx > 0 {
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x - 8, tip_y - 5),
        );
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x - 8, tip_y + 5),
        );
    } else if dx < 0 {
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x + 8, tip_y - 5),
        );
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x + 8, tip_y + 5),
        );
    }
}

fn draw_direction_arrows(canvas: &mut Canvas<Window>) {
    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;
    let h = config::INTERSECTION_HALF as i32;
    let lw = config::LANE_WIDTH as i32;

    draw_arrow(canvas, cx + lw / 2, 70, 0, 40);
    draw_arrow(canvas, cx - lw / 2, config::WINDOW_HEIGHT as i32 - 70, 0, -40);
    draw_arrow(canvas, 70, cy - lw / 2, 40, 0);
    draw_arrow(
        canvas,
        config::WINDOW_WIDTH as i32 - 70,
        cy + lw / 2,
        -40,
        0,
    );

    draw_arrow(canvas, cx + lw / 2, cy - h - 50, 0, 30);
    draw_arrow(canvas, cx - lw / 2, cy + h + 50, 0, -30);
    draw_arrow(canvas, cx + h + 50, cy - lw / 2, -30, 0);
    draw_arrow(canvas, cx - h - 50, cy + lw / 2, 30, 0);
}

fn stroke(canvas: &mut Canvas<Window>, x1: i32, y1: i32, x2: i32, y2: i32) {
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(x1, y1),
        sdl2::rect::Point::new(x2, y2),
    );
}

/// Simple 5×7 stroke font for direction labels (no TTF dependency).
fn draw_label_word(canvas: &mut Canvas<Window>, word: &str, x: i32, y: i32, scale: i32) {
    let mut cursor = x;
    for ch in word.chars() {
        draw_label_char(canvas, ch, cursor, y, scale);
        cursor += 6 * scale;
    }
}

fn draw_label_char(canvas: &mut Canvas<Window>, ch: char, x: i32, y: i32, s: i32) {
    let mut seg = |x1: i32, y1: i32, x2: i32, y2: i32| {
        stroke(canvas, x + x1 * s, y + y1 * s, x + x2 * s, y + y2 * s);
    };
    match ch.to_ascii_uppercase() {
        'N' => {
            seg(0, 6, 0, 0);
            seg(0, 0, 4, 6);
            seg(4, 6, 4, 0);
        }
        'O' => {
            seg(1, 0, 3, 0);
            seg(1, 6, 3, 6);
            seg(0, 1, 0, 5);
            seg(4, 1, 4, 5);
        }
        'R' => {
            seg(0, 0, 0, 6);
            seg(0, 6, 3, 6);
            seg(3, 6, 4, 5);
            seg(4, 5, 3, 4);
            seg(3, 4, 4, 0);
        }
        'T' => {
            seg(0, 6, 4, 6);
            seg(2, 6, 2, 0);
        }
        'H' => {
            seg(0, 0, 0, 6);
            seg(4, 0, 4, 6);
            seg(0, 3, 4, 3);
        }
        'S' => {
            seg(4, 6, 1, 6);
            seg(1, 6, 0, 5);
            seg(0, 5, 0, 4);
            seg(0, 4, 4, 2);
            seg(4, 2, 4, 1);
            seg(4, 1, 3, 0);
            seg(3, 0, 0, 0);
        }
        'E' => {
            seg(0, 0, 0, 6);
            seg(0, 6, 4, 6);
            seg(0, 3, 3, 3);
            seg(0, 0, 4, 0);
        }
        'W' => {
            seg(0, 6, 0, 0);
            seg(0, 0, 2, 3);
            seg(2, 3, 4, 0);
            seg(4, 0, 4, 6);
        }
        _ => {}
    }
}

fn label_width(word: &str, scale: i32) -> i32 {
    word.chars().count() as i32 * 6 * scale
}

fn label_height(scale: i32) -> i32 {
    8 * scale
}

fn draw_cardinal_labels(canvas: &mut Canvas<Window>) {
    set_color(canvas, 210, 215, 225);
    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;
    let scale = 2;
    let pad = 14;

    draw_label_word(canvas, "NORTH", cx - label_width("NORTH", scale) / 2, pad, scale);
    draw_label_word(
        canvas,
        "SOUTH",
        cx - label_width("SOUTH", scale) / 2,
        config::WINDOW_HEIGHT as i32 - pad - label_height(scale),
        scale,
    );
    draw_label_word(canvas, "WEST", pad, cy - label_height(scale) / 2, scale);
    draw_label_word(
        canvas,
        "EAST",
        config::WINDOW_WIDTH as i32 - pad - label_width("EAST", scale),
        cy - label_height(scale) / 2,
        scale,
    );
}

fn draw_vehicles(canvas: &mut Canvas<Window>, sim: &Simulation) {
    for v in &sim.vehicles {
        set_color(canvas, v.color.r, v.color.g, v.color.b);
        let hw = (v.width * 0.5) as i32;
        let hh = (v.height * 0.5) as i32;
        fill_rect(
            canvas,
            v.x as i32 - hw,
            v.y as i32 - hh,
            (hw * 2) as u32,
            (hh * 2) as u32,
        );
    }
}
