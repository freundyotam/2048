#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod algorithm;
mod board;
mod display;
mod game;

use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::Duration;

use game::Game;
mod strategies;



fn main() -> Result<(), std::io::Error>{
    let stdout_raw = stdout();
    let mut stdout = BufWriter::new(stdout_raw.lock());
    crossterm::terminal::enable_raw_mode()?;
    let board = board::Board::new();
    let mut game: Game<5> = game::Game::new();

    display::display_game(&mut stdout, &board, &game)?.flush()?;
    let mut strategy = strategies::ExpectimaxStrategy::<5>::new();
    loop {
        let (_best_score, best_move) = strategy.expectimax(&game, 3);
        game.movement(&best_move.unwrap());
        game.new_random_tile();
        display::display_game(&mut stdout, &board, &game)?.flush()?;
        thread::sleep(Duration::from_millis(100));
    }

}
