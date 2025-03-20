use rand::{seq::SliceRandom, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use strum_macros::EnumIter;

use crate::{algorithm, game::{self, Direction, Game, GameStatus}};

#[derive(Clone, EnumIter)]
pub enum ThreeDDirection {
    Up,
    Down,
    Left,
    Right,
    Inward,
    Outward,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ThreeDGame<const N: usize> {
    status: GameStatus,
    already_won: bool,
    score: i32,
    data: Vec<i32>,
    dimention: i32,
}
impl <const N: usize> ThreeDGame<N> {
    pub fn new() -> Self {
        let mut rng = Xoshiro256Plus::from_entropy();
        let mut data = vec![0; N * N * N];
        data[0] = 1;
        data[1] = 1;
        data.shuffle(&mut rng);
        ThreeDGame {
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
        let mut copy: ThreeDGame<N> = self.clone();
        !(copy.right() || copy.left() || copy.up() || copy.down())
    }

    fn horizontal(&mut self, dir: ThreeDDirection) -> bool {
        let mut mutated = false;
        let mut score = 0;
        let mut won = false;
        self.data.chunks_mut(N).for_each(|row| {
            let (new_row, new_score, _) = match dir {
                ThreeDDirection::Right => algorithm::slide_right(row),
                ThreeDDirection::Left => algorithm::slide_left(row),
                _ => (row.to_vec(), 0, false),
            };
            if new_score == 2048 {
                won = true;
            }
            score += new_score;
            for i in 0..N {
                if row[i] != new_row[i] {
                    row[i] = new_row[i];
                    mutated = true;
                }
            }
        });
        self.score += score;
        if won && !self.already_won {
            self.status = GameStatus::Won;
            self.already_won = true;
        }
        mutated
    }
    fn vertical(&mut self, dir: ThreeDDirection) -> bool {
        algorithm::transpose_3d::<N>(&mut self.data);
        let mutated = match dir {
            ThreeDDirection::Up => self.left(),
            ThreeDDirection::Down => self.right(),
            _ => false,
        };
        algorithm::transpose_3d::<N>(&mut self.data);
        mutated
    }
    
    fn inward_outward(&mut self, dir: ThreeDDirection) -> bool {
        let mut mutated = false;
        let mut score = 0;
        let mut won = false;
    
        // Iterate over each (i, j) column
        for i in 0..N {
            for j in 0..N {
                // Extract the depth slice for (i, j)
                let mut depth = (0..N)
                    .map(|k| self.data[i + N * (j + N * k)])
                    .collect::<Vec<_>>();
    
                // Slide and merge the depth slice
                let (new_depth, new_score, _) = match dir {
                    ThreeDDirection::Inward => algorithm::slide_left(&depth),
                    ThreeDDirection::Outward => algorithm::slide_right(&depth),
                    _ => (depth.clone(), 0, false),
                };
    
                // Check for winning tile
                if new_score == 2048 {
                    won = true;
                }
    
                // Update score
                score += new_score;
    
                // Write back the modified depth slice and check for mutations
                for k in 0..N {
                    let idx = i + N * (j + N * k);
                    if self.data[idx] != new_depth[k] {
                        self.data[idx] = new_depth[k];
                        mutated = true;
                    }
                }
            }
        }
    
        // Update game score and status
        self.score += score;
        if won && !self.already_won {
            self.status = GameStatus::Won;
            self.already_won = true;
        }
    
        mutated
    }
    pub fn print_boards(&self) {
        println!("Game State ({}x{}x{}):", N, N, N);
        for k in 0..N {
            println!("Layer {}:", k + 1); // Print the layer header
            for j in 0..N {
                for i in 0..N {
                    let idx = i + N * (j + N * k);
                    print!("{} ", self.data[idx]); // Print row elements without extra spaces
                }
                println!(); // Move to the next row
            }
            println!(); // Separate layers with a blank line
        }
    }

    

    pub fn new_tile_xy(&mut self, x: i32, y :i32, z: i32, value: i32) {
        self.data[(x * self.dimention * self.dimention + y * self.dimention + z) as usize] = value;
    }
    pub fn new_tile(&mut self, position :usize, value: i32) {
        self.data[position] = value;
    }

    pub fn right(&mut self) -> bool {
        self.horizontal(ThreeDDirection::Right)
    }
    pub fn left(&mut self) -> bool {
        self.horizontal(ThreeDDirection::Left)
    }
    pub fn up(&mut self) -> bool {
        self.vertical(ThreeDDirection::Up)
    }
    pub fn down(&mut self) -> bool {
        self.vertical(ThreeDDirection::Down)
    }
    pub fn inward(&mut self) -> bool {
        self.inward_outward(ThreeDDirection::Inward)
    }
    pub fn outward(&mut self) -> bool {
        self.inward_outward(ThreeDDirection::Outward)
    }

    pub fn movement(&mut self, direction: &ThreeDDirection) -> bool {
        match direction {
            ThreeDDirection::Up => self.up(),
            ThreeDDirection::Left => self.left(),
            ThreeDDirection::Right => self.right(),
            ThreeDDirection::Down => self.down(),
            ThreeDDirection::Inward => self.inward(),
            ThreeDDirection::Outward => self.outward(),
            _ => false,
        }
    }
    pub fn get_tiles_sum(&self) -> i32{
        let mut sum = 0;
        for value in self.data().iter() {
            sum += value;
        }
        sum
    }
    pub fn get_max_tile(&self) -> i32{
        let mut max = 0;
        for value in self.data().iter() {
            if value > &max {
                max = value.clone();
            }
        }
        2i32.pow(max as u32)
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
        self.new_tile(*empty_tiles.choose(&mut rand::thread_rng()).unwrap() as usize, *[1,2].choose(&mut rand::thread_rng()).unwrap());
    }


}
