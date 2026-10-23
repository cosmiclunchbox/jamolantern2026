mod enemy_ai;
mod enemy_anger;

use macroquad::miniquad::gl::GL_RED;
use macroquad::prelude::*;
use macroquad::rand;
//use ::rand::{random, random_range};
use time::{SignedDuration, Timestamp};

use crate::attack_manager::AttackManager;
use crate::hitbox::MayHaveHitbox;

use crate::camera::GameCamera;

use crate::level::Level;

use crate::living_entity::{Entity};
use crate::knockback_manager::Knockback;
use crate::player::Player;

use crate::constants::{ENEMY_ATTACK_COOLDOWN_MS, ENEMY_ATTACK_REACH, ENEMY_CONSIDERS_IN_LINE_MARGIN, ENEMY_DETECTION_RADIUS, ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE, ENEMY_HESITATION_CHANCE, ENEMY_HESITATION_MAX_LENGTH_MS, ENEMY_HESITATION_MIN_LENGTH_MS, ENEMY_KNOCKBACK_DURATION_MS, ENEMY_MAX_HEALTH, ENEMY_MIN_ANGER_DISTANCE, ENEMY_MIN_DISTANCE_IN_ONE_DIRECTION_FOR_HESITATION, ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE, KNOCKBACK_DURATION_MS};
use crate::resources::Resources;
use crate::util::{get_tile_size, get_dnd_distance, MoveDirection};

use enemy_ai::EnemyAngryAI;

pub use enemy_ai::{EnemyDecision};
pub use enemy_anger::{EnemyAngerLevel, EnemyType};

// x, y are position of TOP LEFT CORNER of enemy hitbox


struct EnemyHesitationManager {
    previous_decision: EnemyDecision,
    previous_decision_position: Option<(f32, f32)>,
    expires_at: Option<Timestamp>
}

impl EnemyHesitationManager {
    pub fn new() -> EnemyHesitationManager {
        EnemyHesitationManager {
            previous_decision: EnemyDecision::StayStill,
            previous_decision_position: None,
            expires_at: None
        }
    }

    pub fn randomly_hesitate_if_changing_direction_while_moving(&mut self, new_decision: EnemyDecision, current_enemy_position: (f32, f32)) {
        if self.is_hesitating() {
            return;
        }

        
        //println!("{:?} {:?}", self.previous_decision, new_decision);

        if let (
            EnemyDecision::Move(prev_direction), 
            EnemyDecision::Move(new_direction)
        ) = (&self.previous_decision, &new_decision) {
            if *prev_direction == *new_direction {
                return;
            } else {
                if let Some(previous_decision_position) = self.previous_decision_position {
                    if get_dnd_distance(previous_decision_position, current_enemy_position) > ENEMY_MIN_DISTANCE_IN_ONE_DIRECTION_FOR_HESITATION {
                        self.randomly_hesitate(new_decision.clone());
                    }
                }
            }
        }

        self.previous_decision = new_decision;
        self.previous_decision_position = Some(current_enemy_position);
    }

    fn randomly_hesitate(&mut self, new_decision: EnemyDecision) {
        let hesitation_roll = rand::gen_range(0.0, 1.0);// random::<f32>();
        //println!("{}", hesitation_roll);
        if hesitation_roll >= ENEMY_HESITATION_CHANCE {
            //println!("{:?}", hesitation_roll);
            self.previous_decision = new_decision;
            return;
        }
        
        let hesitation_time = rand::gen_range(ENEMY_HESITATION_MIN_LENGTH_MS, ENEMY_HESITATION_MAX_LENGTH_MS);
        let expires_at: Timestamp = Timestamp::now().checked_add(
            SignedDuration::milliseconds(hesitation_time as i64)
        ).expect("gonna just expect this checked add should work ig");
        self.expires_at = Some(expires_at);
    }

    pub fn is_hesitating(&self) -> bool {
        let Some(expires_at) = self.expires_at else {
            return false;
        };
        Timestamp::now() <= expires_at
    }
}

pub struct Enemy {
    enemy_entity: Entity,
    controller: EnemyAngryAI,
    disposition: EnemyType,
    anger_level: EnemyAngerLevel,
    attack_manager: AttackManager,
    hesitation_manager: EnemyHesitationManager
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
            attack_manager: AttackManager::new(ENEMY_ATTACK_COOLDOWN_MS),
            hesitation_manager: EnemyHesitationManager::new()
        }
    }

    pub fn render(&self, camera: &GameCamera, resources: &Resources) {
        let (logical_x, logical_y) = self.current_x_y_position_top_left();
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(logical_x, logical_y);
        let enemy_width = ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let enemy_height = ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * get_tile_size();
        let enemy_color = match (self.is_taking_damage(), self.can_take_damage(), &self.anger_level) {
            (true, _, _) => MAROON,
            (false, false, _) => DARKBROWN,
            (false, true, EnemyAngerLevel::Neutral) => BLUE,
            (false, true, EnemyAngerLevel::Alert) => DARKBLUE,
            (false, true, _) => Color { r: 0.5, g: 0.2, b: 1.0, a: 1.0 }
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

    pub fn take_damage_if_condition_and_not_iframe(
        &mut self,
        knockback: Option<Knockback>, 
        check_should_damage: impl Fn(&Entity) -> bool
    ) -> bool {
        if check_should_damage(&self.enemy_entity) {
            return self.take_damage_if_not_iframe(knockback);
        }
        false
    }

    fn take_damage_if_not_iframe(&mut self, knockback: Option<Knockback>) -> bool {
        self.enemy_entity.take_damage_if_not_iframe(knockback)
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

    pub fn trigger_aggro_if_alert(&mut self) {
        //println!("IM GONNA GET U");
        self.anger_level = self.anger_level.transition(false, false, true);
    }

    pub fn decide_next_action(&mut self, level: &Level, player: &Player) -> EnemyDecision {
        // yes this should probably be split into multiple functions I know

        let (neutral_to_alert, alert_to_neutral, alert_to_angry) = match &self.disposition {
            EnemyType::Idle => {
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
            EnemyType::Enraged => {
                (
                    get_dnd_distance(
                        self.current_x_y_position_center(), 
                        player.current_x_y_position_center()
                    ) < ENEMY_DETECTION_RADIUS,
                    false,
                    get_dnd_distance(
                        self.current_x_y_position_center(), 
                        player.current_x_y_position_center()
                    ) < ENEMY_DETECTION_RADIUS,
                )
            }
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
        player.take_damage_if_condition_and_not_iframe(
            Some(
                Knockback::new(self.current_facing_direction(), knockback_amount, KNOCKBACK_DURATION_MS)
            ),
            |_: &Entity| true
        );
        self.attack_manager.start_cooldown()
    }

    pub fn is_hesitating(&self) -> bool {
        self.hesitation_manager.is_hesitating()
    }

    pub fn randomly_hesitate_if_changing_direction_while_moving(&mut self, new_decision: EnemyDecision) {
        self.hesitation_manager.randomly_hesitate_if_changing_direction_while_moving(
            new_decision, 
            self.current_x_y_position_center()
        );
    }

    pub fn cancel_velocity(&mut self) {
        self.enemy_entity.cancel_velocity();
    }
}

impl MayHaveHitbox for Enemy {
    fn get_hitbox(&self) -> Option<Rect> {
        self.enemy_entity.get_hitbox()
    }
}