use macroquad::prelude::*;

use crate::attack_manager::AttackManager;
use crate::enemy::Enemy;
use crate::hitbox::MayHaveHitbox;

use crate::camera::GameCamera;

use crate::constants::{PLAYER_ATTACK_COOLDOWN_MS, PLAYER_ATTACK_WIDTH, PLAYER_ATTACK_REACH, PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE, PLAYER_MAX_HEALTH, PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE};
use crate::living_entity::{Entity};
use crate::knockback_manager::Knockback;
use crate::util::{MoveDirection, coordinates_to_tile_index, get_tile_size};

// x, y are position of TOP LEFT CORNER of player hitbox
pub struct Player {
    player_entity: Entity,
    attack_manager: AttackManager
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
                downmost_y,
                PLAYER_MAX_HEALTH
            ),
            attack_manager: AttackManager::new(PLAYER_ATTACK_COOLDOWN_MS)
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        let (logical_x, logical_y) = self.current_x_y_position_top_left();
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(logical_x, logical_y);
        let player_width = PLAYER_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let player_height = PLAYER_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let player_color = if self.is_taking_damage() {
            MAROON
        } else if !self.can_take_damage() {
            DARKGRAY
        } else {
            BLACK
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

    pub fn take_knockback(&mut self) {
        self.player_entity.take_knockback()
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
        self.player_entity.get_current_health()
    }

    pub fn is_dead(&self) -> bool {
        self.player_entity.is_dead()
    }

    pub fn take_damage_if_not_iframe(&mut self, knockback: Option<Knockback>) {
        self.player_entity.take_damage_if_not_iframe(knockback);
    }

    pub fn is_taking_damage(&self) -> bool {
        self.player_entity.is_taking_damage()
    }

    pub fn being_knocked_back(&self) -> bool {
        self.player_entity.being_knocked_back()
    }

    pub fn can_take_damage(&self) -> bool {
        self.player_entity.can_take_damage()
    }

    pub fn attack_enemy_if_in_range(&mut self, enemy: &mut Enemy, knockback_amount: f32) {
        let (player_center_x, player_center_y) = self.current_x_y_position_center();
        let (enemy_center_x, enemy_center_y) = enemy.current_x_y_position_center();
        match self.current_facing_direction() {
            MoveDirection::Up => {
                if enemy_center_y > player_center_y { // below player
                    return;
                }
                if enemy_center_y < player_center_y - PLAYER_ATTACK_REACH { // too far above player
                    return;
                }
                if (enemy_center_x - player_center_x).abs() > PLAYER_ATTACK_WIDTH {
                    return;
                }
            },
            MoveDirection::Down => {
                if enemy_center_y < player_center_y { // above player
                    return;
                }
                if enemy_center_y > player_center_y + PLAYER_ATTACK_REACH { // too far below player
                    return;
                }
                if (enemy_center_x - player_center_x).abs() > PLAYER_ATTACK_WIDTH {
                    return;
                }
            },
            MoveDirection::Left => {
                if enemy_center_x > player_center_x {
                    return;
                }
                if enemy_center_x < player_center_x - PLAYER_ATTACK_REACH {
                    return;
                }
                if (enemy_center_y - player_center_y).abs() > PLAYER_ATTACK_WIDTH {
                    return;
                }
            },
            MoveDirection::Right => {
                if enemy_center_x < player_center_x {
                    return;
                }
                if enemy_center_x > player_center_x + PLAYER_ATTACK_REACH {
                    return;
                }
                if (enemy_center_y - player_center_y).abs() > PLAYER_ATTACK_WIDTH {
                    return;
                }
            },
        }

        enemy.take_damage_if_not_iframe(
            Some(
                Knockback::new(self.current_facing_direction(), knockback_amount)
            )
        );
        self.attack_manager.start_cooldown()
    }
}

impl MayHaveHitbox for Player {
    fn get_hitbox(&self) -> Option<Rect> {
        self.player_entity.get_hitbox()
    }
}