use crate::config;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteType {
    Left,
    Right,
    Straight,
}

impl RouteType {
    pub const ALL: [RouteType; 3] = [RouteType::Left, RouteType::Right, RouteType::Straight];
}

/// Eight lanes: four road arms × two directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LaneId {
    NorthSb,
    NorthNb,
    SouthNb,
    SouthSb,
    EastWb,
    EastEb,
    WestEb,
    WestWb,
}

impl LaneId {
    pub const ALL: [LaneId; 8] = [
        LaneId::NorthSb,
        LaneId::NorthNb,
        LaneId::SouthNb,
        LaneId::SouthSb,
        LaneId::EastWb,
        LaneId::EastEb,
        LaneId::WestEb,
        LaneId::WestWb,
    ];

    pub fn index(self) -> usize {
        match self {
            LaneId::NorthSb => 0,
            LaneId::NorthNb => 1,
            LaneId::SouthNb => 2,
            LaneId::SouthSb => 3,
            LaneId::EastWb => 4,
            LaneId::EastEb => 5,
            LaneId::WestEb => 6,
            LaneId::WestWb => 7,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            LaneId::NorthSb => "N_SB",
            LaneId::NorthNb => "N_NB",
            LaneId::SouthNb => "S_NB",
            LaneId::SouthSb => "S_SB",
            LaneId::EastWb => "E_WB",
            LaneId::EastEb => "E_EB",
            LaneId::WestEb => "W_EB",
            LaneId::WestWb => "W_WB",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn route_color(route: RouteType) -> ColorRgb {
    match route {
        RouteType::Left => ColorRgb {
            r: 60,
            g: 140,
            b: 255,
        },
        RouteType::Right => ColorRgb {
            r: 255,
            g: 210,
            b: 40,
        },
        RouteType::Straight => ColorRgb {
            r: 50,
            g: 200,
            b: 90,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalState {
    Red,
    Green,
}

/// Keyboard spawn direction index: 0=↑ 1=↓ 2=→ 3=←
pub fn lane_for_spawn_direction(index: usize) -> LaneId {
    const MAP: [LaneId; 4] = [
        LaneId::SouthNb,
        LaneId::NorthSb,
        LaneId::WestEb,
        LaneId::EastWb,
    ];
    MAP.get(index).copied().unwrap_or(LaneId::SouthNb)
}

pub fn lane_capacity(lane_length: f32) -> i32 {
    let slot = config::VEHICLE_LENGTH + config::SAFETY_GAP;
    if slot <= 0.0 {
        return 0;
    }
    (lane_length / slot).floor() as i32
}
