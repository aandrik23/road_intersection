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

fn fill_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                let _ = canvas.draw_point(sdl2::rect::Point::new(cx + x, cy + y));
            }
        }
    }
}

fn draw_roads(canvas: &mut Canvas<Window>) {
    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;
    let h = config::INTERSECTION_HALF as i32;
    let road_half = config::LANE_WIDTH as i32 + 6;

    let margin_x = 95;
    let margin_y = 55;

    set_color(canvas, 35, 38, 46);
    fill_rect(canvas, 0, 0, config::WINDOW_WIDTH, config::WINDOW_HEIGHT);

    set_color(canvas, 62, 66, 74);

    fill_rect(
        canvas,
        cx - road_half,
        margin_y,
        (road_half * 2) as u32,
        (config::WINDOW_HEIGHT as i32 - margin_y * 2) as u32,
    );

    fill_rect(
        canvas,
        margin_x,
        cy - road_half,
        (config::WINDOW_WIDTH as i32 - margin_x * 2) as u32,
        (road_half * 2) as u32,
    );

    set_color(canvas, 45, 48, 55);
    fill_rect(canvas, cx - h, cy - h, (h * 2) as u32, (h * 2) as u32);

    set_color(canvas, 220, 225, 235);

    let exit_nb = config::EXIT_NB_X as i32;
    let exit_sb = config::EXIT_SB_X as i32;
    let exit_eb = config::EXIT_EB_Y as i32;
    let exit_wb = config::EXIT_WB_Y as i32;
    let win_h = config::WINDOW_HEIGHT as i32;
    let win_w = config::WINDOW_WIDTH as i32;

    // Departure lane centerlines (spec: ↑ north, ↓ south, → east, ← west).
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(exit_nb, margin_y),
        sdl2::rect::Point::new(exit_nb, cy - h),
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(exit_sb, cy + h),
        sdl2::rect::Point::new(exit_sb, win_h - margin_y),
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(cx + h, exit_eb),
        sdl2::rect::Point::new(win_w - margin_x, exit_eb),
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(margin_x, exit_wb),
        sdl2::rect::Point::new(cx - h, exit_wb),
    );

    set_color(canvas, 40, 42, 48);
    fill_rect(
        canvas,
        cx - 2,
        margin_y,
        4,
        (config::WINDOW_HEIGHT as i32 - margin_y * 2) as u32,
    );
    fill_rect(
        canvas,
        margin_x,
        cy - 2,
        (config::WINDOW_WIDTH as i32 - margin_x * 2) as u32,
        4,
    );
}

fn draw_stop_lines(canvas: &mut Canvas<Window>, sim: &Simulation) {
    set_color(canvas, 240, 240, 240);
    for lane_id in LaneId::ALL {
        let lane = sim.world.lane(lane_id);
        if !lane.inbound {
            continue;
        }
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

fn lamp_color(active: bool, green: bool) -> (u8, u8, u8) {
    if !active {
        return (42, 44, 50);
    }
    if green {
        (40, 210, 80)
    } else {
        (235, 50, 60)
    }
}

fn draw_traffic_light(canvas: &mut Canvas<Window>, lane: LaneId, x: i32, y: i32, state: SignalState) {
    let red_on = state == SignalState::Red;
    let green_on = state == SignalState::Green;
    let spacing = 11;
    let lamp_r = 5;

    // Compact side-mounted signal: thin outline + two lamps (no solid block on the lane).
    set_color(canvas, 55, 58, 66);
    match lane {
        LaneId::NorthSb | LaneId::SouthNb => {
            let _ = canvas.draw_rect(Rect::new(
                x - 6,
                y - spacing - lamp_r - 2,
                12,
                (spacing * 2 + lamp_r * 2 + 4) as u32,
            ));
            set_color(canvas, 70, 72, 80);
            fill_rect(canvas, x - 1, y - spacing - 4, 2, (spacing * 2 + 8) as u32);
        }
        LaneId::WestEb | LaneId::EastWb => {
            let _ = canvas.draw_rect(Rect::new(
                x - spacing - lamp_r - 2,
                y - 6,
                (spacing * 2 + lamp_r * 2 + 4) as u32,
                12,
            ));
            set_color(canvas, 70, 72, 80);
            fill_rect(canvas, x - spacing - 4, y - 1, (spacing * 2 + 8) as u32, 2);
        }
        _ => return,
    }

    let (rx, ry, gx, gy) = match lane {
        LaneId::NorthSb => (x, y - spacing, x, y + spacing),
        LaneId::SouthNb => (x, y + spacing, x, y - spacing),
        LaneId::WestEb => (x - spacing, y, x + spacing, y),
        LaneId::EastWb => (x + spacing, y, x - spacing, y),
        _ => return,
    };

    let (cr, cg, cb) = lamp_color(red_on, false);
    set_color(canvas, cr, cg, cb);
    fill_circle(canvas, rx, ry, lamp_r);

    let (cr, cg, cb) = lamp_color(green_on, true);
    set_color(canvas, cr, cg, cb);
    fill_circle(canvas, gx, gy, lamp_r);
}

fn draw_traffic_lights(canvas: &mut Canvas<Window>, sim: &Simulation) {
    for lane_id in LaneId::ALL {
        let lane = sim.world.lane(lane_id);
        if !lane.inbound {
            continue;
        }

        let state = sim.lane_signal(lane_id);
        let x = lane.light_pos.x as i32;
        let y = lane.light_pos.y as i32;
        draw_traffic_light(canvas, lane_id, x, y, state);
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
    let ix0 = config::IX0 as i32;
    let ix1 = config::IX1 as i32;
    let iy0 = config::IY0 as i32;
    let iy1 = config::IY1 as i32;
    let enter_nb = config::ENTER_NB_X as i32;
    let enter_sb = config::ENTER_SB_X as i32;
    let enter_eb = config::ENTER_EB_Y as i32;
    let enter_wb = config::ENTER_WB_Y as i32;
    let margin_y = 70;
    let margin_x = 70;
    let win_h = config::WINDOW_HEIGHT as i32;
    let win_w = config::WINDOW_WIDTH as i32;

    // Enter-lane arrows matching the spec diagram (↓ ↑ ← →).
    // From north (southbound ↓) on west lane.
    draw_arrow(canvas, enter_sb, margin_y, 0, 40);
    // From south (northbound ↑) on east lane.
    draw_arrow(canvas, enter_nb, win_h - margin_y, 0, -40);
    // From west (eastbound →) on south row.
    draw_arrow(canvas, margin_x, enter_eb, 40, 0);
    // From east (westbound ←) on north row.
    draw_arrow(canvas, win_w - margin_x, enter_wb, -40, 0);

    // Near the stop line, same directions.
    draw_arrow(canvas, enter_sb, iy0 - 50, 0, 30);
    draw_arrow(canvas, enter_nb, iy1 + 50, 0, -30);
    draw_arrow(canvas, ix0 - 50, enter_eb, 30, 0);
    draw_arrow(canvas, ix1 + 50, enter_wb, -30, 0);
}

fn draw_label_char(canvas: &mut Canvas<Window>, ch: char, x: i32, y: i32, scale: i32) {
    let pattern: [&str; 7] = match ch.to_ascii_uppercase() {
        'A' => ["01110", "10001", "10001", "11111", "10001", "10001", "10001"],
        'E' => ["11111", "10000", "10000", "11110", "10000", "10000", "11111"],
        'H' => ["10001", "10001", "10001", "11111", "10001", "10001", "10001"],
        'N' => ["10001", "11001", "10101", "10011", "10001", "10001", "10001"],
        'O' => ["01110", "10001", "10001", "10001", "10001", "10001", "01110"],
        'R' => ["11110", "10001", "10001", "11110", "10100", "10010", "10001"],
        'S' => ["01111", "10000", "10000", "01110", "00001", "00001", "11110"],
        'T' => ["11111", "00100", "00100", "00100", "00100", "00100", "00100"],
        'U' => ["10001", "10001", "10001", "10001", "10001", "10001", "01110"],
        'W' => ["10001", "10001", "10001", "10101", "10101", "11011", "10001"],
        _ => ["00000", "00000", "00000", "00000", "00000", "00000", "00000"],
    };

    for (row, line) in pattern.iter().enumerate() {
        for (col, pixel) in line.chars().enumerate() {
            if pixel == '1' {
                fill_rect(
                    canvas,
                    x + col as i32 * scale,
                    y + row as i32 * scale,
                    scale as u32,
                    scale as u32,
                );
            }
        }
    }
}

fn draw_label_word(canvas: &mut Canvas<Window>, word: &str, x: i32, y: i32, scale: i32) {
    let mut cursor = x;

    for ch in word.chars() {
        draw_label_char(canvas, ch, cursor, y, scale);
        cursor += 6 * scale;
    }
}

fn label_width(word: &str, scale: i32) -> i32 {
    word.chars().count() as i32 * 6 * scale
}

fn label_height(scale: i32) -> i32 {
    7 * scale
}

fn draw_cardinal_labels(canvas: &mut Canvas<Window>) {
    set_color(canvas, 235, 238, 245);

    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;
    let scale = 3;
    let pad = 24;

    draw_label_word(
        canvas,
        "NORTH",
        cx - label_width("NORTH", scale) / 2,
        18,
        scale,
    );

    draw_label_word(
        canvas,
        "SOUTH",
        cx - label_width("SOUTH", scale) / 2,
        config::WINDOW_HEIGHT as i32 - 18 - label_height(scale),
        scale,
    );

    draw_label_word(
        canvas,
        "WEST",
        pad,
        cy - label_height(scale) / 2,
        scale,
    );

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
        let color = v.color();
        set_color(canvas, color.r, color.g, color.b);
        let pos = v.position();
        let (width, height) = v.draw_extents();
        let hw = (width * 0.5) as i32;
        let hh = (height * 0.5) as i32;
        fill_rect(
            canvas,
            pos.x as i32 - hw,
            pos.y as i32 - hh,
            (hw * 2) as u32,
            (hh * 2) as u32,
        );
    }
}
