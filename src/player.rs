use macroquad::prelude::*;

use crate::hitbox::MayHaveHitbox;

use crate::camera::GameCamera;

use crate::constants::{PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE, PLAYER_MAX_HEALTH, PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE};
use crate::living_entity::Entity;
use crate::util::{MoveDirection, get_tile_size};
use crate::iframe::IFrame;

// x, y are position of TOP LEFT CORNER of player hitbox
pub struct Player {
    player_entity: Entity,
    health: u32,
    previous_iframe: Option<IFrame>
}

impl Player {
    pub fn new(spawn_x: f32, spawn_y: f32, leftmost_x: f32, upmost_y: f32, rightmost_x: f32, downmost_y: f32) -> Player {
        Player {
            player_entity: Entity::new(
                spawn_x,
                spawn_y,
                PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE,
                PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE,
                leftmost_x,
                upmost_y,
                rightmost_x,
                downmost_y
            ),
            health: PLAYER_MAX_HEALTH,
            previous_iframe: None
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        let (logical_x, logical_y) = self.current_x_y_position_top_left();
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(logical_x, logical_y);
        let player_width = PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let player_height = PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let player_color = match &self.previous_iframe {
            None => BLACK,
            Some(iframe) => if iframe.is_during_damage_tick() { MAROON } else if !iframe.is_expired() { DARKGRAY } else { BLACK }
        };
        draw_rectangle(top_left_x, top_left_y, player_width, player_height, player_color);
        match self.current_facing_direction() {
            MoveDirection::Up => {
                draw_circle(top_left_x + player_width * 0.5, top_left_y + player_height * 0.25, player_width * 0.2, YELLOW);
            },
            MoveDirection::Down => {
                draw_circle(top_left_x + player_width * 0.5, top_left_y + player_height * 0.75, player_width * 0.2, YELLOW);
            }
            MoveDirection::Left => {
                draw_circle(top_left_x + player_width * 0.25, top_left_y + player_height * 0.5, player_width * 0.2, YELLOW);
            },
            MoveDirection::Right => {
                draw_circle(top_left_x + player_width * 0.75, top_left_y + player_height * 0.5, player_width * 0.2, YELLOW);
            },
        }
    }

    pub fn current_x_y_position_top_left(&self) -> (f32, f32) {
        self.player_entity.current_x_y_position_top_left()
    }

    pub fn current_x_y_position_center(&self) -> (f32, f32) {
        self.player_entity.current_x_y_position_center()
    }

    pub fn current_facing_direction(&self) -> MoveDirection {
        self.player_entity.current_facing_direction()
    }

    pub fn set_direction_based_on_velocity(&mut self) {
        self.player_entity.set_direction_based_on_velocity();
    }

    pub fn get_current_velocity(&self) -> (f32, f32) {
        self.player_entity.get_current_velocity()
    }

    pub fn change_horizontal_velocity_with_friction(&mut self, acceleration: f32) {
        self.player_entity.change_horizontal_velocity_with_friction(acceleration);
    }

    pub fn change_vertical_velocity_with_friction(&mut self, acceleration: f32) {
        self.player_entity.change_vertical_velocity_with_friction(acceleration);
    }

    pub fn move_horizontal_by_current_velocity_clamped_check_collision(
        &mut self, 
        velocity_threshold: f32,
        check_collision: impl Fn(&Entity) -> bool
    ) -> bool {
        self.player_entity.move_horizontal_by_current_velocity_clamped_check_collision(velocity_threshold, check_collision)
    }

    pub fn move_vertical_by_current_velocity_clamped_check_collision(
        &mut self, 
        velocity_threshold: f32,
        check_collision: impl Fn(&Entity) -> bool
    ) -> bool {
        self.player_entity.move_vertical_by_current_velocity_clamped_check_collision(velocity_threshold, check_collision)
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
        self.player_entity.get_hitbox()
    }
}