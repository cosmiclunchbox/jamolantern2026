use macroquad::prelude::*;
use crate::constants::{DRAW_LEVEL_OFFSET_PIXELS_X, DRAW_LEVEL_OFFSET_PIXELS_Y, NUM_TILES_PER_SCREEN_WIDTH};
use crate::util::get_tile_size;

pub struct GameCamera {
    center_logical_x: f32,
    center_logical_y: f32,
    tl_logical_x_bound: f32,
    tl_logical_y_bound: f32,
    br_logical_x_bound: f32,
    br_logical_y_bound: f32,
    logical_width: f32,
    logical_height: f32
}

impl GameCamera {
    pub fn new(starting_x: f32, starting_y: f32, leftmost_x: f32, upmost_y: f32, rightmost_x: f32, downmost_y: f32) -> GameCamera {
        let logical_width = NUM_TILES_PER_SCREEN_WIDTH as f32;
        let logical_height = screen_height() / screen_width() * (NUM_TILES_PER_SCREEN_WIDTH as f32);
        let (center_logical_x, center_logical_y) = GameCamera::clamp_center_position_within_bounds(
            starting_x, 
            starting_y, 
            leftmost_x, 
            upmost_y, 
            rightmost_x,
            downmost_y,
            logical_width,
            logical_height
        );
        //println!("HIHIHIHIHIHIHHHHIIHHIHIHII {} {}", center_logical_x, center_logical_y);
        GameCamera {
            center_logical_x,
            center_logical_y,
            tl_logical_x_bound: leftmost_x,
            tl_logical_y_bound: upmost_y,
            br_logical_x_bound: rightmost_x,
            br_logical_y_bound: downmost_y,
            logical_width,
            logical_height
        }
    }

    fn clamp_center_position_within_bounds(
        center_x: f32, 
        center_y: f32, 
        leftmost_x: f32, 
        upmost_y: f32, 
        rightmost_x: f32, 
        downmost_y: f32,
        window_width: f32,
        window_height: f32
    ) -> (f32, f32) {
        let center_x_clamped = clamp(center_x, leftmost_x + window_width / 2.0, rightmost_x - window_width / 2.0);
        let center_y_clamped = clamp(center_y, upmost_y + window_height / 2.0, downmost_y - window_height / 2.0);
        (center_x_clamped, center_y_clamped)
    }

    pub fn track_target_position_clamped(&mut self, target_x: f32, target_y: f32) {
        let (center_logical_x, center_logical_y) = GameCamera::clamp_center_position_within_bounds(
            target_x, 
            target_y, 
            self.tl_logical_x_bound, 
            self.tl_logical_y_bound, 
            self.br_logical_x_bound,
            self.br_logical_y_bound,
            self.logical_width,
            self.logical_height
        );
        self.center_logical_x = center_logical_x;
        self.center_logical_y = center_logical_y;
    }

    pub fn get_screen_position_for_logical_coordinates(&self, x: f32, y: f32) -> (f32, f32) {
        let tile_size = get_tile_size();
        (
            (x - (self.center_logical_x - self.logical_width / 2.0)) * tile_size + DRAW_LEVEL_OFFSET_PIXELS_X, 
            (y - (self.center_logical_y - self.logical_height / 2.0)) * tile_size + DRAW_LEVEL_OFFSET_PIXELS_Y
        )
    }
}