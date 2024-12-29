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


pub struct ExpectimaxStrategy<const N: usize>{
    pub cache: HashMap<Game<N>, (f64, Option<Direction>)>,
    pub depth: usize,
}
impl<const N: usize> Strategy<N> for ExpectimaxStrategy<N> {
    fn calculate_next_move(&mut self, game: &Game<N>) -> Option<Direction> {
        let (_best_score, best_move) = self.expectimax(game, self.depth);
        println!("Best move: {:?}", best_move);
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
        if state.check_if_lost(){
            return (f64::NEG_INFINITY, None);
        }
        if depth == 0 {
            return (self.utility(state), None);
        }

        let mut best_score: f64 = f64::NEG_INFINITY;
        let mut best_move = Some(Direction::Down);
         
        for step in Direction::iter() {
            let (_, max_tile_value) = state.get_max_tile();
            if step == Direction::Down && max_tile_value < 2048 {
                continue;
            }

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


    pub fn utility_max_tile(&self, state: &Game<N>) -> f64 {
        let (_, max_tile) = state.get_max_tile();
        max_tile as f64
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
        let (_, max_tile) = state.get_max_tile();
        let non_empty_tiles = (N*N - state.get_empty_tiles().len()) as f64;
        max_tile as f64 / (non_empty_tiles * non_empty_tiles)
    }
    pub fn utility(&self, state: &Game<N>) -> f64 {
        let mut rng = rand::thread_rng(); // Create a new random number generator
        if rng.gen_range(1..=5000) == 10{
            self.log_to_file(format!("corner utility: {}, monotone utility: {}, max tile utility: {}, num empty tiles utility: {}, board: {:?}", self.corner_utility(state), self.monotone_utility(state), self.utility_max_tile(state), self.utility_num_empty_tiles(state), state.data()).as_str());
        }
        self.corner_utility(state) + self.monotone_utility(state) + self.utility_max_tile(state)  + self.utility_num_empty_tiles(state)
    }

    pub fn corner_utility(&self, state: &Game<N>) -> f64 {
        // sleep a second
        // std::thread::sleep(std::time::Duration::from_secs(1));
        // println!("Snake utility got this board {:?}", state.data());
        // let stdout_raw = stdout();
        // let mut stdout = BufWriter::new(stdout_raw.lock());
        // let board = board::Board::new();
        // display::display_game(&mut stdout, &board, &state).unwrap();
        let mut corner_score = 0.0 as f64;
        let (i, max_tile) = state.get_max_tile();
        if max_tile <= 2048 || i == 0 {
            for (index, &value) in state.data().iter().enumerate() {
                corner_score += (((index / N) as usize + index % N as usize) * value.pow(6) as usize) as f64;
            }
        } else {
            for (index, &value) in state.data().iter().enumerate() {
                corner_score += ((index / N).abs_diff(i / N) + (index % N).abs_diff(i % N)) as f64 * value.pow(6) as f64;
            }
        }
        -corner_score
    }

    pub fn monotone_utility(&self, state: &Game<N>) -> f64 {
        let mut score = 0.0;
        for i in 0..N {
            if(state.data()[i * N] > state.data()[i * N + 1] && state.data()[i * N + 1] > state.data()[i * N + 2] && state.data()[i * N + 2] > state.data()[i * N + 3]) {
                score += state.data()[i] as f64;
            }
        }
        score
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