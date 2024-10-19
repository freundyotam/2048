#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod algorithm;
mod board;
mod display;
mod game;

use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::QueueableCommand;
use game::GameStatus;
mod strategies;



fn main() -> Result<(), std::io::Error>{
    let board = board::Board::new();
    let mut game = game::Game::new();
    let stdout_raw = stdout();
    let mut stdout = BufWriter::new(stdout_raw.lock());
    crossterm::terminal::enable_raw_mode()?;
    let board = board::Board::new();
    let mut game = game::Game::new();

    display::display_game(&mut stdout, &board, &game)?.flush()?;
    loop {
        let (best_score, best_move) = strategies::expectimax(&game, 4);
        game.movement(&best_move.unwrap());
        game.new_random_tile();
        display::display_game(&mut stdout, &board, &game)?.flush()?;
        thread::sleep(Duration::from_millis(100));
    }

}
