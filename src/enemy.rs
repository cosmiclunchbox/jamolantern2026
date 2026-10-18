mod enemy_ai;
mod enemy_anger;

use macroquad::miniquad::gl::GL_RED;
use macroquad::prelude::*;

use crate::attack_manager::AttackManager;
use crate::hitbox::MayHaveHitbox;

use crate::camera::GameCamera;

use crate::level::Level;

use crate::living_entity::{Entity};
use crate::knockback_manager::Knockback;
use crate::player::Player;

use crate::constants::{ENEMY_ATTACK_COOLDOWN_MS, ENEMY_ATTACK_REACH, ENEMY_CONSIDERS_IN_LINE_MARGIN, ENEMY_DETECTION_RADIUS, ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE, ENEMY_MAX_HEALTH, ENEMY_MIN_ANGER_DISTANCE, ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE, FRICTION};
use crate::util::{get_tile_size, get_dnd_distance, MoveDirection};
use crate::iframe::IFrame;

use enemy_ai::EnemyAngryAI;

pub use enemy_ai::{EnemyDecision};
pub use enemy_anger::{EnemyAngerLevel, EnemyType};

// x, y are position of TOP LEFT CORNER of enemy hitbox

pub struct Enemy {
    enemy_entity: Entity,
    controller: EnemyAngryAI,
    disposition: EnemyType,
    anger_level: EnemyAngerLevel,
    attack_manager: AttackManager
}

impl Enemy {
    pub fn new(spawn_x: f32, spawn_y: f32, leftmost_x: f32, upmost_y: f32, rightmost_x: f32, downmost_y: f32, personality: EnemyType) -> Enemy {
        Enemy {
            enemy_entity: Entity::new(
                spawn_x,
                spawn_y,
                ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE,
                ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE,
                leftmost_x,
                upmost_y,
                rightmost_x,
                downmost_y,
                ENEMY_MAX_HEALTH
            ),
            controller: EnemyAngryAI::at_position(spawn_x, spawn_y),
            disposition: personality,
            anger_level: EnemyAngerLevel::Neutral,
            attack_manager: AttackManager::new(ENEMY_ATTACK_COOLDOWN_MS)
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        let (logical_x, logical_y) = self.current_x_y_position_top_left();
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(logical_x, logical_y);
        let enemy_width = ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let enemy_height = ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let enemy_color = match (self.is_taking_damage(), self.can_take_damage(), &self.anger_level) {
            (true, _, _) => MAROON,
            (false, false, _) => DARKBROWN,
            (false, true, EnemyAngerLevel::Alert) => DARKBLUE,
            (false, true, _) => BLUE
        };
        draw_rectangle(top_left_x, top_left_y, enemy_width, enemy_height, enemy_color);
        match self.current_facing_direction() {
            MoveDirection::Up => {
                draw_circle(top_left_x + enemy_width * 0.5, top_left_y + enemy_height * 0.25, enemy_width * 0.2, RED);
            },
            MoveDirection::Down => {
                draw_circle(top_left_x + enemy_width * 0.5, top_left_y + enemy_height * 0.75, enemy_width * 0.2, RED);
            }
            MoveDirection::Left => {
                draw_circle(top_left_x + enemy_width * 0.25, top_left_y + enemy_height * 0.5, enemy_width * 0.2, RED);
            },
            MoveDirection::Right => {
                draw_circle(top_left_x + enemy_width * 0.75, top_left_y + enemy_height * 0.5, enemy_width * 0.2, RED);
            },
        }
    }

    pub fn current_x_y_position_top_left(&self) -> (f32, f32) {
        self.enemy_entity.current_x_y_position_top_left()
    }

    pub fn current_x_y_position_center(&self) -> (f32, f32) {
        self.enemy_entity.current_x_y_position_center()
    }

    pub fn current_facing_direction(&self) -> MoveDirection {
        self.enemy_entity.current_facing_direction()
    }

    pub fn set_direction_based_on_velocity(&mut self) {
        self.enemy_entity.set_direction_based_on_velocity();
    }

    pub fn get_current_velocity(&self) -> (f32, f32) {
        self.enemy_entity.get_current_velocity()
    }

    pub fn change_horizontal_velocity_with_friction(&mut self, acceleration: f32) {
        self.enemy_entity.change_horizontal_velocity_with_friction(acceleration);
    }

    pub fn change_vertical_velocity_with_friction(&mut self, acceleration: f32) {
        self.enemy_entity.change_vertical_velocity_with_friction(acceleration);
    }

    pub fn take_knockback(&mut self) {
        self.enemy_entity.take_knockback()
    }


    pub fn move_horizontal_by_current_velocity_clamped_check_collision(
        &mut self, 
        velocity_threshold: f32,
        check_collision: impl Fn(&Entity) -> bool
    ) -> bool {
        //println!("{:?}", (self.logical_x_velocity, self.logical_y_velocity));
        self.enemy_entity.move_horizontal_by_current_velocity_clamped_check_collision(velocity_threshold, check_collision)
    }

    pub fn move_vertical_by_current_velocity_clamped_check_collision(
        &mut self, 
        velocity_threshold: f32,
        check_collision: impl Fn(&Entity) -> bool
    ) -> bool {
        //println!("{:?}", (self.logical_x_velocity, self.logical_y_velocity));
        self.enemy_entity.move_vertical_by_current_velocity_clamped_check_collision(velocity_threshold, check_collision)
    }

    pub fn get_current_health(&self) -> u32 {
        self.enemy_entity.get_current_health()
    }

    pub fn is_dead(&self) -> bool {
        self.enemy_entity.is_dead()
    }

    pub fn take_damage_if_not_iframe(&mut self, knockback: Option<Knockback>) {
        self.enemy_entity.take_damage_if_not_iframe(knockback);
    }

    pub fn is_taking_damage(&self) -> bool {
        self.enemy_entity.is_taking_damage()
    }

    pub fn being_knocked_back(&self) -> bool {
        self.enemy_entity.being_knocked_back()
    }

    pub fn can_take_damage(&self) -> bool {
        self.enemy_entity.can_take_damage()
    }

    pub fn get_current_target(&self) -> (i32, i32) {
        (self.controller.next_tile_index_x, self.controller.next_tile_index_y)
    }

    pub fn decide_next_action(&mut self, level: &Level, player: &Player) -> EnemyDecision {
        // yes this should probably be split into multiple functions I know

        let (neutral_to_alert, alert_to_neutral, alert_to_angry) = match &self.disposition {
            EnemyType::Neutral => {
                (
                    get_dnd_distance(
                        self.current_x_y_position_center(), 
                        player.current_x_y_position_center()
                    ) < ENEMY_DETECTION_RADIUS,
                    get_dnd_distance(
                        self.current_x_y_position_center(), 
                        player.current_x_y_position_center()
                    ) >= ENEMY_DETECTION_RADIUS,
                    false
                )
            },
            EnemyType::Impatient => {
                (
                    get_dnd_distance(
                        self.current_x_y_position_center(), 
                        player.current_x_y_position_center()
                    ) < ENEMY_DETECTION_RADIUS,
                    get_dnd_distance(
                        self.current_x_y_position_center(), 
                        player.current_x_y_position_center()
                    ) >= ENEMY_DETECTION_RADIUS,
                    (
                        player.current_x_y_position_center().0 > self.current_x_y_position_center().0 + ENEMY_MIN_ANGER_DISTANCE &&
                        player.current_x_y_position_center().1 < self.current_x_y_position_center().1 + ENEMY_CONSIDERS_IN_LINE_MARGIN &&
                        player.current_x_y_position_center().1 > self.current_x_y_position_center().1 - ENEMY_CONSIDERS_IN_LINE_MARGIN
                    )
                )
            },
        };

        self.anger_level = self.anger_level.transition(
            neutral_to_alert,
            alert_to_neutral,
            alert_to_angry
        );

        match &self.anger_level {
            EnemyAngerLevel::Angry => {
                let (logical_x, logical_y) = self.current_x_y_position_center();
                self.controller.update_current_enemy_center_position(logical_x, logical_y);
                let (player_x, player_y) = player.current_x_y_position_center();
                self.controller.set_target_location(player_x, player_y);
                self.controller.get_next_enemy_decision_in_level(level)
            },
            _ => EnemyDecision::StayStill
        }
    }

    pub fn attack_player_if_in_range_and_not_in_cooldown(&mut self, player: &mut Player, knockback_amount: f32) {
        if get_dnd_distance(self.current_x_y_position_center(), player.current_x_y_position_center()) > ENEMY_ATTACK_REACH {
            return;
        }
        if self.attack_manager.on_cooldown() {
            return;
        }
        player.take_damage_if_not_iframe(
            Some(
                Knockback::new(self.current_facing_direction(), knockback_amount)
            )
        );
        self.attack_manager.start_cooldown()
    }
}

impl MayHaveHitbox for Enemy {
    fn get_hitbox(&self) -> Option<Rect> {
        self.enemy_entity.get_hitbox()
    }
}