#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod algorithm;
mod board;
mod display;
mod game;
mod three_d_game;
mod strategies;
use crate::strategies::expectimax::ExpectimaxStrategy;
use crate::strategies::expectimax_three_d::ExpectimaxStrategyThreeD;
use core::time;
use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::Duration;
use game::Game;
use strategies::strategy::Strategy;
use three_d_game::ThreeDDirection;



fn main() -> Result<(), std::io::Error>{
    let stdout_raw = stdout();
    let mut stdout = BufWriter::new(stdout_raw.lock());
    crossterm::terminal::enable_raw_mode()?;
    let board = board::Board::new();
    const BOARD_DIMISION: usize = 4;
    let mut game: Game<BOARD_DIMISION> = game::Game::new();
    let mut game3d: three_d_game::ThreeDGame<BOARD_DIMISION> = three_d_game::ThreeDGame::new();
    let mut strategy = ExpectimaxStrategyThreeD::<BOARD_DIMISION>::new(3, 0.9, 0.1, 0.1, 0.1, 0.1);
    loop {
        let best_move = strategy.calculate_next_move(&game3d);
        game3d.movement(&best_move.unwrap());
        game3d.new_random_tile();
        thread::sleep(Duration::from_millis(100));
    }

}
