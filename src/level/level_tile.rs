use macroquad::prelude::*;

const BACKGROUND_COLOR: Color = Color::from_rgba(200, 240, 255, 255);

#[derive(Debug)]
pub enum LevelTile {
    Air,
    Ground
}

impl LevelTile {
    pub fn from_numeric_id(id: u32) -> LevelTile {
        match id {
            0 => LevelTile::Air,
            1 => LevelTile::Ground,
            _ => panic!("Unrecognized tile id.")
        }
    }

    pub fn render(&self, top_left_x: f32, top_left_y: f32, tile_size: f32) {
        match self {
            LevelTile::Air => {
                draw_rectangle(top_left_x, top_left_y, tile_size, tile_size, BACKGROUND_COLOR);
            },
            LevelTile::Ground => {
                draw_rectangle(top_left_x, top_left_y, tile_size, tile_size, GREEN);
            }
        }
    }
}