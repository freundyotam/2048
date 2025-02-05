use crate::game::Game;
use crate::game::Direction;
use strum::IntoEnumIterator;
use std::f64;
use std::collections::HashMap;
use itertools::iproduct;
use crate::strategies::strategy::Strategy;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use rand::Rng; 
use std::cmp;



pub struct ExpectimaxStrategy<const N: usize>{
    pub cache: HashMap<Game<N>, (f64, Option<Direction>)>,
    pub depth: usize,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub delta: f64,
    pub lambda: f64,
}
impl<const N: usize> Strategy<N> for ExpectimaxStrategy<N> {
    fn calculate_next_move(&mut self, game: &Game<N>) -> Option<Direction> {
        let (_best_score, best_move) = self.expectimax(game, self.depth);
        best_move
    }
}
impl<const N: usize> ExpectimaxStrategy<N> {
    pub fn new(depth: usize, alpha: f64, beta: f64, gamma: f64, delta: f64, lambda: f64) -> Self {
        ExpectimaxStrategy {
            cache: HashMap::<Game<N>, (f64, Option<Direction>)>::new(),
            depth: depth,
            alpha: alpha,
            beta: beta,
            gamma: gamma,
            delta: delta,
            lambda: lambda,
        }
    }

    pub fn expectimax(&mut self, state: &Game<N>, depth: usize) -> (f64, Option<Direction>) {
        if state.check_if_lost(){
            return (f64::NEG_INFINITY, None);
        }
        if depth == 0 {
            return (self.utility_snake_shape(state), None);
        }

        let mut best_score: f64 = f64::NEG_INFINITY;
        let mut best_move = None;
         
        for step in Direction::iter() {
            let mut state_after_my_turn = state.clone();
            if !state_after_my_turn.movement(&step){ // Staying in the same state is not a valid move
                continue;
            }
            let mut expected_value: f64 = 0.0;
            let empty_tiles_list = state_after_my_turn.get_empty_tiles();
            let empty_list_len = empty_tiles_list.len();
            let all_tiles_and_possibilities: Vec<_> = iproduct!(empty_tiles_list.iter(), [1,2].iter()).map(|(a, b)| (*a, *b)).collect();
            for (empty_index, tile_value) in all_tiles_and_possibilities {
                let mut state_after_new_tile = state_after_my_turn.clone();
                state_after_new_tile.new_tile(empty_index as usize, tile_value);
                // Self::printBoard(state_after_new_tile.data());
                let mut score = 0.0;
                match self.cache.get(&state_after_new_tile) {
                    Some((cache_score, _)) => score = *cache_score,
                    None => {
                        (score, _) = self.expectimax(&state_after_new_tile, depth - 1);
                        self.cache.insert(state_after_new_tile, (score, None));
                    }
                }
                if tile_value == 1 {
                    score *= 0.9;
                } else {
                    score *= 0.1;
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


    pub fn utility_max_tile(&self, state: &Game<N>) -> f64 {
        let (_, max_tile) = state.get_max_tile();
        max_tile.ilog(2) as f64
    }
    pub fn utility_average(&self, state: &Game<N>) -> f64 {
        // let tiles_sum = state.get_tiles_sum() as f64;
        let non_empty_tiles = (N*N - state.get_empty_tiles().len()) as f64;
        1.0 as f64 / (non_empty_tiles as f64 * non_empty_tiles as f64) as f64
    }
    pub fn utility_empty_tiles(&self, state: &Game<N>) -> f64 {
        let empty_tiles = state.get_empty_tiles().len() as f64;
        return empty_tiles;
    }
    pub fn utility_num_empty_tiles(&self, state: &Game<N>) -> f64 {
        let empty_tiles = state.get_empty_tiles().len() as f64;
        empty_tiles
    }
    pub fn utility_sum_tiles(&self, state: &Game<N>) -> f64 {
        let tiles_sum = state.get_tiles_sum() as f64;
        tiles_sum
    }
    pub fn utility_max_tile_over_empty_tiles_squared(&self, state: &Game<N>) -> f64 {
        let (_, max_tile) = state.get_max_tile();
        let non_empty_tiles = (N*N - state.get_empty_tiles().len()) as f64;
        max_tile as f64 / (non_empty_tiles * non_empty_tiles)
    }
    pub fn utility(&self, state: &Game<N>) -> f64 {
        let mut rng = rand::thread_rng(); // Create a new random number generator
    
        // let monotone = self.monotone_utility(state) / 16.0;
        let corner_utility = self.center_utility(state);
        // if monotone > 3.0 {
        //     return self.corner_utility(state) * 10.0;
        // } else {
        //     self.corner_utility(state) * 10.0  + self.monotone_utility(state) + self.utility_num_empty_tiles(state)
        // }
        // if rng.gen_range(1..=5000) == 10{
        //     self.log_to_file(format!("corner utility: {}, monotone utility: {}, max tile utility: {}, num empty tiles utility: {}, board: {:?}", corner_utility, monotone, self.utility_max_tile(state), self.utility_num_empty_tiles(state), state.data()).as_str());
        // }
        corner_utility
    }

    pub fn gamma_utility(&self, state: &Game<N>) -> f64 {
        return self.alpha * self.utility_max_tile(state) + self.beta * self.utility_empty_tiles(state) + self.gamma * self.center_utility(state) + self.delta * self.snake_utility(state) + self.lambda * self.corner_utility(state); // TODO this should be - 
    }

    pub fn corner_utility(&self, state: &Game<N>) -> f64 {
        // sleep a second
        // std::thread::sleep(std::time::Duration::from_secs(1));
        // println!("Snake utility got this board {:?}", state.data());
        // let stdout_raw = stdout();
        // let mut stdout = BufWriter::new(stdout_raw.lock());
        // let board = board::Board::new();
        // display::display_game(&mut stdout, &board, &state).unwrap();
        let mut corner_score = 0.0f64;
        for (index, &value) in state.data().iter().enumerate() {
            corner_score += (((index / N) as usize + index % N as usize) * value as usize) as f64;
        }
        corner_score / 10.0
    }

    pub fn center_utility(&self, state: &Game<N>) -> f64 {
        // sleep a second
        // std::thread::sleep(std::time::Duration::from_secs(1));
        // println!("Snake utility got this board {:?}", state.data());
        // let stdout_raw = stdout();
        // let mut stdout = BufWriter::new(stdout_raw.lock());
        // let board = board::Board::new();
        // display::display_game(&mut stdout, &board, &state).unwrap();

        let mut center_score = 0.0f64;

        for (index, &value) in state.data().iter().enumerate() {
            if index == 5 || index == 6 || index == 9 || index == 10 {
                center_score += 0.0 * value as f64;
            } else {
                center_score += 1.0* value as f64;
            } 
        }
        center_score / 10.0
    }

    pub fn monotone_utility(&self, state: &Game<N>) -> f64 {
        let mut score = 0.0;
        for i in 0..N {
            if(state.data()[i * N] > state.data()[i * N + 1] && state.data()[i * N + 1] > state.data()[i * N + 2] && state.data()[i * N + 2] > state.data()[i * N + 3]) {
                score += 1.0;
            }
        }
        score
    }

    pub fn snake_utility(&self, state: &Game<N>) -> f64 {
        let mut score = 0.0;
        let mut snake_order = Vec::new();
    
        // Generate snake-like order
        for row in 0..N {
            if row % 2 == 0 {
                // Left to right
                for col in 0..N {
                    snake_order.push((row, col));
                }
            } else {
                // Right to left
                for col in (0..N).rev() {
                    snake_order.push((row, col));
                }
            }
        }
    
        // Iterate through snake order and compute score
        for i in 0..(snake_order.len() - 1) {
            let (row1, col1) = snake_order[i];
            let (row2, col2) = snake_order[i + 1];
            let index1 = row1 * N + col1;
            let index2 = row2 * N + col2;
    
            if state.data()[index1] + 1 == state.data()[index2] {
                score += state.data()[index1] as f64;
            }
        }
    
        score
    }


    pub fn snake_utility2(&self, state: &Game<N>) -> f64 {
        let mut score = 0.0;
        for i in 0..N*N{
            score += state.data()[i] as f64 * (i + 1) as f64;
        }
        score
    }

    pub fn utility_snake_shape(&self, state: &Game<N>) -> f64 {
        let sum = state.get_tiles_snake_sum();
        sum
    }

    pub fn is_next_to_each_other((x1, y1): (i32, i32), (x2, y2): (i32,i32), state: &Game<N>) -> i32 {
        if x1 < 0 || x1 >= N as i32 || y1 < 0 || y1 >= N as i32 || x2 < 0 || x2 >= N as i32 || y2 < 0 || y2 >= N as i32 {
            return 0;
        } else {
            if state.data()[(x1 * N as i32 + y1) as usize] ==  state.data()[(x2 * N as i32 + y2) as usize]{
                return 1;
            } else {
                return 0;
            }
        }
    }
    pub fn log_to_file(&self, line: &str) {
        let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("snake_utility.txt")
        .unwrap();

         if let Err(e) = writeln!(file, "{}", line) {
             eprintln!("Couldn't write to file: {}", e);
        }

    }

}

