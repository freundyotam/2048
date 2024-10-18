#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod algorithm;
mod board;
mod display;
mod game;

use core::time;
use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::QueueableCommand;
use game::GameStatus;

fn main(){
    let board = board::Board::new();
    let mut game = game::Game::new();
    println!("{:?}\n{:?}\n{:?}\n{:?}\n\n", game.get_state()[0], game.get_state()[1], game.get_state()[2], game.get_state()[3]);
    game.movement(KeyCode::Left);
    println!("{:?}\n{:?}\n{:?}\n{:?}\n\n", game.get_state()[0], game.get_state()[1], game.get_state()[2], game.get_state()[3]);

    game.movement(KeyCode::Right);
    println!("{:?}\n{:?}\n{:?}\n{:?}\n\n", game.get_state()[0], game.get_state()[1], game.get_state()[2], game.get_state()[3]);

    game.movement(KeyCode::Up);
    println!("{:?}\n{:?}\n{:?}\n{:?}\n\n", game.get_state()[0], game.get_state()[1], game.get_state()[2], game.get_state()[3]);

    game.new_tile();
    println!("{:?}\n{:?}\n{:?}\n{:?}\n\n", game.get_state()[0], game.get_state()[1], game.get_state()[2], game.get_state()[3]);

    

}
