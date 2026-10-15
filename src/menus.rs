use macroquad::prelude::*;

pub struct GameOverMenu {

}

impl GameOverMenu {
    pub fn new() -> GameOverMenu {
        GameOverMenu {  }
    }

    pub fn render(&self) {
        clear_background(PURPLE);
        draw_text("you died :(", 20.0, 100.0, 100.0, DARKGRAY);
    }
}