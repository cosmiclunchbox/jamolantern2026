use macroquad::prelude::*;
use ndarray::Array2;

mod level;
use level::Level;

mod hitbox;

mod player;
use crate::player::Player;

mod camera;
use camera::GameCamera;

mod constants;
use crate::constants::{WINDOW_TITLE, LEVEL_FILE_PATH, PLAYER_MOVEMENT_SPEED};

mod util;


fn window_configuration() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: 1200,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}


struct ActiveGameState {
    level: Level,
    camera: GameCamera,
    player: Player
}

impl ActiveGameState {
    fn new() -> ActiveGameState {
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
        ActiveGameState {
            level,
            camera,
            player
        }
    }

    fn render(&self) {
        //clear_background(BACKGROUND_COLOR);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_fps();

        self.level.render(&self.camera);
        self.player.render(&self.camera);
    }

    fn update_on_each_frame(&mut self) {
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


enum GameState {
    Playing(ActiveGameState),
    Win,
    Lose
}

impl GameState {
    fn start() -> GameState {
        GameState::Playing(ActiveGameState::new())
    }

    fn render(&self) {
        match self {
            GameState::Playing(s) => {
                s.render()
            },
            GameState::Win => (), // TODO
            GameState::Lose => (), // TODO
        }
    }

    fn update_on_each_frame(&mut self) {
        match self {
            GameState::Playing(s) => {
                s.update_on_each_frame()
            },
            GameState::Win => (), // TODO
            GameState::Lose => (), // TODO
        }
    }
}


#[macroquad::main(window_configuration)]
async fn main() {
    let mut game_state = GameState::start();
    loop {
        game_state.update_on_each_frame();
        game_state.render();
        next_frame().await
    }
}
