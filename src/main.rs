#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod algorithm;
mod board;
mod display;
mod game;
mod strategies;
use crate::strategies::expectimax::ExpectimaxStrategy;
use core::time;
use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::Duration;
use game::Game;
use strategies::strategy::Strategy;



fn main() -> Result<(), std::io::Error>{
    let stdout_raw = stdout();
    let mut stdout = BufWriter::new(stdout_raw.lock());
    crossterm::terminal::enable_raw_mode()?;
    let board = board::Board::new();
    const BOARD_DIMISION: usize = 4;
    let mut game: Game<BOARD_DIMISION> = game::Game::new();

    display::display_game(&mut stdout, &board, &game)?.flush()?;
    let mut strategy = ExpectimaxStrategy::<BOARD_DIMISION>::new(3);
    loop {
        let best_move = strategy.calculate_next_move(&game);
        game.movement(&best_move.unwrap());
        game.new_random_tile();
        display::display_game(&mut stdout, &board, &game)?.flush()?;
        thread::sleep(Duration::from_millis(100));
    }

}
