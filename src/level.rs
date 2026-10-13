mod level_tile;

pub use level_tile::LevelTile;

use ndarray::Array2;
use macroquad::prelude::*;
use std::fs;

use crate::constants::NUM_TILES_PER_SCREEN_WIDTH;
use crate::camera::GameCamera;

//const NUM_TILES_PER_SCREEN_WIDTH: u32 = 12;


#[derive(Debug)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub grid: Array2<LevelTile>,
}

struct LevelCreationFileParsingResult {
    level_vec: Vec<LevelTile>,
    width: usize,
    height: usize
}

impl Level {

    pub fn from_level_file(file_path: &str) -> Level {
        let LevelCreationFileParsingResult { level_vec, width, height } = Level::read_level_file_into_vec_of_vecs(file_path);
        let grid = Array2::from_shape_vec((height, width), level_vec)
            .expect("Failed to load level.");
        print!("{:?}", grid);
        Level {
            width,
            height,
            grid
        }
    }

    fn load_row_from_str_into_vec_and_return_num_elems_in_row(level_vec: &mut Vec<LevelTile>, level_str_row: &str) -> usize {
        let level_str_elems = level_str_row.split(" ");
        let mut count = 0;
        for level_str_elem in level_str_elems {
            let level_elem_id = level_str_elem.parse::<u32>().expect("Encountered invalid tile ID in level file that was not unsigned int.");
            level_vec.push(LevelTile::from_numeric_id(level_elem_id));
            count += 1;
        }
        count
    }

    fn read_level_file_into_vec_of_vecs(file_path: &str) -> LevelCreationFileParsingResult {
        let mut level: Vec<LevelTile> = Vec::new();

        let level_str = fs::read_to_string(file_path).expect("Couldn't read level from file path.");
        let level_str_rows = level_str.split("\n");

        let mut level_width = 0;
        let mut level_height = 0;
        let mut first_row = true;
        for level_str_row in level_str_rows {
            let new_level_width = Level::load_row_from_str_into_vec_and_return_num_elems_in_row(&mut level, level_str_row);
            if first_row {
                first_row = false;
                level_width = new_level_width;
            } else {
                assert!(new_level_width == level_width);
            }
            level_height += 1;
        }
        
        LevelCreationFileParsingResult {
            level_vec: level,
            width: level_width,
            height: level_height
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        let start_x = 0.0;
        let start_y = 0.0;
        for x in 0..self.width {
            for y in 0..self.height {
                self.grid[(y, x)].render(camera, x as f32, y as f32);
            }
        }
    }
}