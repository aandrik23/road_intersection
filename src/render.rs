use crate::config;
use crate::simulation::Simulation;
use crate::sprites::SpriteAtlas;
use crate::types::{route_color, LaneId, RouteType, SignalState, VehicleKind};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

// --- 90s arcade / SNES-style palette (simple, easy on the eyes) ---
mod theme {
    pub const BG: (u8, u8, u8) = (56, 104, 48);
    pub const GRASS_LIGHT: (u8, u8, u8) = (72, 128, 60);
    pub const CURB: (u8, u8, u8) = (88, 72, 48);
    pub const ASPHALT: (u8, u8, u8) = (72, 72, 80);
    pub const ASPHALT_DARK: (u8, u8, u8) = (56, 56, 64);
    pub const INTERSECTION: (u8, u8, u8) = (64, 64, 72);
    pub const INTERSECTION_LINE: (u8, u8, u8) = (200, 200, 208);
    pub const LANE_MARK: (u8, u8, u8) = (248, 248, 120);
    pub const CENTER_DIVIDER: (u8, u8, u8) = (248, 248, 248);
    pub const STOP_LINE: (u8, u8, u8) = (255, 255, 255);
    pub const ARROW: (u8, u8, u8) = (255, 255, 255);
    pub const LABEL: (u8, u8, u8) = (255, 255, 255);
    pub const LABEL_BOX: (u8, u8, u8) = (0, 0, 128);
    pub const ROUTE_GHOST: (u8, u8, u8) = (120, 120, 136);
    pub const CROSSWALK: (u8, u8, u8) = (255, 255, 255);
    pub const HUD_BG: (u8, u8, u8) = (0, 0, 128);
    pub const HUD_BAR: (u8, u8, u8) = (252, 188, 0);
    pub const HUD_TEXT: (u8, u8, u8) = (255, 255, 255);
    pub const HUD_MUTED: (u8, u8, u8) = (180, 180, 220);
    pub const HUD_BOX: (u8, u8, u8) = (0, 0, 168);
    pub const BLACK: (u8, u8, u8) = (0, 0, 0);
    pub const WHITE: (u8, u8, u8) = (255, 255, 255);
    pub const LAMP_RED: (u8, u8, u8) = (220, 0, 0);
    pub const LAMP_GREEN: (u8, u8, u8) = (0, 200, 0);
}

pub struct AppRenderer {
    pub canvas: Canvas<Window>,
    sprites: SpriteAtlas,
}

impl AppRenderer {
    pub fn new(canvas: Canvas<Window>) -> Result<Self, String> {
        let creator = canvas.texture_creator();
        let sprites = SpriteAtlas::load(&creator)?;
        Ok(Self { canvas, sprites })
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
        draw_traffic_lights(&mut self.canvas, &mut self.sprites, sim);
        draw_vehicles(&mut self.canvas, &mut self.sprites, sim);
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

/// Classic raised UI box (SNES / Win95 menu style).
fn draw_retro_box(canvas: &mut Canvas<Window>, x: i32, y: i32, w: u32, h: u32, fill: (u8, u8, u8)) {
    set_color(canvas, fill.0, fill.1, fill.2);
    fill_rect(canvas, x, y, w, h);
    set_color(canvas, theme::WHITE.0, theme::WHITE.1, theme::WHITE.2);
    fill_rect(canvas, x, y, w, 2);
    fill_rect(canvas, x, y, 2, h);
    set_color(canvas, theme::BLACK.0, theme::BLACK.1, theme::BLACK.2);
    fill_rect(canvas, x, y + h as i32 - 2, w, 2);
    fill_rect(canvas, x + w as i32 - 2, y, 2, h);
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
    set_color(canvas, theme::BG.0, theme::BG.1, theme::BG.2);
    fill_rect(canvas, 0, 0, config::WINDOW_WIDTH, play_h as u32);

    let step = 32i32;
    set_color(
        canvas,
        theme::GRASS_LIGHT.0,
        theme::GRASS_LIGHT.1,
        theme::GRASS_LIGHT.2,
    );
    for gy in (0..play_h).step_by(step as usize) {
        for gx in (0..config::WINDOW_WIDTH as i32).step_by(step as usize) {
            if (gx / step + gy / step) % 2 == 0 {
                fill_rect(canvas, gx + 8, gy + 8, 4, 4);
            }
        }
    }

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
        let cw = config::CROSSWALK_DEPTH as i32;
        fill_rect(
            canvas,
            enter_sb - span / 2 + offset,
            iy0 - cw,
            stripe as u32,
            cw as u32,
        );
        fill_rect(
            canvas,
            enter_nb - span / 2 + offset,
            iy1,
            stripe as u32,
            cw as u32,
        );
        fill_rect(
            canvas,
            ix0 - cw,
            enter_eb - span / 2 + offset,
            cw as u32,
            stripe as u32,
        );
        fill_rect(
            canvas,
            ix1,
            enter_wb - span / 2 + offset,
            cw as u32,
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

    set_color(
        canvas,
        theme::ASPHALT_DARK.0,
        theme::ASPHALT_DARK.1,
        theme::ASPHALT_DARK.2,
    );
    fill_rect(canvas, cx - 2, y, 4, s);
    fill_rect(canvas, x, cy - 2, s, 4);

    set_color(
        canvas,
        theme::INTERSECTION_LINE.0,
        theme::INTERSECTION_LINE.1,
        theme::INTERSECTION_LINE.2,
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(cx, y + 4),
        sdl2::rect::Point::new(cx, y + s as i32 - 4),
    );
    let _ = canvas.draw_line(
        sdl2::rect::Point::new(x + 4, cy),
        sdl2::rect::Point::new(x + s as i32 - 4, cy),
    );
}

/// Orient pole toward the intersection and red lens above green for each approach.
fn signal_sprite_rotation(lane: LaneId) -> f64 {
    match lane {
        LaneId::NorthSb => 180.0,
        LaneId::SouthNb => 0.0,
        LaneId::WestEb => 90.0,
        LaneId::EastWb => 270.0,
        _ => 0.0,
    }
}

fn draw_traffic_light_sprite(
    canvas: &mut Canvas<Window>,
    sprites: &mut SpriteAtlas,
    lane: LaneId,
    x: f32,
    y: f32,
    state: SignalState,
) {
    let texture = if state == SignalState::Green {
        &mut sprites.signal_green
    } else {
        &mut sprites.signal_red
    };
    let w = (sprites.signal_w as f32 * config::SIGNAL_DRAW_SCALE).round() as u32;
    let h = (sprites.signal_h as f32 * config::SIGNAL_DRAW_SCALE).round() as u32;
    let _ = blit_sprite_centered(
        canvas,
        texture,
        w,
        h,
        x,
        y,
        signal_sprite_rotation(lane),
        None,
    );
}

fn draw_traffic_lights(canvas: &mut Canvas<Window>, sprites: &mut SpriteAtlas, sim: &Simulation) {
    for lane_id in LaneId::ALL {
        let lane = sim.world.lane(lane_id);
        if !lane.inbound {
            continue;
        }
        let state = sim.lane_signal(lane_id);
        draw_traffic_light_sprite(
            canvas,
            sprites,
            lane_id,
            lane.light_pos.x,
            lane.light_pos.y,
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
    draw_retro_box(canvas, x, y, w as u32, h as u32, theme::LABEL_BOX);
    set_color(canvas, theme::LABEL.0, theme::LABEL.1, theme::LABEL.2);
    draw_label_word(canvas, word, x + pad_x, y + pad_y, scale);
}

fn draw_chip(canvas: &mut Canvas<Window>, label: &str, x: i32, y: i32, scale: i32) -> i32 {
    let pad_x = 8;
    let pad_y = 5;
    let w = label_width(label, scale) + pad_x * 2;
    let h = label_height(scale) + pad_y * 2;
    draw_retro_box(canvas, x, y, w as u32, h as u32, theme::HUD_BOX);
    set_color(canvas, theme::HUD_TEXT.0, theme::HUD_TEXT.1, theme::HUD_TEXT.2);
    draw_label_word(canvas, label, x + pad_x, y + pad_y, scale);
    w
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

/// SDL rotation: 0° = up; vehicle heading uses atan2 (0° = east).
fn vehicle_sprite_rotation(heading: f32) -> f64 {
    (heading + 90.0) as f64
}

fn blit_sprite_centered(
    canvas: &mut Canvas<Window>,
    texture: &mut Texture,
    dst_w: u32,
    dst_h: u32,
    cx: f32,
    cy: f32,
    angle_deg: f64,
    tint: Option<(u8, u8, u8)>,
) -> Result<(), String> {
    if let Some((r, g, b)) = tint {
        texture.set_color_mod(r, g, b);
    } else {
        texture.set_color_mod(255, 255, 255);
    }
    texture.set_alpha_mod(255);

    let dst = Rect::from_center(
        Point::new(cx as i32, cy as i32),
        dst_w,
        dst_h,
    );
    let center = Point::new(dst_w as i32 / 2, dst_h as i32 / 2);
    canvas
        .copy_ex(texture, None, dst, angle_deg, center, false, false)
        .map_err(|e| e.to_string())
}

fn draw_vehicle_sprite(
    canvas: &mut Canvas<Window>,
    sprites: &mut SpriteAtlas,
    sim: &Simulation,
    index: usize,
) {
    let v = &sim.vehicles[index];
    let pos = v.position();
    let (width, height) = v.draw_sprite_size();
    let (min_w, min_h) = match v.kind {
        VehicleKind::Car => (18.0, 28.0),
        VehicleKind::Motorcycle => (10.0, 20.0),
    };
    let w = width.max(min_w) as u32;
    let h = height.max(min_h) as u32;
    let c = v.color();
    let angle = vehicle_sprite_rotation(v.heading());
    let (body, details) = match v.kind {
        VehicleKind::Car => (&mut sprites.car_body, &mut sprites.car_details),
        VehicleKind::Motorcycle => (&mut sprites.motorcycle_body, &mut sprites.motorcycle_details),
    };

    // Drop shadow from body silhouette
    let _ = blit_sprite_centered(
        canvas,
        body,
        w,
        h,
        pos.x + 2.0,
        pos.y + 2.0,
        angle,
        Some((40, 40, 48)),
    );
    // Route-colored body panels
    let _ = blit_sprite_centered(
        canvas,
        body,
        w,
        h,
        pos.x,
        pos.y,
        angle,
        Some((c.r, c.g, c.b)),
    );
    // Untinted glass, wheels, and lights
    let _ = blit_sprite_centered(canvas, details, w, h, pos.x, pos.y, angle, None);
}

fn draw_vehicles(canvas: &mut Canvas<Window>, sprites: &mut SpriteAtlas, sim: &Simulation) {
    for i in 0..sim.vehicles.len() {
        draw_vehicle_sprite(canvas, sprites, sim, i);
    }
}

fn active_signal_label(sim: &Simulation) -> (&'static str, bool) {
    const LANES: [(LaneId, &str); 4] = [
        (LaneId::NorthSb, "NORTH IN"),
        (LaneId::SouthNb, "SOUTH IN"),
        (LaneId::WestEb, "WEST IN"),
        (LaneId::EastWb, "EAST IN"),
    ];
    for (lane, label) in LANES {
        if sim.is_green(lane) {
            return (label, true);
        }
    }
    ("ALL RED", false)
}

fn draw_signal_status(canvas: &mut Canvas<Window>, x: i32, y: i32, label: &str, green: bool) -> i32 {
    let scale = 2;
    let pad_x = 12;
    let pad_y = 8;
    let text_w = label_width(label, scale);
    let w = text_w + pad_x * 2 + 18;
    let h = label_height(scale) + pad_y * 2;

    draw_retro_box(canvas, x, y, w as u32, h as u32, theme::HUD_BOX);

    let dot_r = 4;
    let dot_x = x + pad_x;
    let dot_y = y + h / 2;
    let lamp = if green {
        theme::LAMP_GREEN
    } else {
        theme::LAMP_RED
    };
    set_color(canvas, lamp.0, lamp.1, lamp.2);
    fill_circle(canvas, dot_x, dot_y, dot_r);

    set_color(canvas, theme::HUD_TEXT.0, theme::HUD_TEXT.1, theme::HUD_TEXT.2);
    draw_label_word(canvas, label, x + pad_x + 16, y + pad_y, scale);
    w
}

fn draw_vehicle_badge(canvas: &mut Canvas<Window>, x: i32, y: i32, count: usize) -> i32 {
    let scale = 2;
    let num = count.to_string();
    let label = format!("CARS {num}");
    let pad_x = 12;
    let pad_y = 8;
    let w = label_width(&label, scale) + pad_x * 2;
    let h = label_height(scale) + pad_y * 2;
    draw_retro_box(canvas, x, y, w as u32, h as u32, theme::HUD_BOX);
    set_color(canvas, theme::HUD_TEXT.0, theme::HUD_TEXT.1, theme::HUD_TEXT.2);
    draw_label_word(canvas, &label, x + pad_x, y + pad_y, scale);
    w
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

    set_color(canvas, theme::HUD_BAR.0, theme::HUD_BAR.1, theme::HUD_BAR.2);
    fill_rect(canvas, 0, hud_y, config::WINDOW_WIDTH, 4);

    let scale = 2;
    let pad = 16;
    let row_top = hud_y + 14;
    let row_bottom = hud_y + 56;

    draw_retro_box(canvas, pad, row_top - 4, 248, 40, theme::HUD_BOX);
    set_color(canvas, theme::HUD_TEXT.0, theme::HUD_TEXT.1, theme::HUD_TEXT.2);
    draw_label_word(canvas, "TRAFFIC SIM", pad + 12, row_top + 4, scale);
    set_color(canvas, theme::HUD_MUTED.0, theme::HUD_MUTED.1, theme::HUD_MUTED.2);
    draw_label_word(canvas, "1990S MODE", pad + 12, row_top + 20, 1);

    let mut chip_x = pad + 268;
    for label in ["ARROWS", "R RANDOM", "ESC"] {
        chip_x += draw_chip(canvas, label, chip_x, row_top, scale) + 8;
    }

    let (phase_label, phase_green) = active_signal_label(sim);
    let status_x = chip_x + 8;
    let status_w = draw_signal_status(canvas, status_x, row_top, phase_label, phase_green);
    let badge_x = status_x + status_w + 8;
    let _badge_w = draw_vehicle_badge(canvas, badge_x, row_top, sim.vehicles.len());

    set_color(canvas, theme::HUD_MUTED.0, theme::HUD_MUTED.1, theme::HUD_MUTED.2);
    draw_label_word(
        canvas,
        "ARROWS=SPAWN  R=RANDOM  ESC=QUIT",
        pad + 12,
        row_bottom,
        1,
    );

    let legend_w = 310u32;
    let legend_x = config::WINDOW_WIDTH as i32 - legend_w as i32 - pad;
    draw_retro_box(canvas, legend_x, row_bottom - 6, legend_w, 32, theme::HUD_BOX);
    draw_route_legend(canvas, legend_x + 12, row_bottom);
}

fn draw_route_legend(canvas: &mut Canvas<Window>, x: i32, y: i32) {
    let scale = 1;
    let swatch = 10;
    let gap = 98;
    let labels = [
        ("LEFT", RouteType::Left),
        ("RIGHT", RouteType::Right),
        ("STRAIGHT", RouteType::Straight),
    ];

    set_color(canvas, theme::HUD_MUTED.0, theme::HUD_MUTED.1, theme::HUD_MUTED.2);
    draw_label_word(canvas, "ROUTES", x, y - 2, scale);

    for (i, (name, route)) in labels.iter().enumerate() {
        let ox = x + i as i32 * gap;
        let oy = y + 10;
        let c = route_color(*route);
        set_color(canvas, c.r, c.g, c.b);
        fill_rect(canvas, ox, oy, swatch as u32, swatch as u32);
        set_color(canvas, theme::BLACK.0, theme::BLACK.1, theme::BLACK.2);
        let _ = canvas.draw_rect(Rect::new(ox, oy, swatch as u32, swatch as u32));
        set_color(canvas, theme::HUD_TEXT.0, theme::HUD_TEXT.1, theme::HUD_TEXT.2);
        draw_label_word(canvas, name, ox + swatch + 6, oy - 1, scale);
    }
}
