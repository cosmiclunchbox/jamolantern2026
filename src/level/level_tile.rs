use macroquad::prelude::*;
use crate::camera::GameCamera;
use crate::util::get_tile_size;

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

    pub fn render(&self, camera: &GameCamera, tl_logical_x: f32, tl_logical_y: f32) {
        let tile_size = get_tile_size();
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(tl_logical_x, tl_logical_y);
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