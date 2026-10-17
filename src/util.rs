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

#[derive(Clone, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right
}