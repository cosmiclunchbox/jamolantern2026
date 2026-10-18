use macroquad::prelude::*;

use crate::hitbox::MayHaveHitbox;

use crate::constants::{FRICTION, KNOCKBACK_FRICTION};
use crate::iframe::IFrame;
use crate::util::{MoveDirection};
use crate::knockback_manager::Knockback;


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
    facing: MoveDirection,
    health: u32,
    previous_iframe: Option<IFrame>,
    knockback_state: Option<Knockback>
}

impl Entity {
    pub fn new(
        spawn_x: f32, 
        spawn_y: f32, 
        hitbox_width: f32, 
        hitbox_height: f32, 
        leftmost_x: f32, 
        upmost_y: f32, 
        rightmost_x: f32, 
        downmost_y: f32,
        total_health: u32,
    ) -> Entity {
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
            facing: MoveDirection::Right,
            health: total_health,
            previous_iframe: None,
            knockback_state: None
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

    fn change_knockback_horizontal_velocity_with_friction(&mut self, knockback_amount: f32, being_accelerated: bool) {
        if being_accelerated {
            self.logical_x_velocity += knockback_amount;
        }
        self.logical_x_velocity *= KNOCKBACK_FRICTION;
    }

    fn change_knockback_vertical_velocity_with_friction(&mut self, knockback_amount: f32, being_accelerated: bool) {
        if being_accelerated {
            self.logical_y_velocity += knockback_amount;
        }
        self.logical_y_velocity *= KNOCKBACK_FRICTION;
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

    pub fn is_taking_damage(&self) -> bool {
        match &self.previous_iframe {
            None => false,
            Some(iframe) => iframe.is_during_damage_tick()
        }
    }

    pub fn can_take_damage(&self) -> bool {
        match &self.previous_iframe {
            None => true,
            Some(iframe) => iframe.is_expired()
        }
    }

    pub fn being_knocked_back(&self) -> bool {
        match &self.knockback_state {
            None => false,
            Some(knockback) => knockback.being_knocked_back()
        }
    }

    pub fn take_damage_if_not_iframe(&mut self, knockback: Option<Knockback>) {
        if self.can_take_damage() {
            self.take_damage();
            if let Some(knockback) = knockback {
                self.knockback_state = Some(knockback.clone());
                //self.take_knockback_set_velocity(knockback.direction, knockback.amount);
            }
        }
    }

    fn take_knockback_set_velocity(&mut self, damage_direction: MoveDirection, knockback_amount: f32, being_accelerated: bool) {
        match damage_direction {
            MoveDirection::Up => self.change_knockback_vertical_velocity_with_friction(-knockback_amount, being_accelerated),
            MoveDirection::Down => self.change_knockback_vertical_velocity_with_friction(knockback_amount, being_accelerated),
            MoveDirection::Left => self.change_knockback_horizontal_velocity_with_friction(-knockback_amount, being_accelerated),
            MoveDirection::Right => self.change_knockback_horizontal_velocity_with_friction(knockback_amount, being_accelerated),
        }
    }

    pub fn take_knockback(&mut self) {
        let Some(knockback) = &self.knockback_state else {
            return;
        };
        self.take_knockback_set_velocity(knockback.direction.clone(), knockback.amount, knockback.being_accelerated_backwards());
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