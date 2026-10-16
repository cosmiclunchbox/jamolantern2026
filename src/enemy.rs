mod enemy_ai;
mod enemy_anger;

use macroquad::prelude::*;

use crate::hitbox::MayHaveHitbox;

use crate::camera::GameCamera;

use crate::level::Level;

use crate::player::Player;

use crate::constants::{ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE, ENEMY_MAX_HEALTH, ENEMY_MOVEMENT_SPEED, ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE, FRICTION};
use crate::util::get_tile_size;
use crate::iframe::IFrame;

use enemy_ai::EnemyAI;

pub use enemy_ai::{EnemyDecision, MoveDirection};

// x, y are position of TOP LEFT CORNER of enemy hitbox
#[derive(Debug)]
pub struct Enemy {
    logical_x: f32,
    logical_y: f32,
    tl_logical_x_bound: f32,
    tl_logical_y_bound: f32,
    br_logical_x_bound: f32,
    br_logical_y_bound: f32,
    logical_x_velocity: f32,
    logical_y_velocity: f32,
    health: u32,
    previous_iframe: Option<IFrame>,
    controller: EnemyAI
}

impl Enemy {
    pub fn new(spawn_x: f32, spawn_y: f32, leftmost_x: f32, upmost_y: f32, rightmost_x: f32, downmost_y: f32) -> Enemy {
        Enemy {
            logical_x: spawn_x,
            logical_y: spawn_y,
            tl_logical_x_bound: leftmost_x,
            tl_logical_y_bound: upmost_y,
            br_logical_x_bound: rightmost_x,
            br_logical_y_bound: downmost_y,
            logical_x_velocity: 0.0,
            logical_y_velocity: 0.0,
            health: ENEMY_MAX_HEALTH,
            previous_iframe: None,
            controller: EnemyAI::at_position(spawn_x, spawn_y)
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(self.logical_x, self.logical_y);
        let enemy_width = ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let enemy_height = ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let enemy_color = match &self.previous_iframe {
            None => BLUE,
            Some(iframe) => if iframe.is_during_damage_tick() { MAROON } else if !iframe.is_expired() { DARKBROWN } else { BLUE }
        };
        draw_rectangle(top_left_x, top_left_y, enemy_width, enemy_height, enemy_color);
    }

    pub fn current_x_y_position_top_left(&self) -> (f32, f32) {
        (self.logical_x, self.logical_y)
    }

    pub fn current_x_y_position_center(&self) -> (f32, f32) {
        (self.logical_x + ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE / 2.0, self.logical_y + ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE / 2.0)
    }

    fn clamp_enemy_position(&mut self) {
        self.logical_x = clamp(self.logical_x, self.tl_logical_x_bound, self.br_logical_x_bound - ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE);
        self.logical_y = clamp(self.logical_y, self.tl_logical_y_bound, self.br_logical_y_bound - ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE);
    }

    fn move_horizontal(&mut self, distance: f32) {
        self.logical_x += distance;
    }

    fn move_vertical(&mut self, distance: f32) {
        self.logical_y += distance;
    }

    pub fn move_horizontal_clamped_check_collision(&mut self, distance: f32, check_collision: impl Fn(&Enemy) -> bool) -> bool {
        self.move_horizontal(distance);
        let colliding = check_collision(self);
        //println!("{}", colliding);
        if colliding {
            self.move_horizontal(-distance);
        }
        self.clamp_enemy_position();
        colliding
    }

    pub fn move_vertical_clamped_check_collision(&mut self, distance: f32, check_collision: impl Fn(&Enemy) -> bool) -> bool {
        self.move_vertical(distance);
        let colliding = check_collision(self);
        //println!("{}", colliding);
        if colliding {
            self.move_vertical(-distance);
        }
        self.clamp_enemy_position();
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


    pub fn move_horizontal_by_current_velocity_clamped_check_collision(
        &mut self, 
        velocity_threshold: f32,
        check_collision: impl Fn(&Enemy) -> bool
    ) -> bool {
        //println!("{:?}", (self.logical_x_velocity, self.logical_y_velocity));
        if self.logical_x_velocity.abs() < velocity_threshold {
            self.set_horizontal_velocity(0.0);
        }
        let colliding = self.move_horizontal_clamped_check_collision(self.logical_x_velocity, check_collision);
        if colliding {
            self.set_horizontal_velocity(0.0);
        }
        colliding
    }

    pub fn move_vertical_by_current_velocity_clamped_check_collision(
        &mut self, 
        velocity_threshold: f32,
        check_collision: impl Fn(&Enemy) -> bool
    ) -> bool {
        //println!("{:?}", (self.logical_x_velocity, self.logical_y_velocity));
        if self.logical_y_velocity.abs() < velocity_threshold {
            self.set_vertical_velocity(0.0);
        }
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

    pub fn get_current_target(&self) -> (i32, i32) {
        (self.controller.next_tile_index_x, self.controller.next_tile_index_y)
    }

    pub fn decide_next_action(&mut self, level: &Level, player: &Player) -> EnemyDecision {
        self.controller.update_current_enemy_center_position(
            self.logical_x + ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * 0.5,
            self.logical_y + ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * 0.5
        );
        let (player_x, player_y) = player.current_x_y_position_center();
        self.controller.set_target_location(player_x, player_y);
        self.controller.get_next_enemy_direction_in_level(level)
    }
}

impl MayHaveHitbox for Enemy {
    fn get_hitbox(&self) -> Option<Rect> {
        Some(
            Rect {
                x: (self.logical_x * 100.).round() * 0.01,
                y: (self.logical_y * 100.).round() * 0.01,
                w: ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE,
                h: ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE
            }
        )
    }
}