use macroquad::prelude::*;

pub struct GameOverMenu {

}

impl GameOverMenu {
    pub fn new() -> GameOverMenu {
        GameOverMenu {  }
    }

    pub fn render(&self) {
        clear_background(PURPLE);
        draw_text("you died :(", 100.0, 200.0, 100.0, DARKGRAY);
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
    }
}