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


// Function to calculate the Manhattan distance between two points
pub fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

pub fn distance_to_corner(row: usize, col: usize, n: usize) -> f64 {

    // let top_left_distance = manhattan_distance(row, col, 0, 0);
    let top_right_distance = manhattan_distance(row, col, 0, n - 1);
    // let bottom_left_distance = manhattan_distance(row, col, n - 1, 0);
    // let bottom_right_distance = manhattan_distance(row, col, 0, 0);

    // let distance = top_left_distance
    // .min(top_right_distance)
    // .min(bottom_left_distance)
    // .min(bottom_right_distance);

    let distance = top_right_distance;
    // Return inverse of the distance to prioritize tiles closer to the corner
    distance as f64
}


pub struct ExpectimaxStrategy<const N: usize>{
    pub cache: HashMap<Game<N>, (f64, Option<Direction>)>,
    pub depth: usize,
}
impl<const N: usize> Strategy<N> for ExpectimaxStrategy<N> {
    fn calculate_next_move(&mut self, game: &Game<N>) -> Option<Direction> {
        let (_best_score, best_move) = self.expectimax(game, self.depth);
        let best_move = self.random_move(game);
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
            return (self.utility_max_tile_average(state), None);
        }

        let mut best_score: f64 = 0.0;
        let mut best_move = None;
         
        for step in Direction::iter() {
            match step{
                Direction::Down => {
                    if !self.can_move_only_down(state) {
                        continue;
                    }
                }
                _ => {}
            }
              
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
        } 
        (best_score, best_move)
    

    }

    pub fn can_move_only_down(&self, state: &Game<N>) -> bool {
        let directions: Vec<Direction> = Direction::iter().collect();
        for direction in directions {
            match direction {
                Direction::Down => {continue;}
                _ => {
                    let mut state_clone = state.clone();
                    if state_clone.movement(&direction) {
                        return false;
                    }
                }
            }
        }
        true
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
    pub fn utility_sum_tiles_average(&self, state: &Game<N>) -> f64 {
        let sum_tile = self.utility_sum_tiles(state) as f64;
        let non_empty_tiles = (N*N - state.get_empty_tiles().len()) as f64;
        sum_tile / (non_empty_tiles * non_empty_tiles)
    }


    // Utility function to calculate the distance of the max tile from the edges
    pub fn utility_manhattan_distance(&self, state: &Game<N>) -> f64 {
        let mut top_tiles = Vec::new();

        // Gather all tiles with their positions
        for row in 0..N {
            for col in 0..N {
                let tile_value = state.get_tile(row, col);
                if tile_value > 0 {
                    top_tiles.push((tile_value, (row, col)));
                }
            }
        }

        // Sort tiles by value in descending order
        top_tiles.sort_by(|a, b| b.0.cmp(&a.0));

        // Take the top 4 tiles (or fewer if not enough tiles exist)
        let top_positions: Vec<(usize, usize)> = top_tiles.iter().take(4).map(|&(_, pos)| pos).collect();

        // Initialize utility
        let mut utility = 0.0;

        // Distance of the first max to the corner (e.g., top-left corner (0, 0))
        if let Some(&(max_row, max_col)) = top_positions.get(0) {
            let distance_to_corner = distance_to_corner(max_row, max_col, N);
            utility += 100000.0 / (distance_to_corner as f64 + 0.1);
        }

        // Add distances between consecutive max tiles
        let max_pairs = 3;
        let mut count = 0;
        for pair in top_positions.windows(2) {
            if let [(row1, col1), (row2, col2)] = pair {
                let distance = manhattan_distance(*row1, *col1, *row2, *col2);
                utility += 1.0 / (distance * ((count + 1) * 100)) as f64;
            }
            count += 1;
            if count == max_pairs {
                break;
            }
        }

        utility
    }

    pub fn utility_manhattan_distance_to_corner_maxtiles_average(&self, state: &Game<N>) -> f64 {
        let utility1 = self.utility_manhattan_distance(state);
        let utility2 = self.utility_max_tile_average(state);
        utility1 * utility2
    }


    pub fn utility_tile_uniqueness(&self, state: &Game<N>) -> f64 {
        let mut tile_count: HashMap<i32, usize> = HashMap::new();

        // Count occurrences of each tile value
        for row in 0..N {
            for col in 0..N {
                let value = state.get_tile(row, col);
                if value > 0 {
                    *tile_count.entry(value).or_insert(0) += 1;
                }
            }
        }

        // Compute "uniqueness score" (fewer unique tiles -> higher score)
        let unique_count = tile_count.len() as f64;

        // Normalize the score to prefer fewer unique values
        let max_possible_unique = (N * N) as f64; // Worst case: all tiles unique
        let score = (max_possible_unique - unique_count) / max_possible_unique;

        score // Higher score indicates less uniqueness (better for merges)
    }

    pub fn utility_tile_uniqeness_tiles_maxTile_average(&self, state: &Game<N>) -> f64 {
        let max_tile_average = self.utility_max_tile_average(state) as f64;
        let uniqueness_bonus = self.utility_tile_uniqueness(state) * 10.0; // Weight uniqueness more
        max_tile_average * uniqueness_bonus
    }
    
    
}