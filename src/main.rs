#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
use std::process::exit;
use std::time::SystemTime;
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
use game::{Direction, Game};
use strategies::strategy::Strategy;
use std::fs::OpenOptions;
use chrono::prelude::*;
use rand::Rng; // 0.8.5

fn main() -> Result<(), std::io::Error>{
    let stdout_raw = stdout();
    let mut stdout = BufWriter::new(stdout_raw.lock());
    crossterm::terminal::enable_raw_mode()?;
    let board = board::Board::new();
    const BOARD_DIMISION: usize = 4;
    let mut game: Game<BOARD_DIMISION> = game::Game::new();
    let depth = 4;
    display::display_game(&mut stdout, &board, &game)?.flush()?;


    let mut alpha = 0.425;
    let mut beta =  0.13;
    let mut gamma = 0.767;
    let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(format!("log_games_depth_{:?}_size_{:?}_time_{:?}_gamma_again2.txt", depth, BOARD_DIMISION, chrono::offset::Local::now()).as_str())
            .unwrap();

    // let mut max_points = 0;
    // let mut old_max_points = 0;
    // let mut old_alpha = 0.0;
    // let mut old_beta = 0.0;
    // let mut old_gamma = 0.0;
    // for _ in 0..500 {
    //     alpha = rand::thread_rng().gen_range(0..1000) as f64 / 1000.0;
    //     beta = rand::thread_rng().gen_range(0..1000) as f64 / 1000.0;
    //     gamma = rand::thread_rng().gen_range(0..1000) as f64 / 1000.0;
    //     for _ in 0..30 {
    //         let mut strategy = ExpectimaxStrategy::<BOARD_DIMISION>::new(depth, alpha, beta, gamma);
    //         let mut game: Game<BOARD_DIMISION> = game::Game::new();
    //         while true {
    //             let best_move: Option<Direction> = strategy.calculate_next_move(&game);
    //             match best_move {
    //                 Some(_) => {
    //                     let best_move = best_move.unwrap();
    //                     game.movement(&best_move);
    //                     if game.get_empty_tiles().len() == 0 {
    //                         // writeln!(file, "reseting game, depth is : {}", format!("{:?}", depth).as_str());
    //                         if game.get_max_tile().1 >= 2048 {
    //                             writeln!(file, "game won, max tile is : {}", format!("{:?}", game.get_max_tile().0).as_str());
    //                             max_points += game.get_max_tile().1;
    //                         }
    //                         break;
    //                     }
    //                     game.new_random_tile();
    //                     // writeln!(file, "{}", format!("{:?}", game.data()).as_str()); 
    //                     display::display_game(&mut stdout, &board, &game)?.flush()?;
    //                 }
    //                 None => {
    //                     writeln!(file, "resetting game");
    //                     if game.get_max_tile().1 >= 2048 {
    //                         writeln!(file, "game won, max tile is : {}", format!("{:?}", game.get_max_tile().0).as_str());
    //                         max_points += game.get_max_tile().1;
    //                     }
    //                     break;
    //                 }
    //             }   
            
    //         }
    //     }
    //     if max_points > old_max_points {
    //         old_max_points = max_points;
    //         old_alpha = alpha;
    //         old_beta = beta;
    //         old_gamma = gamma;
    //     }    
    //     writeln!(file, "alpha is : {}, beta is : {}, gamma is : {}, max wins is : {}", old_alpha, old_beta, old_gamma, old_max_points);

    // }
    // writeln!(file, "alpha is : {}, beta is : {}, gamma is : {}, max wins is : {}", old_alpha, old_beta, old_gamma, old_max_points)
    


    for _ in 0..110 {
        let mut strategy = ExpectimaxStrategy::<BOARD_DIMISION>::new(depth, alpha, beta, gamma);
        let mut game: Game<BOARD_DIMISION> = game::Game::new();
        while true {
            let best_move: Option<Direction> = strategy.calculate_next_move(&game);
            match best_move {
                Some(_) => {
                    let best_move = best_move.unwrap();
                    game.movement(&best_move);
                    if game.get_empty_tiles().len() == 0 {
                        writeln!(file, "reseting game, depth is : {}", format!("{:?}", depth).as_str());
                        let best_move: Option<Direction> = strategy.calculate_next_move(&game);
                        let mut game: Game<BOARD_DIMISION> = game::Game::new();
                        break;
                    }
                    game.new_random_tile();
                    writeln!(file, "{}", format!("{:?}", game.data()).as_str()); 
                    display::display_game(&mut stdout, &board, &game)?.flush()?;
                }
                None => {
                    writeln!(file, "resetting game");
                    let best_move: Option<Direction> = strategy.calculate_next_move(&game);
                    let mut game: Game<BOARD_DIMISION> = game::Game::new();
                    break;
                }
            }   
        }
    }
    writeln!(file, "alpha is : {}, beta is : {}, gamma is : {}", alpha, beta, gamma)
}

