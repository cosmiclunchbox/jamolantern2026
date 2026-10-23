use macroquad::prelude::*;

use crate::GameSceneTransition;

pub struct MainMenu {

}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {  }
    }

    pub fn render(&self) {
        clear_background(LIGHTGRAY);
        draw_text("package of no return", 100.0, 200.0, 100.0, DARKGRAY);
        draw_text("wasd to move", 100.0, 400.0, 50.0, DARKGRAY);
        draw_text("space to use your package as a weapon", 100.0, 500.0, 50.0, DARKGRAY);
        draw_text("press space to begin", 100.0, 600.0, 50.0, DARKGRAY);
    }

    pub fn update_on_each_frame(&self) -> GameSceneTransition {
        if is_key_released(KeyCode::Space) {
            GameSceneTransition::ToNewGameScene
        } else {
            GameSceneTransition::Continue
        }
    }
}

pub struct GameOverMenu {

}

impl GameOverMenu {
    pub fn new() -> GameOverMenu {
        GameOverMenu {  }
    }

    pub fn render(&self) {
        clear_background(PURPLE);
        draw_text("you died :(", 100.0, 200.0, 100.0, DARKGRAY);
        draw_text("your package will never be returned now", 100.0, 400.0, 50.0, DARKGRAY);
        draw_text("press space to try again", 100.0, 500.0, 50.0, DARKGRAY);
    }

    pub fn update_on_each_frame(&self) -> GameSceneTransition {
        if is_key_released(KeyCode::Space) {
            GameSceneTransition::ToNewGameScene
        } else {
            GameSceneTransition::Continue
        }
    }
}

pub struct YouWinMenu {

}

impl YouWinMenu {
    pub fn new() -> YouWinMenu {
        YouWinMenu {  }
    }

    pub fn render(&self) {
        clear_background(PINK);
        draw_text("you win :D", 100.0, 200.0, 100.0, DARKGRAY);
        draw_text("you manage to return your package in time", 100.0, 400.0, 50.0, DARKGRAY);
        draw_text("press space to play again", 100.0, 500.0, 50.0, DARKGRAY);
    }

    pub fn update_on_each_frame(&self) -> GameSceneTransition {
        if is_key_released(KeyCode::Space) {
            GameSceneTransition::ToNewGameScene
        } else {
            GameSceneTransition::Continue
        }
    }
}