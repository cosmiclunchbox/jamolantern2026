mod level_tile;

pub use level_tile::LevelTile;

use ndarray::Array2;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::fs;

use crate::constants::NUM_TILES_PER_SCREEN_WIDTH;
use crate::camera::GameCamera;

//const NUM_TILES_PER_SCREEN_WIDTH: u32 = 12;


#[derive(Debug)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    grid: HashMap<(i32, i32), LevelTile>,
}

struct LevelCreationFileParsingResult {
    grid: HashMap<(i32, i32), LevelTile>,
    width: usize,
    height: usize
}

impl Level {

    pub fn from_level_file(file_path: &str) -> Level {
        let LevelCreationFileParsingResult { grid, width, height } = Level::read_level_file_into_vec_of_vecs(file_path);
        println!("{:?}", grid);
        Level {
            width,
            height,
            grid
        }
    }

    fn load_row_from_str_into_vec_and_return_num_elems_in_row(
        level_vec: &mut HashMap<(i32, i32), LevelTile>, 
        level_str_row: &str, 
        level_col_number: usize
    ) -> usize {
        let level_str_elems = level_str_row.split(" ");
        let mut count = 0;
        for (x, level_str_elem) in level_str_elems.enumerate() {
            let level_elem_id = level_str_elem.parse::<u32>().expect("Encountered invalid tile ID in level file that was not unsigned int.");
            level_vec.insert(
                (x as i32, level_col_number as i32), 
                LevelTile::from_x_y_and_tile_id(
                    x as i32, 
                    level_col_number as i32, 
                    level_elem_id
                )
            );
            count += 1;
        }
        count
    }

    fn read_level_file_into_vec_of_vecs(file_path: &str) -> LevelCreationFileParsingResult {
        let mut level = HashMap::new();

        let level_str = fs::read_to_string(file_path).expect("Couldn't read level from file path.");
        let level_str_rows = level_str.split("\n");

        let mut level_width = 0;
        let mut level_height = 0;
        let mut first_row = true;
        for (y, level_str_row) in level_str_rows.enumerate() {
            let new_level_width = Level::load_row_from_str_into_vec_and_return_num_elems_in_row(&mut level, level_str_row, y);
            if first_row {
                first_row = false;
                level_width = new_level_width;
            } else {
                assert!(new_level_width == level_width);
            }
            level_height += 1;
        }
        
        LevelCreationFileParsingResult {
            grid: level,
            width: level_width,
            height: level_height
        }
    }

    pub fn render(&self, camera: &GameCamera) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.grid.get(&(x as i32, y as i32)).expect("Entire rectangular grid should be tiles.").render(camera);
            }
        }
    }
}