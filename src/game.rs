use super::algorithm;
use strum_macros::EnumIter;

use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use rand_xoshiro::Xoshiro256Plus;

#[derive(Clone, EnumIter)]
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
    data: Vec<i32>,
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

    fn horizontal(&mut self, dir: Direction) -> bool {
        let mut mutated = false;
        let mut score = 0;
        let mut won = false;
        self.data.chunks_mut(N).for_each(|row| {
            let (new_row, new_score) = match dir {
                Direction::Right => algorithm::slide_right(row),
                Direction::Left => algorithm::slide_left(row),
                _ => (row.to_vec(), 0),
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
    pub fn get_max_tile(&self) -> i32{
        let mut max = 0;
        for value in self.data().iter() {
            if value > &max {
                max = value.clone();
            }
        }
        max
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
