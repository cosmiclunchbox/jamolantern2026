use macroquad::prelude::*;

use crate::GameSceneTransition;
use crate::enemy::{Enemy, EnemyDecision, MoveDirection};
use crate::level::{Level, TouchingLevelElement};
use crate::player::Player;
use crate::camera::GameCamera;
use crate::constants::{ENEMY_MOVEMENT_SPEED, LEVEL_FILE_PATH, PLAYER_MOVEMENT_SPEED};

pub struct ActiveGameWorld {
    level: Level,
    camera: GameCamera,
    player: Player,
    enemy: Enemy
}

impl ActiveGameWorld {
    pub fn new() -> ActiveGameWorld {
        let level = Level::from_level_file(LEVEL_FILE_PATH);
        let camera = GameCamera::new(
            0.0, 
            0.0, 
            0.0, 
            0.0, 
            level.width as f32, 
            level.height as f32
        );
        let player = Player::new(
            1.0, 
            1.0,
            0.0, 
            0.0, 
            level.width as f32, 
            level.height as f32
        );
        let enemy = Enemy::new(
            15.0,
            1.0,
            0.0, 
            0.0, 
            level.width as f32, 
            level.height as f32
        );
        ActiveGameWorld {
            level,
            camera,
            player,
            enemy
        }
    }

    pub fn render(&self) {
        self.level.render(&self.camera);
        self.player.render(&self.camera);
        self.enemy.render(&self.camera);

        draw_text(String::from("FPS: ") + &get_fps().to_string(), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(String::from("HP: ") + &self.player.get_current_health().to_string(), 20.0, 100.0, 100.0, DARKGRAY);
        //draw_fps();
    }

    pub fn update_on_each_frame(&mut self) -> GameSceneTransition {
        let delta_time = get_frame_time();

        // check collision with solids function
        let check_collision_with_solid_tiles = |p: &Player| {
            let colliding_tiles = self.level.get_tiles_that_entity_is_colliding_with(p);
            let colliding_ground_tiles = colliding_tiles.iter().filter(
                |x| match x { TouchingLevelElement::Solid => true, _ => false }
            ).collect::<Vec<_>>();
            !colliding_ground_tiles.is_empty()
        };

        let player_x_acceleration = PLAYER_MOVEMENT_SPEED * delta_time * (
            (is_key_down(KeyCode::Right) as i32) as f32 - (is_key_down(KeyCode::Left) as i32) as f32
        );
        let player_y_acceleration = PLAYER_MOVEMENT_SPEED * delta_time * (
            (is_key_down(KeyCode::Down) as i32) as f32 - (is_key_down(KeyCode::Up) as i32) as f32
        );
        
        /*if is_key_down(KeyCode::Right) {
            self.player.move_horizontal_clamped_check_collision(PLAYER_MOVEMENT_SPEED * delta_time, check_collision);
        }

        if is_key_down(KeyCode::Left) {
            self.player.move_horizontal_clamped_check_collision(-PLAYER_MOVEMENT_SPEED * delta_time, check_collision);
        }

        if is_key_down(KeyCode::Down) {
            self.player.move_vertical_clamped_check_collision(PLAYER_MOVEMENT_SPEED * delta_time, check_collision);
        }

        if is_key_down(KeyCode::Up) {
            self.player.move_vertical_clamped_check_collision(-PLAYER_MOVEMENT_SPEED * delta_time, check_collision);
        }*/

        self.player.change_horizontal_velocity_with_friction(player_x_acceleration);
        self.player.move_horizontal_by_current_velocity_clamped_check_collision(check_collision_with_solid_tiles);

        self.player.change_vertical_velocity_with_friction(player_y_acceleration);
        self.player.move_vertical_by_current_velocity_clamped_check_collision(check_collision_with_solid_tiles);

        let is_player_touching_danger_tile = self.level.get_tiles_that_entity_is_colliding_with(&self.player).iter().fold(
            false, |b, x| b || match x { TouchingLevelElement::Danger => true, _ => false }
        );
        if is_player_touching_danger_tile {
            self.player.take_damage_if_not_iframe();
        }
        if self.player.is_dead() {
            return GameSceneTransition::ToLoseScene;
        }

        // enemy ai (yes I know this hsould be its own function)
        let enemy_decision = self.enemy.decide_next_action(&self.level, &self.player);
        //println!("{:?}", self.enemy.current_x_y_position_center());
        //println!("{:?}", enemy_decision);
        println!("{:?}", (self.enemy.get_current_target()));
        match enemy_decision {
            EnemyDecision::StayStill => {},
            EnemyDecision::Move(MoveDirection::Up) => {
                self.enemy.change_vertical_velocity_with_friction(-ENEMY_MOVEMENT_SPEED * delta_time);
            },
            EnemyDecision::Move(MoveDirection::Down) => {
                self.enemy.change_vertical_velocity_with_friction(ENEMY_MOVEMENT_SPEED * delta_time);
            },
            EnemyDecision::Move(MoveDirection::Left) => {
                self.enemy.change_horizontal_velocity_with_friction(-ENEMY_MOVEMENT_SPEED * delta_time);
            },
            EnemyDecision::Move(MoveDirection::Right) => {
                self.enemy.change_horizontal_velocity_with_friction(ENEMY_MOVEMENT_SPEED * delta_time);
            },
        }
        let check_enemy_collision_with_solid_tiles = |e: &Enemy| {
            let colliding_tiles = self.level.get_tiles_that_entity_is_colliding_with(e);
            let colliding_ground_tiles = colliding_tiles.iter().filter(
                |x| match x { TouchingLevelElement::Solid => true, _ => false }
            ).collect::<Vec<_>>();
            !colliding_ground_tiles.is_empty()
        };

        self.enemy.move_horizontal_by_current_velocity_clamped_check_collision(check_enemy_collision_with_solid_tiles);
        self.enemy.move_vertical_by_current_velocity_clamped_check_collision(check_enemy_collision_with_solid_tiles);

        let (player_x, player_y) = self.player.current_x_y_position_center();
        self.camera.track_target_position_clamped(player_x, player_y);

        return GameSceneTransition::Continue;
    }
}