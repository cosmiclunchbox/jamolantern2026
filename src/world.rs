use macroquad::prelude::*;

use crate::level::Level;
use crate::player::Player;
use crate::camera::GameCamera;
use crate::constants::{LEVEL_FILE_PATH, PLAYER_MOVEMENT_SPEED};

pub struct ActiveGameWorld {
    level: Level,
    camera: GameCamera,
    player: Player
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
        ActiveGameWorld {
            level,
            camera,
            player
        }
    }

    pub fn render(&self) {
        self.level.render(&self.camera);
        self.player.render(&self.camera);

        draw_text(String::from("FPS: ") + &get_fps().to_string(), 20.0, 20.0, 30.0, DARKGRAY);
        //draw_fps();
    }

    pub fn update_on_each_frame(&mut self) {
        let delta_time = get_frame_time();
        let check_collision = |p: &Player| {
            let colliding_tiles = self.level.get_tiles_that_entity_is_colliding_with(p);
            //println!("{:?}", colliding_tiles);
            !colliding_tiles.is_empty()
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
        self.player.move_horizontal_by_current_velocity_clamped_check_collision(check_collision);

        self.player.change_vertical_velocity_with_friction(player_y_acceleration);
        self.player.move_vertical_by_current_velocity_clamped_check_collision(check_collision);

        let (player_x, player_y) = self.player.current_x_y_position_center();
        self.camera.track_target_position_clamped(player_x, player_y);
    }
}