use crate::game::Game;
use crate::game::Direction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use std::f64;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use itertools::iproduct;
use crate::strategies::strategy::Strategy;

pub struct ExpectimaxStrategy<const N: usize>{
    pub cache: HashMap<Game<N>, (f64, Option<Direction>)>,
    pub depth: usize,
}
impl<const N: usize> Strategy<N> for ExpectimaxStrategy<N> {
    fn calculate_next_move(&mut self, game: &Game<N>) -> Option<Direction> {
        let (_best_score, best_move) = self.expectimax(game, self.depth);
        // let best_move = self.random_move(game);
        // thread::sleep(Duration::from_millis(500));
        best_move
    }

}
impl<const N: usize> ExpectimaxStrategy<N> {
    pub fn new(depth: usize) -> Self {
        ExpectimaxStrategy {
            cache: HashMap::<Game<N>, (f64, Option<Direction>)>::new(),
            depth: depth,
        }
    }
    fn expectimax(&mut self, state: &Game<N>, depth: usize) -> (f64, Option<Direction>) {
        if depth == 0 || state.check_if_lost() {
            return (self.utility_max_tile_distance(state), None);
        }

        let mut best_score: f64 = 0.0;
        let mut best_move = None;
         
        for step in Direction::iter() {
            // match step{
            //     Direction::Down => {continue;}
            //     _ => {
              
            let mut state_after_my_turn = state.clone();
            if !state_after_my_turn.movement(&step) { // Staying in the same state is not a valid move
                continue;
            }
            let mut expected_value: f64 = 0.0;
            let empty_tiles_list = state_after_my_turn.get_empty_tiles();
            let empty_list_len = empty_tiles_list.len();
            let all_tiles_and_possibilities: Vec<_> = iproduct!(empty_tiles_list.iter(), [2,4].iter()).map(|(a, b)| (*a, *b)).collect();
            for (empty_index, tile_value) in all_tiles_and_possibilities {
                let mut state_after_new_tile = state_after_my_turn.clone();
                state_after_new_tile.new_tile(empty_index as usize, tile_value);
                let mut score = 0.0;
                match self.cache.get(&state_after_new_tile) {
                    Some((cache_score, _)) => score = *cache_score,
                    None => {
                        (score, _) = self.expectimax(&state_after_new_tile, depth - 1);
                        self.cache.insert(state_after_new_tile, (score, None));
                    }
                }
                expected_value += (1.0 / empty_list_len as f64) as f64 * score;
                if expected_value > best_score {
                    best_score = expected_value;
                    best_move = Some(step.clone());
                }
            }
        // }
        // }
        }
        (best_score, best_move)
    

    }


    fn random_move(&mut self, state: &Game<N>) -> Option<Direction> {
        let mut rng = thread_rng(); // Random number generator
        let mut directions: Vec<Direction> = Direction::iter().collect();
        directions.shuffle(&mut rng); // Randomize the order of directions
    
        // thread::sleep(Duration::from_millis(500));
        for direction in directions {
            let mut state_clone = state.clone();
            if state_clone.movement(&direction) {
                return Some(direction); // Return the first valid random move
            }
        }
        None // Return None if no valid move is possible
    }

    pub fn utility_max_tile(&self, state: &Game<N>) -> f64 {
        state.get_max_tile() as f64
    }
    pub fn utility_average(&self, state: &Game<N>) -> f64 {
        let tiles_sum = state.get_tiles_sum() as f64;
        let non_empty_tiles = (N*N - state.get_empty_tiles().len()) as f64;
        tiles_sum / (non_empty_tiles * non_empty_tiles)
    }
    pub fn utility_num_empty_tiles(&self, state: &Game<N>) -> f64 {
        let empty_tiles = state.get_empty_tiles().len() as f64;
        empty_tiles
    }
    pub fn utility_sum_tiles(&self, state: &Game<N>) -> f64 {
        let tiles_sum = state.get_tiles_sum() as f64;
        tiles_sum
    }
    pub fn utility_max_tile_average(&self, state: &Game<N>) -> f64 {
        let max_tile = state.get_max_tile() as f64;
        let non_empty_tiles = (N*N - state.get_empty_tiles().len()) as f64;
        max_tile / (non_empty_tiles * non_empty_tiles)
    }

    // Utility function to calculate the distance of the max tile from the edges
    pub fn utility_max_tile_distance(&self, state: &Game<N>) -> f64 {
        let mut max_value = 0;
        let mut max_position = (0, 0);

        // Find the position of the maximum tile
        for row in 0..N {
            for col in 0..N {
                let tile_value = state.get_tile(row, col) as i32;
                if tile_value > max_value {
                    max_value = tile_value;
                    max_position = (row, col);
                }
            }
        }

        let (row, col) = max_position;

        // Calculate distances to edges
        let top_left_side_distance = row + col;
        let bottom_left_side_distance = (N - 1 - row) + col;
        let top_right_side_distance = row + (N - 1 - col);
        let bottom_right_side_distance = (N - 1 - row) + (N - 1 - col);

        // Minimum distance to any edge
        let min_distance = top_left_side_distance
            .min(bottom_left_side_distance)
            .min(top_right_side_distance)
            .min(bottom_right_side_distance) as f64;

        // let min_distance = top_right_side_distance as f64;

        // Inverse the distance to prioritize tiles closer to edges
        let non_empty_tiles = (N*N - state.get_empty_tiles().len()) as f64;
        state.get_max_tile() as f64 / ((min_distance + 1.0) * (non_empty_tiles * non_empty_tiles))
    }

    
    
}