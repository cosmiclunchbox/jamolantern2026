use macroquad::prelude::*;
use crate::constants::{NUM_TILES_PER_SCREEN_WIDTH};

pub fn get_tile_size() -> f32 { screen_width() / (NUM_TILES_PER_SCREEN_WIDTH as f32) }

pub fn get_dnd_distance(pos1: (f32, f32), pos2: (f32, f32)) -> f32 {
    let horizontal_distance = (pos1.0 - pos2.0).abs();
    let vertical_distance = (pos1.1 - pos2.1).abs();
    if horizontal_distance > vertical_distance {
        horizontal_distance
    } else {
        vertical_distance
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right
}

impl MoveDirection {
    pub fn opposite(&self) -> MoveDirection {
        match self {
            MoveDirection::Up => MoveDirection::Down,
            MoveDirection::Down => MoveDirection::Up,
            MoveDirection::Left => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Left,
        }
    }
}

pub fn adjust_by_frame_rate(num: f32) -> f32 {
    let delta_time = get_frame_time().min(1.0 / 30.0);
    num * delta_time
}

pub fn coordinates_to_tile_index(x: f32, y: f32) -> (i32, i32) {
    (x.trunc() as i32, y.trunc() as i32)
}