use crate::constants::{ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE, ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE};
use crate::enemy::Enemy;
use crate::level::Level;

pub struct EnemyAI<'a> {
    enemy_tl_logical_x: f32,
    enemy_tl_logical_y: f32,
    destination_x: f32,
    destination_y: f32,
    next_tile_index_x: usize, // adjacent to current tile
    next_tile_index_y: usize, // adjacent to current tile
    level: &'a Level
}

struct InWhichTile {
    tile_index_x: usize,
    tile_index_y: usize,
    fully_on_tile: AlignmentRelativeToTile,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right
}

pub enum EnemyDecision {
    Move(MoveDirection)
}

enum AlignmentRelativeToTile {
    Aligned,
    TooFarUp,
    TooFarLeft,
    TooFarRight,
    TooFarDown
}

impl<'a> EnemyAI<'a> {
    pub fn at_position(spawn_x: f32, spawn_y: f32, level: &'a Level) -> EnemyAI<'a> {
        let (next_tile_index_x, next_tile_index_y) = EnemyAI::coordinates_to_tile_index(spawn_x, spawn_y);
        EnemyAI {
            enemy_tl_logical_x: spawn_x,
            enemy_tl_logical_y: spawn_y,
            destination_x: spawn_x,
            destination_y: spawn_y,
            next_tile_index_x,
            next_tile_index_y,
            level
        }
    }

    fn coordinates_to_tile_index(x: f32, y: f32) -> (usize, usize) {
        (x.trunc() as usize, y.trunc() as usize)
    }

    fn current_enemy_coordinates_to_tile_index(&self) -> (usize, usize) {
        EnemyAI::coordinates_to_tile_index(self.enemy_tl_logical_x, self.enemy_tl_logical_y)
    }

    fn current_alignment_relative_to_next_tile_position(&self) -> AlignmentRelativeToTile {
        return if self.enemy_tl_logical_x > self.next_tile_index_x as f32 &&
            self.enemy_tl_logical_x < self.next_tile_index_x as f32 + 1.0 - ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE &&
            self.enemy_tl_logical_y > self.next_tile_index_y as f32 && 
            self.enemy_tl_logical_y < self.next_tile_index_y as f32 + 1.0 - ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE {
            AlignmentRelativeToTile::Aligned
        } else if self.enemy_tl_logical_x <= self.next_tile_index_x as f32 {
            AlignmentRelativeToTile::TooFarLeft
        } else if self.enemy_tl_logical_x >= self.next_tile_index_x as f32 + 1.0 - ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE {
            AlignmentRelativeToTile::TooFarRight
        } else if self.enemy_tl_logical_y <= self.next_tile_index_y as f32 {
            AlignmentRelativeToTile::TooFarUp
        } else {
            AlignmentRelativeToTile::TooFarDown
        }
    }

    fn set_target_location(&mut self, x: f32, y: f32) {
        self.destination_x = x;
        self.destination_y = y;
    }

    fn get_next_enemy_direction(&mut self) -> EnemyDecision {
        let (current_tile_index_x, current_tile_index_y) = self.current_enemy_coordinates_to_tile_index();
        let alignment_relative_to_next_tile = self.current_alignment_relative_to_next_tile_position();
        
        match alignment_relative_to_next_tile {
            AlignmentRelativeToTile::TooFarUp => EnemyDecision::Move(MoveDirection::Down),
            AlignmentRelativeToTile::TooFarLeft => EnemyDecision::Move(MoveDirection::Right),
            AlignmentRelativeToTile::TooFarRight => EnemyDecision::Move(MoveDirection::Left),
            AlignmentRelativeToTile::TooFarDown => EnemyDecision::Move(MoveDirection::Up),
            AlignmentRelativeToTile::Aligned => {

            }
        }
    }

}