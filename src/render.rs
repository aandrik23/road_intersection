use crate::config;
use crate::simulation::Simulation;
use crate::types::{route_color, LaneId, RouteType, SignalState};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// --- Theme (dark dashboard + asphalt) ---
mod theme {
    pub const BG: (u8, u8, u8) = (18, 21, 28);
    pub const SIDEWALK: (u8, u8, u8) = (50, 55, 64);
    pub const CURB: (u8, u8, u8) = (72, 78, 88);
    pub const ASPHALT: (u8, u8, u8) = (58, 63, 72);
    pub const ASPHALT_LIGHT: (u8, u8, u8) = (68, 74, 84);
    pub const INTERSECTION: (u8, u8, u8) = (64, 70, 80);
    pub const INTERSECTION_GUIDE: (u8, u8, u8) = (88, 94, 108);
    pub const KEEP_CLEAR: (u8, u8, u8) = (44, 48, 56);
    pub const LANE_MARK: (u8, u8, u8) = (235, 198, 72);
    pub const CENTER_DIVIDER: (u8, u8, u8) = (38, 41, 48);
    pub const STOP_LINE: (u8, u8, u8) = (248, 250, 252);
    pub const ARROW: (u8, u8, u8) = (200, 206, 218);
    pub const LABEL: (u8, u8, u8) = (168, 176, 192);
    pub const LABEL_BG: (u8, u8, u8) = (32, 36, 44);
    pub const ROUTE_GHOST: (u8, u8, u8) = (72, 78, 90);
    pub const CROSSWALK: (u8, u8, u8) = (230, 234, 242);
    pub const HUD_BG: (u8, u8, u8) = (28, 32, 40);
    pub const HUD_BORDER: (u8, u8, u8) = (56, 120, 200);
    pub const HUD_TEXT: (u8, u8, u8) = (210, 216, 228);
    pub const HUD_MUTED: (u8, u8, u8) = (130, 138, 154);
    pub const LAMP_OFF: (u8, u8, u8) = (36, 40, 48);
    pub const LAMP_RED: (u8, u8, u8) = (255, 72, 88);
    pub const LAMP_RED_GLOW: (u8, u8, u8) = (120, 28, 38);
    pub const LAMP_GREEN: (u8, u8, u8) = (56, 232, 120);
    pub const LAMP_GREEN_GLOW: (u8, u8, u8) = (24, 90, 52);
    pub const HOUSING: (u8, u8, u8) = (24, 28, 34);
    pub const HOUSING_EDGE: (u8, u8, u8) = (70, 76, 88);
    pub const VEHICLE_SHADOW: (u8, u8, u8) = (12, 14, 18);
    pub const VEHICLE_OUTLINE: (u8, u8, u8) = (18, 20, 26);
}

pub struct AppRenderer {
    pub canvas: Canvas<Window>,
}

impl AppRenderer {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self { canvas }
    }

    pub fn draw_frame(&mut self, sim: &Simulation) -> Result<(), String> {
        let play_h = playfield_height();

        set_color(&mut self.canvas, theme::BG.0, theme::BG.1, theme::BG.2);
        self.canvas.clear();

        draw_background(&mut self.canvas, play_h);
        draw_roads(&mut self.canvas, play_h);
        draw_crosswalks(&mut self.canvas);
        draw_intersection_zone(&mut self.canvas);
        draw_route_preview(&mut self.canvas, sim);
        draw_lane_markings(&mut self.canvas, play_h);
        draw_stop_lines(&mut self.canvas, sim);
        draw_direction_arrows(&mut self.canvas);
        draw_cardinal_labels(&mut self.canvas);
        draw_traffic_lights(&mut self.canvas, sim);
        draw_vehicles(&mut self.canvas, sim);
        draw_hud(&mut self.canvas, sim);

        self.canvas.present();
        Ok(())
    }
}

fn playfield_height() -> i32 {
    config::WINDOW_HEIGHT as i32 - config::HUD_HEIGHT as i32
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

fn fill_rounded_rect(canvas: &mut Canvas<Window>, x: i32, y: i32, w: u32, h: u32, radius: i32) {
    let r = radius.min((w as i32 / 2).min(h as i32 / 2));
    if r <= 0 {
        fill_rect(canvas, x, y, w, h);
        return;
    }
    fill_rect(canvas, x + r, y, w.saturating_sub((2 * r) as u32), h);
    fill_rect(canvas, x, y + r, w, h.saturating_sub((2 * r) as u32));
    fill_circle(canvas, x + r, y + r, r);
    fill_circle(canvas, x + w as i32 - r, y + r, r);
    fill_circle(canvas, x + r, y + h as i32 - r, r);
    fill_circle(canvas, x + w as i32 - r, y + h as i32 - r, r);
}

fn draw_dashed_line(
    canvas: &mut Canvas<Window>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    dash: i32,
    gap: i32,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = ((dx * dx + dy * dy) as f32).sqrt() as i32;
    if len == 0 {
        return;
    }
    let steps = len;
    let mut dist = 0;
    let mut drawing = true;
    let mut seg = 0;

    while dist <= steps {
        let t = dist as f32 / steps as f32;
        let px = x0 + (dx as f32 * t) as i32;
        let py = y0 + (dy as f32 * t) as i32;
        if drawing {
            let _ = canvas.draw_point(sdl2::rect::Point::new(px, py));
        }
        seg += 1;
        let limit = if drawing { dash } else { gap };
        if seg >= limit {
            seg = 0;
            drawing = !drawing;
        }
        dist += 2;
    }
}

fn draw_background(canvas: &mut Canvas<Window>, play_h: i32) {
    set_color(canvas, theme::SIDEWALK.0, theme::SIDEWALK.1, theme::SIDEWALK.2);
    fill_rect(canvas, 0, 0, config::WINDOW_WIDTH, play_h as u32);

    let margin_x = 72;
    let margin_y = 48;
    set_color(canvas, theme::CURB.0, theme::CURB.1, theme::CURB.2);
    fill_rect(canvas, 0, 0, config::WINDOW_WIDTH, margin_y as u32);
    fill_rect(
        canvas,
        0,
        play_h - margin_y,
        config::WINDOW_WIDTH,
        margin_y as u32,
    );
    fill_rect(canvas, 0, 0, margin_x as u32, play_h as u32);
    fill_rect(
        canvas,
        config::WINDOW_WIDTH as i32 - margin_x,
        0,
        margin_x as u32,
        play_h as u32,
    );
}

fn draw_roads(canvas: &mut Canvas<Window>, play_h: i32) {
    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;
    let h = config::INTERSECTION_HALF as i32;
    let road_half = config::LANE_WIDTH as i32 + 10;

    let margin_x = 95;
    let margin_y = 55;

    set_color(canvas, theme::ASPHALT.0, theme::ASPHALT.1, theme::ASPHALT.2);

    fill_rect(
        canvas,
        cx - road_half,
        margin_y,
        (road_half * 2) as u32,
        (play_h - margin_y * 2) as u32,
    );

    fill_rect(
        canvas,
        margin_x,
        cy - road_half,
        (config::WINDOW_WIDTH as i32 - margin_x * 2) as u32,
        (road_half * 2) as u32,
    );

    set_color(
        canvas,
        theme::INTERSECTION.0,
        theme::INTERSECTION.1,
        theme::INTERSECTION.2,
    );
    fill_rect(canvas, cx - h, cy - h, (h * 2) as u32, (h * 2) as u32);

    set_color(
        canvas,
        theme::CENTER_DIVIDER.0,
        theme::CENTER_DIVIDER.1,
        theme::CENTER_DIVIDER.2,
    );
    fill_rect(canvas, cx - 2, margin_y, 4, (play_h - margin_y * 2) as u32);
    fill_rect(
        canvas,
        margin_x,
        cy - 2,
        (config::WINDOW_WIDTH as i32 - margin_x * 2) as u32,
        4,
    );
}

fn draw_crosswalks(canvas: &mut Canvas<Window>) {
    let ix0 = config::IX0 as i32;
    let ix1 = config::IX1 as i32;
    let iy0 = config::IY0 as i32;
    let iy1 = config::IY1 as i32;
    let enter_sb = config::ENTER_SB_X as i32;
    let enter_nb = config::ENTER_NB_X as i32;
    let enter_eb = config::ENTER_EB_Y as i32;
    let enter_wb = config::ENTER_WB_Y as i32;

    set_color(
        canvas,
        theme::CROSSWALK.0,
        theme::CROSSWALK.1,
        theme::CROSSWALK.2,
    );

    let stripe = 4;
    let gap = 5;
    let span = 28;

    for i in 0..5 {
        let offset = i * (stripe + gap);
        fill_rect(
            canvas,
            enter_sb - span / 2 + offset,
            iy0 - 10,
            stripe as u32,
            10,
        );
        fill_rect(
            canvas,
            enter_nb - span / 2 + offset,
            iy1,
            stripe as u32,
            10,
        );
        fill_rect(
            canvas,
            ix0 - 10,
            enter_eb - span / 2 + offset,
            10,
            stripe as u32,
        );
        fill_rect(
            canvas,
            ix1,
            enter_wb - span / 2 + offset,
            10,
            stripe as u32,
        );
    }
}

fn draw_lane_markings(canvas: &mut Canvas<Window>, play_h: i32) {
    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;
    let h = config::INTERSECTION_HALF as i32;
    let margin_x = 95;
    let margin_y = 55;

    set_color(
        canvas,
        theme::LANE_MARK.0,
        theme::LANE_MARK.1,
        theme::LANE_MARK.2,
    );

    let exit_nb = config::EXIT_NB_X as i32;
    let exit_sb = config::EXIT_SB_X as i32;
    let exit_eb = config::EXIT_EB_Y as i32;
    let exit_wb = config::EXIT_WB_Y as i32;
    let win_w = config::WINDOW_WIDTH as i32;

    draw_dashed_line(canvas, exit_nb, margin_y, exit_nb, cy - h, 10, 8);
    draw_dashed_line(canvas, exit_sb, cy + h, exit_sb, play_h - margin_y, 10, 8);
    draw_dashed_line(canvas, cx + h, exit_eb, win_w - margin_x, exit_eb, 10, 8);
    draw_dashed_line(canvas, margin_x, exit_wb, cx - h, exit_wb, 10, 8);
}

fn draw_stop_lines(canvas: &mut Canvas<Window>, sim: &Simulation) {
    set_color(
        canvas,
        theme::STOP_LINE.0,
        theme::STOP_LINE.1,
        theme::STOP_LINE.2,
    );
    for lane_id in LaneId::ALL {
        let lane = sim.world.lane(lane_id);
        if !lane.inbound {
            continue;
        }
        let sx = lane.stop_line.x as i32;
        let sy = lane.stop_line.y as i32;
        if lane.heading == 90.0 || lane.heading == 270.0 {
            fill_rect(canvas, sx - 18, sy - 2, 36, 4);
        } else {
            fill_rect(canvas, sx - 2, sy - 18, 4, 36);
        }
    }
}

fn inside_intersection(x: f32, y: f32) -> bool {
    x >= config::IX0 && x <= config::IX1 && y >= config::IY0 && y <= config::IY1
}

fn draw_route_preview(canvas: &mut Canvas<Window>, sim: &Simulation) {
    // Only faint approach-arm hints — skip the busy intersection interior.
    for lane_id in LaneId::ALL {
        if !sim.world.lane(lane_id).inbound {
            continue;
        }
        let path = sim.world.route(lane_id, RouteType::Straight);
        set_color(
            canvas,
            theme::ROUTE_GHOST.0,
            theme::ROUTE_GHOST.1,
            theme::ROUTE_GHOST.2,
        );
        for w in path.waypoints.windows(2) {
            if inside_intersection(w[0].x, w[0].y) && inside_intersection(w[1].x, w[1].y) {
                continue;
            }
            let _ = canvas.draw_line(
                sdl2::rect::Point::new(w[0].x as i32, w[0].y as i32),
                sdl2::rect::Point::new(w[1].x as i32, w[1].y as i32),
            );
        }
    }
}

fn draw_intersection_zone(canvas: &mut Canvas<Window>) {
    let x = (config::CENTER_X - config::INTERSECTION_HALF) as i32;
    let y = (config::CENTER_Y - config::INTERSECTION_HALF) as i32;
    let s = (config::INTERSECTION_HALF * 2.0) as u32;
    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;

    set_color(
        canvas,
        theme::INTERSECTION.0,
        theme::INTERSECTION.1,
        theme::INTERSECTION.2,
    );
    fill_rect(canvas, x, y, s, s);

    // Keep-clear center so overlapping cars are easier to read.
    let clear_r = (config::LANE_WIDTH * 0.55) as i32;
    set_color(
        canvas,
        theme::KEEP_CLEAR.0,
        theme::KEEP_CLEAR.1,
        theme::KEEP_CLEAR.2,
    );
    fill_circle(canvas, cx, cy, clear_r);

    set_color(
        canvas,
        theme::INTERSECTION_GUIDE.0,
        theme::INTERSECTION_GUIDE.1,
        theme::INTERSECTION_GUIDE.2,
    );
    // Lane centerlines through the box (where cars drive, not the border).
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(cx, y),
        sdl2::rect::Point::new(cx, y + s as i32),
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(x, cy),
        sdl2::rect::Point::new(x + s as i32, cy),
    );

    set_color(
        canvas,
        theme::ASPHALT_LIGHT.0,
        theme::ASPHALT_LIGHT.1,
        theme::ASPHALT_LIGHT.2,
    );
    let _ = canvas.draw_rect(Rect::new(x, y, s, s));

}

fn lamp_colors(active: bool, green: bool) -> ((u8, u8, u8), (u8, u8, u8)) {
    if !active {
        return (theme::LAMP_OFF, theme::LAMP_OFF);
    }
    if green {
        (theme::LAMP_GREEN_GLOW, theme::LAMP_GREEN)
    } else {
        (theme::LAMP_RED_GLOW, theme::LAMP_RED)
    }
}

fn draw_traffic_light(
    canvas: &mut Canvas<Window>,
    lane: LaneId,
    x: i32,
    y: i32,
    state: SignalState,
) {
    let red_on = state == SignalState::Red;
    let green_on = state == SignalState::Green;
    let spacing = 12;
    let lamp_r = 6;

    set_color(
        canvas,
        theme::HOUSING.0,
        theme::HOUSING.1,
        theme::HOUSING.2,
    );
    match lane {
        LaneId::NorthSb | LaneId::SouthNb => {
            fill_rounded_rect(
                canvas,
                x - 8,
                y - spacing - lamp_r - 4,
                16,
                (spacing * 2 + lamp_r * 2 + 8) as u32,
                4,
            );
        }
        LaneId::WestEb | LaneId::EastWb => {
            fill_rounded_rect(
                canvas,
                x - spacing - lamp_r - 4,
                y - 8,
                (spacing * 2 + lamp_r * 2 + 8) as u32,
                16,
                4,
            );
        }
        _ => return,
    }

    set_color(
        canvas,
        theme::HOUSING_EDGE.0,
        theme::HOUSING_EDGE.1,
        theme::HOUSING_EDGE.2,
    );
    match lane {
        LaneId::NorthSb | LaneId::SouthNb => {
            let _ = canvas.draw_rect(Rect::new(
                x - 8,
                y - spacing - lamp_r - 4,
                16,
                (spacing * 2 + lamp_r * 2 + 8) as u32,
            ));
        }
        LaneId::WestEb | LaneId::EastWb => {
            let _ = canvas.draw_rect(Rect::new(
                x - spacing - lamp_r - 4,
                y - 8,
                (spacing * 2 + lamp_r * 2 + 8) as u32,
                16,
            ));
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

    let ((gr, gg, gb), (cr, cg, cb)) = lamp_colors(red_on, false);
    set_color(canvas, gr, gg, gb);
    fill_circle(canvas, rx, ry, lamp_r + 2);
    set_color(canvas, cr, cg, cb);
    fill_circle(canvas, rx, ry, lamp_r);

    let ((gr, gg, gb), (cr, cg, cb)) = lamp_colors(green_on, true);
    set_color(canvas, gr, gg, gb);
    fill_circle(canvas, gx, gy, lamp_r + 2);
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
        draw_traffic_light(
            canvas,
            lane_id,
            lane.light_pos.x as i32,
            lane.light_pos.y as i32,
            state,
        );
    }
}

fn draw_arrow(canvas: &mut Canvas<Window>, x: i32, y: i32, dx: i32, dy: i32) {
    set_color(canvas, theme::ARROW.0, theme::ARROW.1, theme::ARROW.2);
    let tip_x = x + dx;
    let tip_y = y + dy;
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(x, y),
        sdl2::rect::Point::new(tip_x, tip_y),
    );
    if dy > 0 {
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x - 6, tip_y - 10),
        );
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x + 6, tip_y - 10),
        );
    } else if dy < 0 {
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x - 6, tip_y + 10),
        );
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x + 6, tip_y + 10),
        );
    } else if dx > 0 {
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x - 10, tip_y - 6),
        );
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x - 10, tip_y + 6),
        );
    } else if dx < 0 {
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x + 10, tip_y - 6),
        );
        let _ = canvas.draw_line(
            sdl2::rect::Point::new(tip_x, tip_y),
            sdl2::rect::Point::new(tip_x + 10, tip_y + 6),
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
    let play_h = playfield_height();
    let win_w = config::WINDOW_WIDTH as i32;

    draw_arrow(canvas, enter_sb, margin_y, 0, 40);
    draw_arrow(canvas, enter_nb, play_h - margin_y, 0, -40);
    draw_arrow(canvas, margin_x, enter_eb, 40, 0);
    draw_arrow(canvas, win_w - margin_x, enter_wb, -40, 0);

    draw_arrow(canvas, enter_sb, iy0 - 50, 0, 30);
    draw_arrow(canvas, enter_nb, iy1 + 50, 0, -30);
    draw_arrow(canvas, ix0 - 50, enter_eb, 30, 0);
    draw_arrow(canvas, ix1 + 50, enter_wb, -30, 0);
}

fn glyph_pattern(ch: char) -> [&'static str; 7] {
    match ch.to_ascii_uppercase() {
        '0' => ["01110", "10001", "10011", "10101", "11001", "10001", "01110"],
        '1' => ["00100", "01100", "00100", "00100", "00100", "00100", "01110"],
        '2' => ["01110", "10001", "00001", "00110", "01000", "10000", "11111"],
        '3' => ["01110", "10001", "00001", "00110", "00001", "10001", "01110"],
        '4' => ["00010", "00110", "01010", "10010", "11111", "00010", "00010"],
        '5' => ["11111", "10000", "11110", "00001", "00001", "10001", "01110"],
        '6' => ["01110", "10000", "11110", "10001", "10001", "10001", "01110"],
        '7' => ["11111", "00001", "00010", "00100", "01000", "01000", "01000"],
        '8' => ["01110", "10001", "10001", "01110", "10001", "10001", "01110"],
        '9' => ["01110", "10001", "10001", "10001", "01111", "00001", "01110"],
        'A' => ["01110", "10001", "10001", "11111", "10001", "10001", "10001"],
        'B' => ["11110", "10001", "10001", "11110", "10001", "10001", "11110"],
        'C' => ["01110", "10001", "10000", "10000", "10000", "10001", "01110"],
        'D' => ["11110", "10001", "10001", "10001", "10001", "10001", "11110"],
        'E' => ["11111", "10000", "10000", "11110", "10000", "10000", "11111"],
        'F' => ["11111", "10000", "10000", "11110", "10000", "10000", "10000"],
        'G' => ["01110", "10001", "10000", "10111", "10001", "10001", "01110"],
        'H' => ["10001", "10001", "10001", "11111", "10001", "10001", "10001"],
        'I' => ["01110", "00100", "00100", "00100", "00100", "00100", "01110"],
        'J' => ["00111", "00010", "00010", "00010", "10010", "10010", "01100"],
        'K' => ["10001", "10010", "10100", "11000", "10100", "10010", "10001"],
        'L' => ["10000", "10000", "10000", "10000", "10000", "10000", "11111"],
        'M' => ["10001", "11011", "10101", "10001", "10001", "10001", "10001"],
        'N' => ["10001", "11001", "10101", "10011", "10001", "10001", "10001"],
        'O' => ["01110", "10001", "10001", "10001", "10001", "10001", "01110"],
        'P' => ["11110", "10001", "10001", "11110", "10000", "10000", "10000"],
        'Q' => ["01110", "10001", "10001", "10001", "10101", "10010", "01101"],
        'R' => ["11110", "10001", "10001", "11110", "10100", "10010", "10001"],
        'S' => ["01111", "10000", "10000", "01110", "00001", "00001", "11110"],
        'T' => ["11111", "00100", "00100", "00100", "00100", "00100", "00100"],
        'U' => ["10001", "10001", "10001", "10001", "10001", "10001", "01110"],
        'V' => ["10001", "10001", "10001", "10001", "10001", "01010", "00100"],
        'W' => ["10001", "10001", "10001", "10101", "10101", "11011", "10001"],
        'X' => ["10001", "10001", "01010", "00100", "01010", "10001", "10001"],
        'Y' => ["10001", "10001", "01010", "00100", "00100", "00100", "00100"],
        'Z' => ["11111", "00001", "00010", "00100", "01000", "10000", "11111"],
        '-' => ["00000", "00000", "00000", "11111", "00000", "00000", "00000"],
        ':' => ["00000", "00100", "00100", "00000", "00100", "00100", "00000"],
        ' ' => ["00000", "00000", "00000", "00000", "00000", "00000", "00000"],
        _ => ["00000", "00000", "00000", "00000", "00000", "00000", "00000"],
    }
}

fn draw_label_char(canvas: &mut Canvas<Window>, ch: char, x: i32, y: i32, scale: i32) {
    let pattern = glyph_pattern(ch);
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

fn draw_label_pill(canvas: &mut Canvas<Window>, word: &str, x: i32, y: i32, scale: i32) {
    let pad_x = 8;
    let pad_y = 4;
    let w = label_width(word, scale) + pad_x * 2;
    let h = label_height(scale) + pad_y * 2;
    set_color(
        canvas,
        theme::LABEL_BG.0,
        theme::LABEL_BG.1,
        theme::LABEL_BG.2,
    );
    fill_rounded_rect(canvas, x, y, w as u32, h as u32, 6);
    set_color(canvas, theme::LABEL.0, theme::LABEL.1, theme::LABEL.2);
    draw_label_word(canvas, word, x + pad_x, y + pad_y, scale);
}

fn draw_cardinal_labels(canvas: &mut Canvas<Window>) {
    let cx = config::CENTER_X as i32;
    let cy = config::CENTER_Y as i32;
    let scale = 2;

    draw_label_pill(
        canvas,
        "NORTH",
        cx - label_width("NORTH", scale) / 2 - 8,
        22,
        scale,
    );
    draw_label_pill(
        canvas,
        "SOUTH",
        cx - label_width("SOUTH", scale) / 2 - 8,
        playfield_height() - 22 - label_height(scale) - 8,
        scale,
    );
    draw_label_pill(canvas, "WEST", 28, cy - label_height(scale) / 2 - 4, scale);
    draw_label_pill(
        canvas,
        "EAST",
        config::WINDOW_WIDTH as i32 - 28 - label_width("EAST", scale) - 16,
        cy - label_height(scale) / 2 - 4,
        scale,
    );
}

fn draw_vehicle(canvas: &mut Canvas<Window>, sim: &Simulation, index: usize) {
    let v = &sim.vehicles[index];
    let pos = v.position();
    let scale = config::VEHICLE_DRAW_SCALE;
    let (width, height) = v.draw_extents();
    let width = width * scale;
    let height = height * scale;
    let hw = (width * 0.5) as i32;
    let hh = (height * 0.5) as i32;
    let x = pos.x as i32 - hw;
    let y = pos.y as i32 - hh;
    let w = (hw * 2).max(4) as u32;
    let h = (hh * 2).max(4) as u32;

    set_color(
        canvas,
        theme::VEHICLE_SHADOW.0,
        theme::VEHICLE_SHADOW.1,
        theme::VEHICLE_SHADOW.2,
    );
    fill_rounded_rect(canvas, x + 2, y + 3, w, h, 5);

    let color = v.color();
    set_color(canvas, color.r, color.g, color.b);
    fill_rounded_rect(canvas, x, y, w, h, 5);

    set_color(
        canvas,
        theme::VEHICLE_OUTLINE.0,
        theme::VEHICLE_OUTLINE.1,
        theme::VEHICLE_OUTLINE.2,
    );
    let _ = canvas.draw_rect(Rect::new(x, y, w, h));

    let heading = v.heading();
    set_color(
        canvas,
        (color.r / 2 + 40).min(255),
        (color.g / 2 + 40).min(255),
        (color.b / 2 + 40).min(255),
    );
    if (45.0..135.0).contains(&heading) || (225.0..315.0).contains(&heading) {
        fill_rect(canvas, x + hw - 4, y + 3, 8, (h / 3).max(4));
    } else {
        fill_rect(canvas, x + 3, y + hh - 4, (w / 3).max(4), 8);
    }
}

fn draw_vehicles(canvas: &mut Canvas<Window>, sim: &Simulation) {
    for i in 0..sim.vehicles.len() {
        draw_vehicle(canvas, sim, i);
    }
}

fn active_phase_label(sim: &Simulation) -> &'static str {
    if sim.is_green(LaneId::NorthSb) || sim.is_green(LaneId::SouthNb) {
        "N-S GREEN"
    } else {
        "E-W GREEN"
    }
}

fn draw_hud(canvas: &mut Canvas<Window>, sim: &Simulation) {
    let hud_y = playfield_height();

    set_color(canvas, theme::HUD_BG.0, theme::HUD_BG.1, theme::HUD_BG.2);
    fill_rect(
        canvas,
        0,
        hud_y,
        config::WINDOW_WIDTH,
        config::HUD_HEIGHT,
    );

    set_color(
        canvas,
        theme::HUD_BORDER.0,
        theme::HUD_BORDER.1,
        theme::HUD_BORDER.2,
    );
    fill_rect(canvas, 0, hud_y, config::WINDOW_WIDTH, 3);

    let scale = 2;
    let row1 = hud_y + 14;
    let row2 = hud_y + 38;
    let row3 = hud_y + 62;

    set_color(canvas, theme::HUD_TEXT.0, theme::HUD_TEXT.1, theme::HUD_TEXT.2);
    draw_label_word(canvas, "ROAD INTERSECTION", 20, row1, scale);

    set_color(canvas, theme::HUD_MUTED.0, theme::HUD_MUTED.1, theme::HUD_MUTED.2);
    draw_label_word(
        canvas,
        "ARROWS SPAWN  R RANDOM  ESC QUIT",
        20,
        row2,
        scale,
    );

    let phase = active_phase_label(sim);
    let count = sim.vehicles.len();
    let mut stats = String::with_capacity(32);
    stats.push_str("VEHICLES ");
    stats.push_str(&count.to_string());
    stats.push_str("   ");
    stats.push_str(phase);

    set_color(canvas, theme::HUD_TEXT.0, theme::HUD_TEXT.1, theme::HUD_TEXT.2);
    draw_label_word(canvas, &stats, 20, row3, scale);

    let legend_x = config::WINDOW_WIDTH as i32 - 320;
    draw_route_legend(canvas, legend_x, row2);
}

fn draw_route_legend(canvas: &mut Canvas<Window>, x: i32, y: i32) {
    let scale = 2;
    let swatch = 12;
    let gap = 100;
    let labels = [("LEFT", RouteType::Left), ("RIGHT", RouteType::Right), ("STRAIGHT", RouteType::Straight)];

    for (i, (name, route)) in labels.iter().enumerate() {
        let ox = x + i as i32 * gap;
        let c = route_color(*route);
        set_color(canvas, c.r, c.g, c.b);
        fill_rounded_rect(canvas, ox, y + 2, swatch as u32, swatch as u32, 3);
        set_color(canvas, theme::HUD_MUTED.0, theme::HUD_MUTED.1, theme::HUD_MUTED.2);
        draw_label_word(canvas, name, ox + swatch + 6, y, scale);
    }
}
