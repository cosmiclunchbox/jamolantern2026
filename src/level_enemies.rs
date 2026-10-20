use std::fs;

use crate::{camera::GameCamera, constants::ENEMY_SPAWN_OFFSET, enemy::{Enemy, EnemyDecision, EnemyType}, knockback_manager::Knockback, level::Level, living_entity::Entity, player::Player, util::MoveDirection};

pub struct EnemyLayout {
    enemies: Vec<Enemy>
}

impl EnemyLayout {
    pub fn from_enemy_file_with_level_dimensions(file_path: &str, level_width: f32, level_height: f32) -> EnemyLayout {
        let mut enemies_vec = Vec::new();
        
        let enemies_str = fs::read_to_string(file_path).expect("Couldn't read enemies from file path.");
        let enemies_str_rows = enemies_str.split("\n");

        for enemy_row in enemies_str_rows {
            let enemy_row_items = enemy_row.split(" ").collect::<Vec<_>>();
            if enemy_row_items.is_empty() {
                continue;
            }
            if enemy_row_items.get(0).expect("non empty row").starts_with("//") {
                continue;
            }
            assert!(enemy_row_items.len() == 3);
            let enemy_x = enemy_row_items.get(0)
                .expect("should be an x position for enemy")
                .parse::<u32>()
                .expect("enemy x position should be positive int") as f32;
            let enemy_y = enemy_row_items.get(1)
                .expect("should be an y position for enemy")
                .parse::<u32>()
                .expect("enemy y position should be positive int") as f32;
            let enemy_type = EnemyType::from_numeric_id(
                enemy_row_items.get(2)
                    .expect("third item should be enemy type id")
                    .parse::<u32>()
                    .expect("enemy type id should be unsigned int")
            );
            let enemy = Enemy::new(
                enemy_x + ENEMY_SPAWN_OFFSET,
                enemy_y + ENEMY_SPAWN_OFFSET,
                0.0, 
                0.0, 
                level_width, 
                level_height,
                enemy_type
            );
            enemies_vec.push(enemy);
        }

        EnemyLayout {
            enemies: enemies_vec
        }
    }

    pub fn do_action_to_all_enemies(
        &mut self,
        act_on_enemy: &mut impl FnMut(&mut Enemy)
    ) {
        for enemy in &mut self.enemies {
            act_on_enemy(enemy);
        }
    }

    pub fn render_all(&self, camera: &GameCamera) {
        for enemy in &self.enemies {
            enemy.render(camera);
        }
    }

    pub fn adjust_all_enemy_positions_by_their_current_velocities_with_collision(
        &mut self,
        velocity_threshold: f32,
        check_collision: &impl Fn(&Entity) -> bool
    ) {
        for enemy in &mut self.enemies {
            enemy.move_horizontal_by_current_velocity_clamped_check_collision(
                velocity_threshold,
                check_collision
            );
            enemy.move_vertical_by_current_velocity_clamped_check_collision(
                velocity_threshold,
                check_collision
            );

            if !enemy.being_knocked_back() {
                enemy.set_direction_based_on_velocity();
            }
        }
    }

    pub fn damage_each_enemy_if_condition_and_not_iframe(
        &mut self,
        knockback: Option<Knockback>,
        check_should_damage: &impl Fn(&Entity) -> bool
    ) {
        for enemy in &mut self.enemies {
            enemy.take_damage_if_condition_and_not_iframe(knockback.clone(), check_should_damage);
        }
    }

    pub fn make_each_enemy_act_based_on_current_world_state(
        &mut self,
        level: &Level,
        player: &mut Player,
        acceleration: f32,
        knockback_amount: f32
    ) {
        for enemy in &mut self.enemies {
            if enemy.being_knocked_back() {
                enemy.take_knockback();

            } else if enemy.is_hesitating() {
                enemy.change_vertical_velocity_with_friction(0.0);
                enemy.change_horizontal_velocity_with_friction(0.0);

            } else {
                // enemy ai (yes I know this hsould be its own function)
                let enemy_decision = enemy.decide_next_action(level, player);
                //println!("{:?}", enemy.current_x_y_position_center());
                //println!("{:?}", enemy_decision);
                //println!("{:?}", (enemy.get_current_target()));
                match enemy_decision {
                    EnemyDecision::StayStill => {
                        enemy.change_vertical_velocity_with_friction(0.0);
                        enemy.change_horizontal_velocity_with_friction(0.0);
                    },
                    EnemyDecision::AttackPlayer => {
                        enemy.attack_player_if_in_range_and_not_in_cooldown(player, knockback_amount);
                        enemy.change_vertical_velocity_with_friction(0.0);
                        enemy.change_horizontal_velocity_with_friction(0.0);
                    }
                    EnemyDecision::Move(MoveDirection::Up) => {
                        enemy.change_vertical_velocity_with_friction(-acceleration);
                        enemy.change_horizontal_velocity_with_friction(0.0);
                    },
                    EnemyDecision::Move(MoveDirection::Down) => {
                        enemy.change_vertical_velocity_with_friction(acceleration);
                        enemy.change_horizontal_velocity_with_friction(0.0);
                    },
                    EnemyDecision::Move(MoveDirection::Left) => {
                        enemy.change_vertical_velocity_with_friction(0.0);
                        enemy.change_horizontal_velocity_with_friction(-acceleration);
                    },
                    EnemyDecision::Move(MoveDirection::Right) => {
                        enemy.change_vertical_velocity_with_friction(0.0);
                        enemy.change_horizontal_velocity_with_friction(acceleration);
                    },
                }

                enemy.randomly_hesitate_if_changing_direction_while_moving(enemy_decision);
            }
        }
    }
}