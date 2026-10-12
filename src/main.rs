use macroquad::prelude::*;
use ndarray::Array2;
use std::fs;

const WINDOW_TITLE: &str = "window title";
//static BACKGROUND_COLOR: Color = Color::new(0.01, 0.01, 0.01, 1.00);
const BACKGROUND_COLOR: Color = Color::from_rgba(200, 240, 255, 255);


const LEVEL_FILE_PATH: &str = "level.txt";

const NUM_TILES_PER_SCREEN_WIDTH: u32 = 12;

fn window_configuration() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: 1200,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}


struct ActiveGameState {
    level: Level
}

impl ActiveGameState {
    fn new() -> ActiveGameState {
        let level = Level::from_level_file(LEVEL_FILE_PATH);
        ActiveGameState {
            level
        }
    }

    fn render(&self) {
        //clear_background(BACKGROUND_COLOR);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_fps();

        self.level.render();
    }

    fn update_on_each_frame(&self) {
        // TODO
    }
}


enum GameState {
    Playing(ActiveGameState),
    Win,
    Lose
}

impl GameState {
    fn start() -> GameState {
        GameState::Playing(ActiveGameState::new())
    }

    fn render(&self) {
        match self {
            GameState::Playing(s) => {
                s.render()
            },
            GameState::Win => (), // TODO
            GameState::Lose => (), // TODO
        }
    }

    fn update_on_each_frame(&self) {
        match self {
            GameState::Playing(s) => {
                s.update_on_each_frame()
            },
            GameState::Win => (), // TODO
            GameState::Lose => (), // TODO
        }
    }
}

#[derive(Debug)]
enum LevelTile {
    Air,
    Ground
}

impl LevelTile {
    fn from_numeric_id(id: u32) -> LevelTile {
        match id {
            0 => LevelTile::Air,
            1 => LevelTile::Ground,
            _ => panic!("Unrecognized tile id.")
        }
    }

    fn render(&self, top_left_x: f32, top_left_y: f32, tile_size: f32) {
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

#[derive(Debug)]
struct Level {
    width: usize,
    height: usize,
    grid: Array2<LevelTile>,
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
        let dim = grid.dim();
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

    fn render(&self) {
        let tile_size = screen_width() / (NUM_TILES_PER_SCREEN_WIDTH as f32);
        let start_x = 0.0;
        let start_y = 0.0;
        for x in 0..self.width {
            for y in 0..self.height {
                self.grid[(y, x)].render(start_x + (x as f32) * tile_size, start_y + (y as f32) * tile_size, tile_size);
            }
        }
    }
}

#[macroquad::main(window_configuration)]
async fn main() {
    let game_state = GameState::start();
    loop {
        game_state.render();
        next_frame().await
    }
}
