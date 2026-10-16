use macroquad::prelude::*;

use crate::hitbox::MayHaveHitbox;

use crate::camera::GameCamera;

use crate::constants::{FRICTION, PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE, PLAYER_MAX_HEALTH, PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE, PLAYER_MOVEMENT_SPEED_THRESHOLD};
use crate::util::get_tile_size;
use crate::iframe::IFrame;

// x, y are position of TOP LEFT CORNER of player hitbox
pub struct Player {
    logical_x: f32,
    logical_y: f32,
    tl_logical_x_bound: f32,
    tl_logical_y_bound: f32,
    br_logical_x_bound: f32,
    br_logical_y_bound: f32,
    logical_x_velocity: f32,
    logical_y_velocity: f32,
    health: u32,
    previous_iframe: Option<IFrame>
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
            logical_x_velocity: 0.0,
            logical_y_velocity: 0.0,
            health: PLAYER_MAX_HEALTH,
            previous_iframe: None
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(self.logical_x, self.logical_y);
        let player_width = PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let player_height = PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let player_color = match &self.previous_iframe {
            None => BLACK,
            Some(iframe) => if iframe.is_during_damage_tick() { MAROON } else if !iframe.is_expired() { DARKGRAY } else { BLACK }
        };
        draw_rectangle(top_left_x, top_left_y, player_width, player_height, player_color);
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

    fn move_vertical(&mut self, distance: f32) {
        self.logical_y += distance;
    }

    pub fn move_horizontal_clamped_check_collision(&mut self, distance: f32, check_collision: impl Fn(&Player) -> bool) -> bool {
        self.move_horizontal(distance);
        let colliding = check_collision(self);
        if colliding {
            self.move_horizontal(-distance);
        }
        self.clamp_player_position();
        colliding
    }

    pub fn move_vertical_clamped_check_collision(&mut self, distance: f32, check_collision: impl Fn(&Player) -> bool) -> bool {
        self.move_vertical(distance);
        let colliding = check_collision(self);
        if colliding {
            self.move_vertical(-distance);
        }
        self.clamp_player_position();
        colliding
    }


    fn set_horizontal_velocity(&mut self, velocity: f32) {
        self.logical_x_velocity = velocity;
    }

    fn set_vertical_velocity(&mut self, velocity: f32) {
        self.logical_y_velocity = velocity;
    }

    pub fn change_horizontal_velocity_with_friction(&mut self, acceleration: f32) {
        self.logical_x_velocity += acceleration;
        self.logical_x_velocity *= FRICTION;
    }

    pub fn change_vertical_velocity_with_friction(&mut self, acceleration: f32) {
        self.logical_y_velocity += acceleration;
        self.logical_y_velocity *= FRICTION;
    }


    pub fn move_horizontal_by_current_velocity_clamped_check_collision(&mut self, check_collision: impl Fn(&Player) -> bool) -> bool {
        let colliding = self.move_horizontal_clamped_check_collision(self.logical_x_velocity, check_collision);
        if colliding {
            self.set_horizontal_velocity(0.0);
        }
        colliding
    }

    pub fn move_vertical_by_current_velocity_clamped_check_collision(&mut self, check_collision: impl Fn(&Player) -> bool) -> bool {
        let colliding = self.move_vertical_clamped_check_collision(self.logical_y_velocity, check_collision);
        if colliding {
            self.set_vertical_velocity(0.0);
        }
        colliding
    }

    pub fn get_current_health(&self) -> u32 {
        self.health
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn take_damage(&mut self) {
        self.health -= 1;
        self.previous_iframe = Some(IFrame::new());
    }

    pub fn take_damage_if_not_iframe(&mut self) {
        match &self.previous_iframe {
            None => self.take_damage(),
            Some(iframe) => {
                if iframe.is_expired() {
                    self.take_damage()
                }
            }
        }
    }
}

impl MayHaveHitbox for Player {
    fn get_hitbox(&self) -> Option<Rect> {
        Some(
            Rect {
                x: self.logical_x,
                y: self.logical_y,
                w: PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE,
                h: PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE
            }
        )
    }
}