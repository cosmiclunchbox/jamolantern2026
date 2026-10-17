use macroquad::prelude::*;

use crate::hitbox::MayHaveHitbox;

use crate::constants::{FRICTION};
use crate::util::{MoveDirection};


// x, y are position of TOP LEFT CORNER of entity hitbox
pub struct Entity {
    logical_x: f32,
    logical_y: f32,
    logical_width: f32,
    logical_height: f32,
    tl_logical_x_bound: f32,
    tl_logical_y_bound: f32,
    br_logical_x_bound: f32,
    br_logical_y_bound: f32,
    logical_x_velocity: f32,
    logical_y_velocity: f32,
    facing: MoveDirection
}

impl Entity {
    pub fn new(spawn_x: f32, spawn_y: f32, hitbox_width: f32, hitbox_height: f32, leftmost_x: f32, upmost_y: f32, rightmost_x: f32, downmost_y: f32) -> Entity {
        Entity {
            logical_x: spawn_x,
            logical_y: spawn_y,
            logical_width: hitbox_width,
            logical_height: hitbox_height,
            tl_logical_x_bound: leftmost_x,
            tl_logical_y_bound: upmost_y,
            br_logical_x_bound: rightmost_x,
            br_logical_y_bound: downmost_y,
            logical_x_velocity: 0.0,
            logical_y_velocity: 0.0,
            facing: MoveDirection::Right
        }
    }

    pub fn current_x_y_position_top_left(&self) -> (f32, f32) {
        (self.logical_x, self.logical_y)
    }

    pub fn current_x_y_position_center(&self) -> (f32, f32) {
        (self.logical_x + self.logical_width / 2.0, self.logical_y + self.logical_height / 2.0)
    }

    pub fn current_facing_direction(&self) -> MoveDirection {
        self.facing.clone()
    }

    fn face_direction(&mut self, direction: MoveDirection) {
        self.facing = direction;
    }

    fn clamp_entity_position(&mut self) {
        self.logical_x = clamp(self.logical_x, self.tl_logical_x_bound, self.br_logical_x_bound - self.logical_width);
        self.logical_y = clamp(self.logical_y, self.tl_logical_y_bound, self.br_logical_y_bound - self.logical_height);
    }

    fn move_horizontal(&mut self, distance: f32) {
        self.logical_x += distance;
        //self.logical_x = (self.logical_x * 100.0).round() / 100.0;
    }

    fn move_vertical(&mut self, distance: f32) {
        self.logical_y += distance;
        //self.logical_y = (self.logical_y * 100.0).round() / 100.0;
    }

    pub fn set_direction_based_on_velocity(&mut self) {
        if self.logical_x_velocity > 0.0 {
            self.face_direction(MoveDirection::Right);
        } else if self.logical_x_velocity < 0.0 {
            self.face_direction(MoveDirection::Left);
        } else if self.logical_y_velocity > 0.0 {
            self.face_direction(MoveDirection::Down);
        } else if self.logical_y_velocity < 0.0 {
            self.face_direction(MoveDirection::Up);
        }
    }

    pub fn move_horizontal_clamped_check_collision(&mut self, distance: f32, check_collision: impl Fn(&Entity) -> bool) -> bool {
        self.move_horizontal(distance);
        let colliding = check_collision(self);
        if colliding {
            self.move_horizontal(-distance);
        }
        self.clamp_entity_position();
        colliding
        //false
    }

    pub fn move_vertical_clamped_check_collision(&mut self, distance: f32, check_collision: impl Fn(&Entity) -> bool) -> bool {
        self.move_vertical(distance);
        let colliding = check_collision(self);
        if colliding {
            self.move_vertical(-distance);
        }
        self.clamp_entity_position();
        colliding
        //false
    }

    pub fn get_current_velocity(&self) -> (f32, f32) {
        (self.logical_x_velocity, self.logical_y_velocity)
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
        check_collision: impl Fn(&Entity) -> bool
    ) -> bool {
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
        check_collision: impl Fn(&Entity) -> bool
    ) -> bool {
        if self.logical_y_velocity.abs() < velocity_threshold {
            self.set_vertical_velocity(0.0);
        }
        let colliding = self.move_vertical_clamped_check_collision(self.logical_y_velocity, check_collision);
        if colliding {
            self.set_vertical_velocity(0.0);
        }
        colliding
    }
}

impl MayHaveHitbox for Entity {
    fn get_hitbox(&self) -> Option<Rect> {
        Some(
            Rect {
                x: (self.logical_x * 100.).round() * 0.01,
                y: (self.logical_y * 100.).round() * 0.01,
                w: self.logical_width,
                h: self.logical_height
            }
        )
    }
}