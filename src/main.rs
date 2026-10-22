use macroquad::prelude::*;

mod resources;
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

use crate::{menus::YouWinMenu, resources::Resources};


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
    Win(YouWinMenu),
    Lose(GameOverMenu)
}

impl GameState {
    fn start() -> GameState {
        GameState::Playing(ActiveGameWorld::new())
    }

    fn render(&self, resources: &Resources) {
        match self {
            GameState::Playing(s) => {
                s.render(resources)
            },
            GameState::Win(w) => {
                w.render()
            }, // TODO
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
            GameState::Win(_w) => GameSceneTransition::Continue, // TODO
            GameState::Lose(_m) => GameSceneTransition::Continue, // TODO
        }
    }

    fn update_on_each_frame_and_transition_scene(mut self) -> GameState {
        match self.update_on_each_frame() {
            GameSceneTransition::Continue => self,
            GameSceneTransition::ToLoseScene => GameState::Lose(GameOverMenu::new()),
            GameSceneTransition::ToWinScene => GameState::Win(YouWinMenu::new()),
            GameSceneTransition::ToNewGameScene => GameState::start()
        }
    }
}


#[macroquad::main(window_configuration)]
async fn main() {
    set_pc_assets_folder("assets");
    let resources = Resources::load().await;
    let mut game_state = GameState::start();
    loop {
        game_state = game_state.update_on_each_frame_and_transition_scene();
        game_state.render(&resources);
        next_frame().await
    }
}
