use macroquad::prelude::*;

use crate::camera::GameCamera;

use crate::constants::{PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE, PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE};
use crate::util::get_tile_size;

// x, y are position of TOP LEFT CORNER of player hitbox
pub struct Player {
    logical_x: f32,
    logical_y: f32,
    tl_logical_x_bound: f32,
    tl_logical_y_bound: f32,
    br_logical_x_bound: f32,
    br_logical_y_bound: f32,
}

impl Player {
    pub fn new(spawn_x: f32, spawn_y: f32, leftmost_x: f32, upmost_y: f32, rightmost_x: f32, downmost_y: f32) -> Player {
        Player {
            logical_x: spawn_x,
            logical_y: spawn_y,
            tl_logical_x_bound: leftmost_x,
            tl_logical_y_bound: upmost_y,
            br_logical_x_bound: rightmost_x,
            br_logical_y_bound: downmost_y,
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(self.logical_x, self.logical_y);
        let player_width = PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let player_height = PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        draw_rectangle(top_left_x, top_left_y, player_width, player_height, BLACK);
    }

    pub fn current_x_y_position_top_left(&self) -> (f32, f32) {
        (self.logical_x, self.logical_y)
    }

    pub fn current_x_y_position_center(&self) -> (f32, f32) {
        (self.logical_x + PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE / 2.0, self.logical_y + PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE / 2.0)
    }

    fn clamp_player_position(&mut self) {
        self.logical_x = clamp(self.logical_x, self.tl_logical_x_bound, self.br_logical_x_bound - PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE);
        self.logical_y = clamp(self.logical_y, self.tl_logical_y_bound, self.br_logical_y_bound - PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE);
    }

    fn move_horizontal(&mut self, distance: f32) {
        self.logical_x += distance;
    }

    pub fn move_horizontal_clamped(&mut self, distance: f32) {
        self.move_horizontal(distance);
        self.clamp_player_position();
    }

    fn move_vertical(&mut self, distance: f32) {
        self.logical_y += distance;
    }

    pub fn move_vertical_clamped(&mut self, distance: f32) {
        self.move_vertical(distance);
        self.clamp_player_position();
    }
}