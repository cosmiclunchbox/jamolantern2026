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
        //clear_background(BACKGROUND_COLOR);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_fps();

        self.level.render(&self.camera);
        self.player.render(&self.camera);
    }

    pub fn update_on_each_frame(&mut self) {
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            self.player.move_horizontal_clamped(PLAYER_MOVEMENT_SPEED * delta_time);
        }
        if is_key_down(KeyCode::Left) {
            self.player.move_horizontal_clamped(-PLAYER_MOVEMENT_SPEED * delta_time);
        }
        if is_key_down(KeyCode::Down) {
            self.player.move_vertical_clamped(PLAYER_MOVEMENT_SPEED * delta_time);
        }
        if is_key_down(KeyCode::Up) {
            self.player.move_vertical_clamped(-PLAYER_MOVEMENT_SPEED * delta_time);
        }
        let (player_x, player_y) = self.player.current_x_y_position_center();
        self.camera.track_target_position_clamped(player_x, player_y);
    }
}