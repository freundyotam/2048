use super::algorithm;
use crossterm::event::KeyCode;
use matrix_display::matrix::position;
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

#[derive(Clone, PartialEq, EnumIter)]
pub enum GameStatus {
    Ongoing,
    Won,
    Lost,
    Interrupted,
}

#[derive(Clone)]
pub struct Game {
    status: GameStatus,
    already_won: bool,
    score: i32,
    data: [i32; 16],
    rng: Xoshiro256Plus,
    dimention: i32,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = Xoshiro256Plus::from_entropy();
        let mut data = [0; 16];
        data[0] = 1;
        data[1] = 1;
        data.shuffle(&mut rng);
        Game {
            status: GameStatus::Ongoing,
            already_won: false,
            score: 0,
            data,
            rng,
            dimention: 4,
        }
    }
    pub fn data(&self) -> [i32; 16] {
        self.data
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
        let mut copy: Game = self.clone();
        !(copy.right() || copy.left() || copy.up() || copy.down())
    }

    fn horizontal(&mut self, dir: Direction) -> bool {
        let mut mutated = false;
        let mut score = 0;
        let mut won = false;
        self.data.chunks_mut(4).for_each(|row| {
            let (new_row, new_score) = match dir {
                Direction::Right => algorithm::slide_right(row),
                Direction::Left => algorithm::slide_left(row),
                _ => (row.to_vec(), 0),
            };
            if new_score == 2048 {
                won = true;
            }
            score += new_score;
            for i in 0..4 {
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
        algorithm::transpose(&mut self.data);
        let mutated = match dir {
            Direction::Up => self.left(),
            Direction::Down => self.right(),
            _ => false,
        };
        algorithm::transpose(&mut self.data);
        mutated
    }
    pub fn new_tile_xy(&mut self, x: i32, y :i32) {
        let value = if self.rng.gen::<i32>() % 10 == 1 {
            4
        } else {
            2
        };

        self.data[(x * self.dimention + y) as usize] = value;
    }
    pub fn new_tile(&mut self, position :usize) {
        let value = if self.rng.gen::<i32>() % 10 == 1 {
            4
        } else {
            2
        };

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

    pub fn movement(&mut self, direction: Direction) -> bool {
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
    pub fn get_tiles_sum(&mut self) -> i32{
        let mut sum = 0;
        for value in self.data().iter() {
            sum += value;
        }
        sum
    }
    pub fn get_max_tile(&mut self) -> i32{
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
}
