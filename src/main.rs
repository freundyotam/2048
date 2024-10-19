#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod algorithm;
mod board;
mod display;
mod game;
mod strategies;

fn main(){
    let board = board::Board::new();
    let mut game = game::Game::new();
    println!("{:?}\n{:?}\n{:?}\n{:?}\n\n", game.get_state()[0], game.get_state()[1], game.get_state()[2], game.get_state()[3]);
    
    println!("{:?}\n{:?}\n{:?}\n{:?}\n\n", game.get_state()[0], game.get_state()[1], game.get_state()[2], game.get_state()[3]);

    println!("{:?}\n{:?}\n{:?}\n{:?}\n\n", game.get_state()[0], game.get_state()[1], game.get_state()[2], game.get_state()[3]);

    strategies::expectimax(&game, 3);

}
