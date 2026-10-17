use std::collections::{HashMap, VecDeque};

use crate::constants::{ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE, ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE};
use crate::enemy::Enemy;
use crate::level::{Level, LevelTileType};
use crate::util::MoveDirection;

#[derive(Debug)]
pub struct EnemyAngryAI {
    enemy_center_logical_x: f32,
    enemy_center_logical_y: f32,
    destination_x: f32,
    destination_y: f32,
    pub next_tile_index_x: i32, // adjacent to current tile
    pub next_tile_index_y: i32, // adjacent to current tile
}


#[derive(Debug)]
pub enum EnemyDecision {
    AttackPlayer,
    StayStill,
    Move(MoveDirection)
}

enum AlignmentRelativeToTile {
    Aligned,
    TooFarUp,
    TooFarLeft,
    TooFarRight,
    TooFarDown
}

impl EnemyAngryAI {
    pub fn at_position(spawn_x: f32, spawn_y: f32) -> EnemyAngryAI {
        let (next_tile_index_x, next_tile_index_y) = EnemyAngryAI::coordinates_to_tile_index(spawn_x, spawn_y);
        EnemyAngryAI {
            enemy_center_logical_x: spawn_x,
            enemy_center_logical_y: spawn_y,
            destination_x: spawn_x,
            destination_y: spawn_y,
            next_tile_index_x,
            next_tile_index_y
        }
    }

    fn coordinates_to_tile_index(x: f32, y: f32) -> (i32, i32) {
        (x.trunc() as i32, y.trunc() as i32)
    }

    fn current_enemy_coordinates_to_tile_index(&self) -> (i32, i32) {
        EnemyAngryAI::coordinates_to_tile_index(self.enemy_center_logical_x, self.enemy_center_logical_y)
    }

    fn current_alignment_relative_to_next_tile_position(&self) -> AlignmentRelativeToTile {
        return if self.enemy_center_logical_x > self.next_tile_index_x as f32 + ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * 0.5 &&
            self.enemy_center_logical_x < self.next_tile_index_x as f32 + 1.0 - ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * 0.5 &&
            self.enemy_center_logical_y > self.next_tile_index_y as f32 + ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * 0.5 && 
            self.enemy_center_logical_y < self.next_tile_index_y as f32 + 1.0 - ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * 0.5 {
            AlignmentRelativeToTile::Aligned
        } else if self.enemy_center_logical_x <= self.next_tile_index_x as f32 + ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * 0.5 {
            AlignmentRelativeToTile::TooFarLeft
        } else if self.enemy_center_logical_x >= self.next_tile_index_x as f32 + 1.0 - ENEMY_WIDTH_AS_MULTIPLE_OF_TILE_SIZE * 0.5 {
            AlignmentRelativeToTile::TooFarRight
        } else if self.enemy_center_logical_y <= self.next_tile_index_y as f32 + ENEMY_HEIGHT_AS_MULTIPLE_OF_TILE_SIZE * 0.5 {
            AlignmentRelativeToTile::TooFarUp
        } else {
            AlignmentRelativeToTile::TooFarDown
        }
    }

    pub fn update_current_enemy_center_position(&mut self, x: f32, y: f32) {
        self.enemy_center_logical_x = x;
        self.enemy_center_logical_y = y;
    }

    pub fn set_target_location(&mut self, x: f32, y: f32) {
        self.destination_x = x;
        self.destination_y = y;
    }

    pub fn get_next_enemy_decision_in_level(&mut self, level: &Level) -> EnemyDecision {
        let current_pos = self.current_enemy_coordinates_to_tile_index();
        let target_pos = EnemyAngryAI::coordinates_to_tile_index(self.destination_x, self.destination_y);
        if current_pos == target_pos {
            println!("YESSIR GOTEM");
            return EnemyDecision::AttackPlayer;
        }

        let alignment_relative_to_next_tile = self.current_alignment_relative_to_next_tile_position();
        match alignment_relative_to_next_tile {
            AlignmentRelativeToTile::TooFarUp => EnemyDecision::Move(MoveDirection::Down),
            AlignmentRelativeToTile::TooFarLeft => EnemyDecision::Move(MoveDirection::Right),
            AlignmentRelativeToTile::TooFarRight => EnemyDecision::Move(MoveDirection::Left),
            AlignmentRelativeToTile::TooFarDown => EnemyDecision::Move(MoveDirection::Up),
            AlignmentRelativeToTile::Aligned => self.set_next_tile_and_determine_next_move_direction_based_on_target_location(level)
        }
    }

    fn get_empty_neighboring_spaces(&self, x: i32, y: i32, level: &Level) -> Vec<(i32, i32)> {
        let mut empty_neighbors = Vec::new();
        for (n_x, n_y) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y),] {
            if let Some(LevelTileType::Ground) = level.get_tile_type_at_coordinates(n_x, n_y) {
                empty_neighbors.push((n_x, n_y));
            }
        }
        empty_neighbors
    }

    fn run_bfs(&self, level: &Level, start_pos: (i32, i32), target_pos: (i32, i32)) -> Option<HashMap<(i32, i32), (i32, i32)>> {
        // returns map of tiles to their parents in the path
        // chunky function incoming!!!
        let (current_tile_index_x, current_tile_index_y) = start_pos;
        let (target_tile_index_x, target_tile_index_y) = target_pos;

        // run a bfs to find the shortest path
        // yes I know this is horribly written code that should be abstracted into multiple functions
        let mut node_parents = HashMap::new();
        let mut pending = VecDeque::new();
        let neighbors = self.get_empty_neighboring_spaces(current_tile_index_x, current_tile_index_y, level);
        for neighbor in neighbors {
            node_parents.insert(neighbor, (current_tile_index_x, current_tile_index_y));
            pending.push_back(neighbor);
        }

        loop {
            let Some(next_tile_to_check) = pending.pop_front() else {
                // no path found to target
                return None;
            };
            if next_tile_to_check == (target_tile_index_x, target_tile_index_y) {
                //println!("OH BABY");
                break Some(node_parents);
            }
            let neighbors = self.get_empty_neighboring_spaces(next_tile_to_check.0, next_tile_to_check.1, level);
            for neighbor in neighbors {
                if !node_parents.contains_key(&neighbor) {
                    node_parents.insert(neighbor, next_tile_to_check);
                    pending.push_back(neighbor);
                }
            }
        }
    }

    fn set_next_tile_and_determine_next_move_direction_based_on_target_location(&mut self, level: &Level) -> EnemyDecision {
        let current_pos = self.current_enemy_coordinates_to_tile_index();
        let target_pos = EnemyAngryAI::coordinates_to_tile_index(self.destination_x, self.destination_y);

        let Some(mut node_parents) = self.run_bfs(
            level, 
            current_pos,
            target_pos
        ) else {
            return EnemyDecision::StayStill;
        };

        // backtracking through bfs pointers to get the next tile to move to from the current position
        let mut next_tile_to_check = target_pos;
        loop {
            let Some(&parent) = node_parents.get(&next_tile_to_check) else {
                panic!("this shouldn't happen, should get to starting node without exhausting all parents 2394");
                break;
            };

            //println!("CURRENT: {:?}", (current_tile_index_x, current_tile_index_y));
            //println!("{:?} --> {:?}", next_tile_to_check, parent);

            if parent == current_pos {
                break;
            } else {
                node_parents.remove(&next_tile_to_check);
                next_tile_to_check = parent;
            }
        };
        let (next_tile_x, next_tile_y) = next_tile_to_check;

        // set next tile to move to
        self.next_tile_index_x = next_tile_x;
        self.next_tile_index_y = next_tile_y;

        // yes yes I know this is copypasted from the other function
        let alignment_relative_to_next_tile = self.current_alignment_relative_to_next_tile_position();
        match alignment_relative_to_next_tile {
            AlignmentRelativeToTile::TooFarUp => EnemyDecision::Move(MoveDirection::Down),
            AlignmentRelativeToTile::TooFarLeft => EnemyDecision::Move(MoveDirection::Right),
            AlignmentRelativeToTile::TooFarRight => EnemyDecision::Move(MoveDirection::Left),
            AlignmentRelativeToTile::TooFarDown => EnemyDecision::Move(MoveDirection::Up),
            AlignmentRelativeToTile::Aligned => panic!("wtf shouldn't be at next position already 094328")
        }
    }

}