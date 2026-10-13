use macroquad::prelude::*;
use crate::constants::{NUM_TILES_PER_SCREEN_WIDTH};

pub fn get_tile_size() -> f32 { screen_width() / (NUM_TILES_PER_SCREEN_WIDTH as f32) }