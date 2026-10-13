use macroquad::prelude::*;

use crate::camera::GameCamera;

use crate::constants::{PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE, PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE};
use crate::util::get_tile_size;

// x, y are position of TOP LEFT CORNER of player hitbox
pub struct Player {
    logical_x: f32,
    logical_y: f32,
}

impl Player {
    pub fn new(spawn_x: f32, spawn_y: f32) -> Player {
        Player {
            logical_x: spawn_x,
            logical_y: spawn_y
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
        (self.logical_x + PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE, self.logical_y + PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE)
    }

    pub fn move_horizontal(&mut self, distance: f32) {
        self.logical_x += distance;
    }

    pub fn move_horizontal_clamped(&mut self, distance: f32) {
        
    }

    pub fn move_vertical(&mut self, distance: f32) {
        self.logical_y += distance;
    }
}