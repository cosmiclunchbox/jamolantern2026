use macroquad::input::KeyCode::D;
use macroquad::prelude::*;
use crate::camera::GameCamera;
use crate::resources::Resources;
use crate::util::get_tile_size;
use crate::hitbox::MayHaveHitbox;

//const BACKGROUND_COLOR: Color = Color::from_rgba(200, 240, 255, 255);

#[derive(Clone, Debug)]
pub enum LevelTileType {
    Grass1,
    Road1,
    Road2,
    Road3,
    Road4,
    Road5,
    Road6,
    Road7,
    Road8,
    Road9,
    Road10,
    Wall,
    Sinkhole,
    UspsEntrance,
}

impl LevelTileType {
    pub fn from_numeric_id(id: u32) -> LevelTileType {
        match id {
            0 => LevelTileType::Grass1,
            1 => LevelTileType::Wall,
            2 => LevelTileType::Sinkhole,
            3 => LevelTileType::UspsEntrance,
            4 => LevelTileType::Road1,
            5 => LevelTileType::Road2,
            6 => LevelTileType::Road3,
            7 => LevelTileType::Road4,
            8 => LevelTileType::Road5,
            9 => LevelTileType::Road6,
            10 => LevelTileType::Road7,
            11 => LevelTileType::Road8,
            12 => LevelTileType::Road9,
            13 => LevelTileType::Road10,
            _ => panic!("Unrecognized tile id.")
        }
    }

    pub fn draw_tile_at_x_y_screen_position(&self, tl_screen_x: f32, tl_screen_y: f32, resources: &Resources) {
        let tile_size = get_tile_size();
        let tile_draw_params = DrawTextureParams {
            dest_size: Some(Vec2::new(tile_size, tile_size)),
            ..Default::default()
        };
        match self {
            LevelTileType::Grass1 => {
                draw_texture_ex(&resources.level_grass1, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road1 => {
                draw_texture_ex(&resources.level_road1, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road2 => {
                draw_texture_ex(&resources.level_road2, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road3 => {
                draw_texture_ex(&resources.level_road3, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road4 => {
                draw_texture_ex(&resources.level_road4, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road5 => {
                draw_texture_ex(&resources.level_road5, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road6 => {
                draw_texture_ex(&resources.level_road6, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road7 => {
                draw_texture_ex(&resources.level_road7, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road8 => {
                draw_texture_ex(&resources.level_road8, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road9 => {
                draw_texture_ex(&resources.level_road9, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Road10 => {
                draw_texture_ex(&resources.level_road10, tl_screen_x, tl_screen_y, WHITE, tile_draw_params);
            },
            LevelTileType::Wall => {
                draw_rectangle(tl_screen_x, tl_screen_y, tile_size, tile_size, GRAY);
            },
            LevelTileType::Sinkhole => {
                draw_rectangle(tl_screen_x, tl_screen_y, tile_size, tile_size, RED);
            },
            LevelTileType::UspsEntrance => {
                draw_rectangle(tl_screen_x, tl_screen_y, tile_size, tile_size, YELLOW);
            },
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            LevelTileType::Wall | LevelTileType::Sinkhole | LevelTileType::UspsEntrance => true,
            _ => false
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

    pub fn render(&self, camera: &GameCamera, resources: &Resources) {
        let (top_left_x, top_left_y) = camera.get_screen_position_for_logical_coordinates(
            self.tl_logical_x as f32, 
            self.tl_logical_y as f32
        );
        self.tile_type.draw_tile_at_x_y_screen_position(top_left_x, top_left_y, resources);
    }
}


impl MayHaveHitbox for LevelTile {
    fn get_hitbox(&self) -> Option<Rect> {
        match self.tile_type {
            LevelTileType::Grass1 | LevelTileType:: Road1 | LevelTileType::Road2 | LevelTileType::Road3 | LevelTileType:: Road4 | 
            LevelTileType::Road5 | LevelTileType::Road6 | LevelTileType::Road7 | LevelTileType::Road8 | LevelTileType::Road9 | LevelTileType::Road10 => None,
            LevelTileType::Wall | LevelTileType::Sinkhole | LevelTileType::UspsEntrance => Some(
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