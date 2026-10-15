use macroquad::prelude::*;
use crate::camera::GameCamera;
use crate::util::get_tile_size;
use crate::hitbox::MayHaveHitbox;

const BACKGROUND_COLOR: Color = Color::from_rgba(200, 240, 255, 255);

#[derive(Debug)]
pub enum LevelTileType {
    Ground,
    Wall,
    Sinkhole,
}

impl LevelTileType {
    pub fn from_numeric_id(id: u32) -> LevelTileType {
        match id {
            0 => LevelTileType::Ground,
            1 => LevelTileType::Wall,
            2 => LevelTileType::Sinkhole,
            _ => panic!("Unrecognized tile id.")
        }
    }

    pub fn draw_tile_at_x_y_screen_position(&self, tl_screen_x: f32, tl_screen_y: f32) {
        let tile_size = get_tile_size();
        match self {
            LevelTileType::Ground => {
                draw_rectangle(tl_screen_x, tl_screen_y, tile_size, tile_size, GREEN);
            },
            LevelTileType::Wall => {
                draw_rectangle(tl_screen_x, tl_screen_y, tile_size, tile_size, GRAY);
            },
            LevelTileType::Sinkhole => {
                draw_rectangle(tl_screen_x, tl_screen_y, tile_size, tile_size, RED);
            },
        }
    }
}


#[derive(Debug)]
pub struct LevelTile {
    tl_logical_x: i32,
    tl_logical_y: i32,
    pub tile_type: LevelTileType
}

impl LevelTile {
    pub fn from_x_y_and_tile_id(top_left_x: i32, top_left_y: i32, tile_type_id: u32) -> LevelTile {
        LevelTile {
            tl_logical_x: top_left_x,
            tl_logical_y: top_left_y,
            tile_type: LevelTileType::from_numeric_id(tile_type_id)
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(
            self.tl_logical_x as f32, 
            self.tl_logical_y as f32
        );
        self.tile_type.draw_tile_at_x_y_screen_position(top_left_x, top_left_y);
    }
}


impl MayHaveHitbox for LevelTile {
    fn get_hitbox(&self) -> Option<Rect> {
        match self.tile_type {
            LevelTileType::Ground => None,
            LevelTileType::Wall | LevelTileType::Sinkhole => Some(
                Rect {
                    x: self.tl_logical_x as f32,
                    y: self.tl_logical_y as f32,
                    w: 1.0,
                    h: 1.0
                }
            )
        }
    }
}