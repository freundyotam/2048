use strum_macros::EnumIter;
use rand::prelude::*;
use crate::algorithm;
use rand::distributions::WeightedIndex;
use std::fs::File;
use std::io::{BufWriter, Result, Write};

use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use rand_xoshiro::Xoshiro256Plus;

#[derive(Clone, EnumIter, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, EnumIter, Eq, Hash)]
pub enum GameStatus {
    Ongoing,
    Won,
    Lost,
    Interrupted,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Game<const N: usize> {
    status: GameStatus,
    already_won: bool,
    score: i32,
    pub data: Vec<i32>,
    dimention: i32,
}

impl <const N: usize> Game<N> {
    pub fn new() -> Self {
        let mut rng = Xoshiro256Plus::from_entropy();
        let mut data = vec![0; N * N];
        data[0] = 1;
        data[1] = 1;
        data.shuffle(&mut rng);
        Game {
            status: GameStatus::Ongoing,
            already_won: false,
            score: 0,
            data,
            dimention: 4,
        }
    }
    pub fn data(&self) -> &Vec<i32> {
        &self.data
    }
    pub fn score(&self) -> i32 {
        self.score
    }
    pub fn status(&self) -> &GameStatus {
        &self.status
    }
    pub fn interrupt(&mut self) {
        self.status = GameStatus::Interrupted;
    }
    pub fn go_on(&mut self) {
        self.status = GameStatus::Ongoing;
    }
    pub fn check_if_lost(&self) -> bool {
        let mut copy: Game<N> = self.clone();
        !(copy.right() || copy.left() || copy.up() || copy.down())
    }

    pub fn transpose(&self) -> Self {
        let mut data = self.data.clone();
        algorithm::transpose::<N>(&mut data);
        Game {
            status: GameStatus::Ongoing,
            already_won: self.already_won,
            score: self.score,
            data,
            dimention: self.dimention,
        }
    }

    fn horizontal(&mut self, dir: Direction) -> bool {
        let mut mutated = false;
        let mut score = 0;
        let mut won = false;
        self.data.chunks_mut(N).for_each(|row| {
            let (new_row, new_score, is_moving) = match dir {
                Direction::Right => algorithm::slide_right(row),
                Direction::Left => algorithm::slide_left(row),
                _ => (row.to_vec(), 0, false),
            };
            if new_score == 2048 {
                won = true;
            }
            score += new_score;
            
            if is_moving == true {
                mutated = true;
            }

            for i in 0..N {
                row[i] = new_row[i];
            }
        });

        self.score += score;
        if won && !self.already_won {
            self.status = GameStatus::Won;
            self.already_won = true;
        }
        
        mutated
    }
    fn vertical(&mut self, dir: Direction) -> bool {
        algorithm::transpose::<N>(&mut self.data);
        let mutated = match dir {
            Direction::Up => self.left(),
            Direction::Down => self.right(),
            _ => false,
        };
        algorithm::transpose::<N>(&mut self.data);
        mutated
    }
    pub fn new_tile_xy(&mut self, x: i32, y :i32) {
        self.data[(x * self.dimention + y) as usize] = 2;
    }
    pub fn new_tile(&mut self, position :usize, value: i32) {
        self.data[position] = value;
    }

    pub fn right(&mut self) -> bool {
        self.horizontal(Direction::Right)
    }
    pub fn left(&mut self) -> bool {
        self.horizontal(Direction::Left)
    }
    pub fn up(&mut self) -> bool {
        self.vertical(Direction::Up)
    }
    pub fn down(&mut self) -> bool {
        self.vertical(Direction::Down)
    }

    pub fn movement(&mut self, direction: &Direction) -> bool {
        match direction {
            Direction::Up => self.up(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
            _ => false,
        }
    }
    pub fn get_state(&mut self) -> [[i32; 4]; 6]{

        let mut state : [[i32; 4]; 6] = [[0; 4]; 6];
        for (index, value) in self.data().iter().enumerate() {
            state[index/4][index%4] = value.clone();
        }
        state
    }
    pub fn get_tiles_sum(&self) -> i32{
        let mut sum = 0;
        for value in self.data().iter() {
            sum += value;
        }
        sum
    }
    pub fn get_max_tile(&self) -> (usize, i32){
        let mut max = 0;
        let mut index = 0;
        for (i, value) in self.data().iter().enumerate() {
            if value > &max {
                max = value.clone();
                index = i;
            }
        }
        (index, 2i32.pow(max as u32))
    }
    
    pub fn get_empty_tiles(&self) -> Vec<u32> {
        let mut empty_tiles: Vec<u32> = Vec::new();
        for (index, &value) in self.data.iter().enumerate() {
            if value == 0 {
                empty_tiles.push(index as u32);
            }
        }
        empty_tiles
    }
    
    pub fn new_random_tile(&mut self) {
        let empty_tiles = self.get_empty_tiles();
        self.new_tile(
            *empty_tiles.choose(&mut rand::thread_rng()).unwrap() as usize,
            {
                let mut rng = rand::thread_rng();
                let weights = [90, 10];
                let dist = WeightedIndex::new(&weights).unwrap();
                [1, 2][dist.sample(&mut rng)]
            },
        );
    }

    pub fn get_tiles_snake_sum(&self) -> f64 {

        let mut sum = 0.0;
        if N == 4 {
            sum = self.get_tiles_snake_sum_4x4();
        }

        if N == 2 {
            sum = self.get_tiles_snake_sum_2x2();
        }
        
        if N == 3 {
            sum = self.get_tiles_snake_sum_3x3();
        }

        if N == 5 {
            sum = self.get_tiles_snake_sum_5x5();
        }

        sum
    }


    pub fn get_tiles_snake_sum_2x2(&self) -> f64 {
        let weight_matrix = [
            [512.0, 128.0],       
            [8.0, 32.0]            
        ];
    
        let mut sum: f64 = 0.0;
    
        for i in 0..2 {
            for j in 0..2 {
                sum += self.get_tile(i, j) as f64 * weight_matrix[i][j];
            }
        }
    
        sum
    }

    pub fn get_tiles_snake_sum_3x3(&self) -> f64 {
        let weight_matrix = [
            [512.0, 1024.0, 2048.0], // Highest row
            [256.0, 128.0, 64.0],       // Zigzag down
            [8.0, 16.0, 32.0]            // Lowest row
        ];
    
        let mut sum: f64 = 0.0;
    
        for i in 0..3 {
            for j in 0..3 {
                sum += self.get_tile(i, j) as f64 * weight_matrix[i][j];
            }
        }
    
        sum
    }
    


    pub fn get_tiles_snake_sum_4x4(&self) -> f64 {
        let weight_matrix = [
            [65536.0, 32768.0, 16384.0, 8192.0],  // Highest priority row
            [512.0, 1024.0, 2048.0, 4096.0],     // Zigzag down
            [256.0, 128.0, 64.0, 32.0],          // Continue snake
            [2.0, 4.0, 8.0, 16.0]                 // Lowest priority row
        ];
    
        let mut sum: f64 = 0.0;
    
        for i in 0..4 {
            for j in 0..4 {
                sum += self.get_tile(i, j) as f64 * weight_matrix[i][j];
            }
        }
    
        sum
    }



    pub fn get_tiles_snake_sum_5x5(&self) -> f64 {
        let weight_matrix = [
            [1048576.0, 524288.0, 262144.0, 131072.0, 65536.0], // Highest priority row
            [2048.0, 4096.0, 8192.0, 16384.0, 32768.0],         // Zigzag down
            [1024.0, 512.0, 256.0, 128.0, 64.0],               // Continue snake
            [2.0, 4.0, 8.0, 16.0, 32.0],                       // Keep priority decreasing
            [1.0, 0.5, 0.25, 0.125, 0.025]                          // Lowest priority row
        ];
    
        let mut sum: f64 = 0.0;
    
        for i in 0..5 {
            for j in 0..5 {
                sum += self.get_tile(i, j) as f64 * weight_matrix[i][j];
            }
        }
    
        sum
    }

    

    pub fn get_smoothness(&self) -> f64 {
        let mut smoothness: f64 = 0.0;
    
        for i in 0..N {
            for j in 0..N {
                let tile_value = self.get_tile(i, j) as f64;
                if tile_value == 0.0 {
                    continue; // Skip empty tiles
                }
    
                // Check right neighbor
                if j + 1 < N {
                    let right_value = self.get_tile(i, j + 1) as f64;
                    if right_value > 0.0 {
                        smoothness -= (tile_value - right_value).abs();
                    }
                }
    
                // Check bottom neighbor
                if i + 1 < N {
                    let bottom_value = self.get_tile(i + 1, j) as f64;
                    if bottom_value > 0.0 {
                        smoothness -= (tile_value - bottom_value).abs();
                    }
                }
            }
        }
    
        smoothness // Negative, since lower absolute difference is better
    }

    pub fn get_merging_potential(&self) -> f64 {
        let mut merges: f64 = 0.0;
    
        for i in 0..N {
            for j in 0..N {
                let tile_value = self.get_tile(i, j) as f64;
                if tile_value == 0.0 {
                    continue; // Skip empty tiles
                }
    
                // Check right neighbor
                if j + 1 < N && self.get_tile(i, j) == self.get_tile(i, j + 1) {
                    merges += tile_value;
                }
    
                // Check bottom neighbor
                if i + 1 < N && self.get_tile(i, j) == self.get_tile(i + 1, j) {
                    merges += tile_value;
                }
            }
        }
    
        merges
    }


    pub fn get_monotonicaly(&self) -> f64 {
        let mut value = 0.0;

        for i in 0..(N*N - 1) {
            let diff = (self.data[i] - self.data[i+1]).abs() as f64;
            value -= diff;
        }
        value
    }

    pub fn get_tile(&self, row: usize, col: usize) -> i32 {
        let index = N * row + col;
        let value = self.data[index].clone() as u32;
        if value == 0 {
            return 0;
        }
        2i32.pow(value)
    }

    pub fn print_board(&self, txt_writer: &mut BufWriter<File>) -> Result<()> {
        for i in 0..N {
            for j in 0..N {
                if self.data[i * N + j] == 0 {
                    write!(txt_writer, "|{:4}|", 0)?;
                }
                else {
                    write!(txt_writer, "|{:4}|", 2i32.pow(self.data[i * N + j] as u32))?;
                }
                
            }
            writeln!(txt_writer)?; // Newline after each row
        }
        writeln!(txt_writer)?; // Extra newline for separation
        Ok(())
    }
}
