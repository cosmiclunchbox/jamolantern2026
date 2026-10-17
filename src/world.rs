use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::ui::KeyCode::Control;

use crate::GameSceneTransition;
use crate::enemy::{Enemy, EnemyDecision, EnemyType};
use crate::level::{Level, TouchingLevelElement};
use crate::player::Player;
use crate::camera::GameCamera;
use crate::constants::{ENEMY_MOVEMENT_SPEED, ENEMY_MOVEMENT_SPEED_THRESHOLD, LEVEL_FILE_PATH, PLAYER_MOVEMENT_SPEED, PLAYER_MOVEMENT_SPEED_THRESHOLD};
use crate::util::MoveDirection;

pub struct ActiveGameWorld {
    level: Level,
    camera: GameCamera,
    player: Player,
    enemy: Enemy,
    context: ControlContext
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
            level.height as f32,
            EnemyType::Impatient
        );
        let context = ControlContext::new();
        ActiveGameWorld {
            level,
            camera,
            player,
            enemy,
            context
        }
    }

    pub fn render(&self) {
        self.level.render(&self.camera);
        self.player.render(&self.camera);
        self.enemy.render(&self.camera);

        draw_text(String::from("FPS: ") + &get_fps().to_string(), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(String::from("HP: ") + &self.player.get_current_health().to_string(), 20.0, 100.0, 100.0, DARKGRAY);
        draw_text(
            String::from("player pos: ") + &self.player.current_x_y_position_center().0.to_string() + ", " + &self.player.current_x_y_position_center().1.to_string(), 
            20.0, 120.0, 30.0, DARKGRAY);
        draw_text(
            String::from("player vel: ") + &self.player.get_current_velocity().0.to_string() + ", " + &self.player.get_current_velocity().1.to_string(), 
            20.0, 140.0, 30.0, DARKGRAY);
        //draw_fps();
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

        let mut player_x_acceleration = 0.0;
        let mut player_y_acceleration = 0.0;

        self.context.do_on_key_press(
            KeyCode::Right,
            || {},
            || { player_x_acceleration = if is_key_down(KeyCode::Left) { 0.0 } else { PLAYER_MOVEMENT_SPEED * delta_time } }
        );
        self.context.do_on_key_press(
            KeyCode::Left,
            || {},
            || { player_x_acceleration = if is_key_down(KeyCode::Right) { 0.0 } else { -PLAYER_MOVEMENT_SPEED * delta_time } }
        );
        self.context.do_on_key_press(
            KeyCode::Down,
            || {},
            || { player_y_acceleration = if is_key_down(KeyCode::Up) { 0.0 } else { PLAYER_MOVEMENT_SPEED * delta_time } }
        );
        self.context.do_on_key_press(
            KeyCode::Up,
            || {},
            || { player_y_acceleration = if is_key_down(KeyCode::Down) { 0.0 } else { -PLAYER_MOVEMENT_SPEED * delta_time } }
        );
        
        /*if is_key_down(KeyCode::Right) {
            player_x_acceleration = PLAYER_MOVEMENT_SPEED * delta_time;
            self.player.face_direction(MoveDirection::Right);
        }

        if is_key_down(KeyCode::Left) {
            player_x_acceleration = -PLAYER_MOVEMENT_SPEED * delta_time;
            self.player.face_direction(MoveDirection::Left);
        }

        if is_key_down(KeyCode::Down) {
            player_y_acceleration = PLAYER_MOVEMENT_SPEED * delta_time;
            self.player.face_direction(MoveDirection::Down);
        }

        if is_key_down(KeyCode::Up) {
            player_y_acceleration = -PLAYER_MOVEMENT_SPEED * delta_time;
            self.player.face_direction(MoveDirection::Up);
        }*/

        self.player.change_horizontal_velocity_with_friction(player_x_acceleration);
        self.player.move_horizontal_by_current_velocity_clamped_check_collision(
            PLAYER_MOVEMENT_SPEED_THRESHOLD * delta_time,
            check_collision_with_solid_tiles
        );

        self.player.change_vertical_velocity_with_friction(player_y_acceleration);
        self.player.move_vertical_by_current_velocity_clamped_check_collision(
            PLAYER_MOVEMENT_SPEED_THRESHOLD * delta_time,
            check_collision_with_solid_tiles
        );

        self.player.set_direction_based_on_velocity();

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
        println!("{:?}", enemy_decision);
        //println!("{:?}", (self.enemy.get_current_target()));
        match enemy_decision {
            EnemyDecision::StayStill => {
                self.enemy.change_vertical_velocity_with_friction(0.0);
                self.enemy.change_horizontal_velocity_with_friction(0.0);
            },
            EnemyDecision::Move(MoveDirection::Up) => {
                self.enemy.change_vertical_velocity_with_friction(-ENEMY_MOVEMENT_SPEED * delta_time);
                self.enemy.change_horizontal_velocity_with_friction(0.0);
            },
            EnemyDecision::Move(MoveDirection::Down) => {
                self.enemy.change_vertical_velocity_with_friction(ENEMY_MOVEMENT_SPEED * delta_time);
                self.enemy.change_horizontal_velocity_with_friction(0.0);
            },
            EnemyDecision::Move(MoveDirection::Left) => {
                self.enemy.change_vertical_velocity_with_friction(0.0);
                self.enemy.change_horizontal_velocity_with_friction(-ENEMY_MOVEMENT_SPEED * delta_time);
            },
            EnemyDecision::Move(MoveDirection::Right) => {
                self.enemy.change_vertical_velocity_with_friction(0.0);
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

        self.enemy.move_horizontal_by_current_velocity_clamped_check_collision(
            ENEMY_MOVEMENT_SPEED_THRESHOLD * delta_time,
            check_enemy_collision_with_solid_tiles
        );
        self.enemy.move_vertical_by_current_velocity_clamped_check_collision(
            ENEMY_MOVEMENT_SPEED_THRESHOLD * delta_time,
            check_enemy_collision_with_solid_tiles
        );

        let (player_x, player_y) = self.player.current_x_y_position_center();
        self.camera.track_target_position_clamped(player_x, player_y);

        return GameSceneTransition::Continue;
    }
}


struct ControlContext {
    keys_pressed_down: HashMap<KeyCode, bool>
}

impl ControlContext {
    fn new() -> ControlContext {
        let mut keys_pressed_down = HashMap::new();
        keys_pressed_down.insert(KeyCode::Right, false);
        keys_pressed_down.insert(KeyCode::Left, false);
        keys_pressed_down.insert(KeyCode::Up, false);
        keys_pressed_down.insert(KeyCode::Down, false);
        ControlContext {
            keys_pressed_down
        }
    }

    fn do_on_key_press(
        &mut self, 
        key: KeyCode, 
        mut when_first_pressed: impl FnMut() -> (),
        mut when_held: impl FnMut() -> ()
    ) {
        if is_key_down(key) {
            let Some(is_pressed) = self.keys_pressed_down.get(&key) else {
                panic!("attempted to check for key press that is not stored in context 2shkldsfl");
                return
            };
            if !is_pressed {
                when_first_pressed();
                self.keys_pressed_down.insert(key.clone(), true);
            }
            when_held();
        } else {
            self.keys_pressed_down.insert(key.clone(), false);
        }
    }
}