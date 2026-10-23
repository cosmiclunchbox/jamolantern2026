use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::ui::KeyCode::Control;

use crate::GameSceneTransition;
use crate::enemy::{Enemy, EnemyDecision, EnemyType};
use crate::hitbox::MayHaveHitbox;
use crate::level::{Level, TouchingLevelElement};
use crate::level_enemies::EnemyLayout;
use crate::living_entity::{Entity};
use crate::knockback_manager::Knockback;
use crate::player::Player;
use crate::camera::GameCamera;
use crate::constants::{ENEMIES_FILE_PATH, ENEMY_KNOCKBACK_AMOUNT, ENEMY_MOVEMENT_SPEED, ENEMY_MOVEMENT_SPEED_THRESHOLD, KNOCKBACK_AMOUNT, LEVEL_FILE_PATH, PLAYER_MOVEMENT_SPEED, PLAYER_MOVEMENT_SPEED_THRESHOLD};
use crate::resources::Resources;
use crate::util::MoveDirection;

pub struct ActiveGameWorld {
    level: Level,
    camera: GameCamera,
    player: Player,
    //enemy: Enemy,
    enemies: EnemyLayout,
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
            2.0, 
            7.0,
            0.0, 
            0.0, 
            level.width as f32, 
            level.height as f32
        );
        /*let player = Player::new(
            45.0, 
            19.0,
            0.0, 
            0.0, 
            level.width as f32, 
            level.height as f32
        );*/
        let enemies = EnemyLayout::from_enemy_file_with_level_dimensions(
            ENEMIES_FILE_PATH,
            level.width as f32,
            level.height as f32
        );
        let context = ControlContext::new();
        ActiveGameWorld {
            level,
            camera,
            player,
            enemies,
            context
        }
    }

    pub fn render(&self, resources: &Resources) {
        self.level.render(&self.camera, resources);
        self.player.render(&self.camera, resources);
        self.enemies.render_all(&self.camera, resources);

        draw_text(String::from("FPS: ") + &get_fps().to_string(), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(String::from("HP:") + &self.player.get_current_health().to_string(), screen_width() - 200.0, 80.0, 100.0, BLACK);
        /*draw_text(
            String::from("player pos: ") + &self.player.current_x_y_position_center().0.to_string() + ", " + &self.player.current_x_y_position_center().1.to_string(), 
            20.0, 120.0, 30.0, DARKGRAY);
        draw_text(
            String::from("player vel: ") + &self.player.get_current_velocity().0.to_string() + ", " + &self.player.get_current_velocity().1.to_string(), 
            20.0, 140.0, 30.0, DARKGRAY);*/
        //draw_fps();
        //draw_fps();
    }

    pub fn update_on_each_frame(&mut self) -> GameSceneTransition {
        let delta_time = get_frame_time().min(1.0 / 30.0);

        // check collision with solids function
        let check_collision_with_solid_tiles = |e: &Entity| {
            let colliding_tiles = self.level.get_tiles_that_entity_is_colliding_with(e);
            let colliding_ground_tiles = colliding_tiles.iter().filter(
                |x| match x { TouchingLevelElement::Solid => true, _ => false }
            ).collect::<Vec<_>>();
            !colliding_ground_tiles.is_empty()
        };

        let is_touching_danger_tile = |e: &Entity| {
            self.level.get_tiles_that_entity_is_colliding_with(e).iter().fold(
                false, |b, x| b || match x { TouchingLevelElement::Danger => true, _ => false }
            )
        };

        let is_touching_win_tile = |e: &Player| {
            self.level.get_tiles_that_entity_is_colliding_with(e).iter().fold(
                false, |b, x| b || match x { TouchingLevelElement::Win => true, _ => false }
            )
        };

        let mut player_x_acceleration = 0.0;
        let mut player_y_acceleration = 0.0;

        self.context.do_on_key_press(
            KeyCode::D,
            || {},
            || { player_x_acceleration = if is_key_down(KeyCode::A) { 0.0 } else { PLAYER_MOVEMENT_SPEED * delta_time } }
        );
        self.context.do_on_key_press(
            KeyCode::A,
            || {},
            || { player_x_acceleration = if is_key_down(KeyCode::D) { 0.0 } else { -PLAYER_MOVEMENT_SPEED * delta_time } }
        );
        self.context.do_on_key_press(
            KeyCode::S,
            || {},
            || { player_y_acceleration = if is_key_down(KeyCode::W) { 0.0 } else { PLAYER_MOVEMENT_SPEED * delta_time } }
        );
        self.context.do_on_key_press(
            KeyCode::W,
            || {},
            || { player_y_acceleration = if is_key_down(KeyCode::S) { 0.0 } else { -PLAYER_MOVEMENT_SPEED * delta_time } }
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

        if self.player.being_knocked_back() {
            self.player.take_knockback();
        } else {
            self.player.change_horizontal_velocity_with_friction(player_x_acceleration);
            self.player.change_vertical_velocity_with_friction(player_y_acceleration);
        }

        self.player.move_horizontal_by_current_velocity_clamped_check_collision(
            PLAYER_MOVEMENT_SPEED_THRESHOLD * delta_time,
            check_collision_with_solid_tiles
        );
        self.player.move_vertical_by_current_velocity_clamped_check_collision(
            PLAYER_MOVEMENT_SPEED_THRESHOLD * delta_time,
            check_collision_with_solid_tiles
        );

        if !self.player.being_knocked_back() {
            self.player.set_direction_based_on_velocity();
        }

        self.player.take_damage_if_condition_and_not_iframe(None, is_touching_danger_tile);
        if self.player.is_dead() {
            return GameSceneTransition::ToLoseScene;
        }

        if is_touching_win_tile(&self.player) {
            return GameSceneTransition::ToWinScene;
        }

        self.context.do_on_key_press(
            KeyCode::Space,
            || { self.player.attack_enemies_if_in_range(&mut self.enemies, ENEMY_KNOCKBACK_AMOUNT * delta_time); },
            || {}
        );

        self.enemies.make_each_enemy_act_based_on_current_world_state(
            &self.level, 
            &mut self.player, 
            ENEMY_MOVEMENT_SPEED * delta_time,
            KNOCKBACK_AMOUNT * delta_time
        );

        self.enemies.adjust_all_enemy_positions_by_their_current_velocities_with_collision(
            ENEMY_MOVEMENT_SPEED_THRESHOLD * delta_time,
            &check_collision_with_solid_tiles
        );

        self.enemies.damage_each_enemy_if_condition_and_not_iframe(None, &is_touching_danger_tile);

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