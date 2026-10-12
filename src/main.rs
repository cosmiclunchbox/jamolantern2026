use macroquad::prelude::*;
use ndarray::Array2;

mod level;
use level::Level;

mod player;

mod constants;

use crate::constants::WINDOW_TITLE;
use crate::constants::LEVEL_FILE_PATH;


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
    level: Level
}

impl ActiveGameState {
    fn new() -> ActiveGameState {
        let level = Level::from_level_file(LEVEL_FILE_PATH);
        ActiveGameState {
            level
        }
    }

    fn render(&self) {
        //clear_background(BACKGROUND_COLOR);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_fps();

        self.level.render();
    }

    fn update_on_each_frame(&self) {
        // TODO
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

    fn update_on_each_frame(&self) {
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
    let game_state = GameState::start();
    loop {
        game_state.render();
        next_frame().await
    }
}
