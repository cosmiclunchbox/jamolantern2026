use macroquad::prelude::*;

mod level;
mod level_enemies;
mod hitbox;
mod iframe;
mod living_entity;
mod player;
mod enemy;
mod attack_manager;
mod knockback_manager;
mod camera;
mod constants;
mod world;
mod util;
mod menus;

use constants::WINDOW_TITLE;
use world::ActiveGameWorld;
use menus::GameOverMenu;


fn window_configuration() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: 1200,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

pub enum GameSceneTransition {
    Continue,
    ToNewGameScene,
    ToWinScene,
    ToLoseScene,
}


enum GameState {
    Playing(ActiveGameWorld),
    Win,
    Lose(GameOverMenu)
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
            GameState::Lose(m) => {
                m.render()
            }, // TODO
        }
    }

    fn update_on_each_frame(&mut self) -> GameSceneTransition {
        match self {
            GameState::Playing(s) => {
                s.update_on_each_frame()
            },
            GameState::Win => GameSceneTransition::Continue, // TODO
            GameState::Lose(_m) => GameSceneTransition::Continue, // TODO
        }
    }

    fn update_on_each_frame_and_transition_scene(mut self) -> GameState {
        match self.update_on_each_frame() {
            GameSceneTransition::Continue => self,
            GameSceneTransition::ToLoseScene => GameState::Lose(GameOverMenu::new()),
            GameSceneTransition::ToWinScene => GameState::Win,
            GameSceneTransition::ToNewGameScene => GameState::start()
        }
    }
}


#[macroquad::main(window_configuration)]
async fn main() {
    let mut game_state = GameState::start();
    loop {
        game_state = game_state.update_on_each_frame_and_transition_scene();
        game_state.render();
        next_frame().await
    }
}
