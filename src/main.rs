use macroquad::prelude::*;
use ndarray::Array2;
use std::fs;

const WINDOW_TITLE: &str = "window title";
//static BACKGROUND_COLOR: Color = Color::new(0.01, 0.01, 0.01, 1.00);
const BACKGROUND_COLOR: Color = Color::from_rgba(200, 240, 255, 255);


const LEVEL_FILE_PATH: &str = "level.txt";

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
        let level = Level::from_level_file(LEVEL_FILE_PATH, 8, 6);
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

impl Level {

    pub fn from_level_file(file_path: &str, expected_width: usize, expected_height: usize) -> Level {
        let level_vecs = Level::read_level_file_into_vec_of_vecs(file_path);
        let grid = Array2::from_shape_vec((expected_height, expected_width), level_vecs)
            .expect("Failed to load level.");
        let dim = grid.dim();
        assert!(dim.0 == expected_height && dim.1 == expected_width);
        print!("{:?}", grid);
        Level {
            width: expected_width,
            height: expected_height,
            grid
        }
    }

    fn read_level_file_into_vec_of_vecs(file_path: &str) -> Vec<LevelTile> {
        let mut level: Vec<LevelTile> = Vec::new();

        let level_str = fs::read_to_string(file_path).expect("Couldn't read level from file path.");
        let level_str_rows = level_str.split("\n");
        /*let expected_width = level_str_rows.next()
            .expect("Couldn't read level width in file.")
            .parse::<usize>()
            .expect("Invalid level width provided.");
        let expected_height = level_str_rows.next()
            .expect("Couldn't read level height in file.")
            .parse::<usize>()
            .expect("Invalid level height provided.");*/
        for level_str_row in level_str_rows {
            let level_str_elems = level_str_row.split(" ");
            for level_str_elem in level_str_elems {
                let level_elem_id = level_str_elem.parse::<u32>().expect("Encountered invalid tile ID in level file that was not unsigned int.");
                level.push(LevelTile::from_numeric_id(level_elem_id));
            }
            //assert!(level_row.len() == expected_width);
            //level.push(level_row);
        }
        
        //assert!(level.len() == expected_height);
        level
    }

    fn render(&self) {
        let tile_size = screen_width() / 12.0;
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
