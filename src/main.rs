use macroquad::prelude::*;

mod level;
mod hitbox;
mod player;
mod camera;
mod constants;
mod world;
mod util;

use constants::WINDOW_TITLE;
use world::ActiveGameWorld;


fn window_configuration() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: 1200,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}


enum GameState {
    Playing(ActiveGameWorld),
    Win,
    Lose
}

impl GameState {
    fn start() -> GameState {
        GameState::Playing(ActiveGameWorld::new())
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
