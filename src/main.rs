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
    let depth = 2;
    display::display_game(&mut stdout, &board, &game)?.flush()?;


    let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(format!("day_1_{:?}_size_{:?}_time_{:?}.txt", depth, BOARD_DIMISION, chrono::offset::Local::now()).as_str())
            .unwrap();

    let mut global_max_points = 0;
    let mut global_aplha = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
    let mut global_beta = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
    let mut global_gamma = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
    let mut global_delta = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
    let mut global_lambda = rand::thread_rng().gen_range(1..10) as f64 / 10.0;

    let mut current_points = 0;
    let mut current_alpha = 0.0;
    let mut current_beta = 0.0;
    let mut current_gamma = 0.0;
    let mut current_delta = 0.0;
    let mut current_lambda = 0.0;
//     for _ in 0..500 {
//         current_alpha = rand::thread_rng().gen_range(0..10) as f64 / 10.0;
//         current_beta = rand::thread_rng().gen_range(0..10) as f64 / 10.0;
//         current_gamma = rand::thread_rng().gen_range(0..10) as f64 / 10.0;
//         current_delta = rand::thread_rng().gen_range(0..10) as f64 / 10.0;
//         current_lambda = rand::thread_rng().gen_range(0..10) as f64 / 10.0;
//         current_points = 0;
//         writeln!(file, "var to begin with alpha is : {}, beta is : {}, gamma is : {}, delta is : {}, lambda is : {}", current_alpha, current_beta, current_gamma, current_delta, current_lambda);
//         for _ in 0..5 {
//             let mut strategy = ExpectimaxStrategy::<BOARD_DIMISION>::new(depth, current_alpha, current_beta, current_gamma, current_delta, current_lambda);
//             let mut game: Game<BOARD_DIMISION> = game::Game::new();
//             while true {
//                 if game.check_if_lost() {
//                     writeln!(file, "resetting game");
//                     current_points += game.get_max_tile().1;
//                     if game.get_max_tile().1 >= 2048 {
//                         writeln!(file, "game won, max tile is : {}", format!("{:?}", game.get_max_tile().1).as_str());
//                     }
//                     break;
//                 }
//                 let best_move: Option<Direction> = strategy.calculate_next_move(&game);
//                 match best_move {
//                     Some(_) => {
//                         let best_move = best_move.unwrap();
//                         game.movement(&best_move);
//                         if game.get_empty_tiles().len() > 0 {
//                             game.new_random_tile();
//                         }
//                         display::display_game(&mut stdout, &board, &game)?.flush()?;
//                     }
//                     None => {
//                         writeln!(file, "resetting game");
//                         current_points += game.get_max_tile().1;
//                         if game.get_max_tile().1 >= 2048 {
//                             writeln!(file, "game won, max tile is : {}", format!("{:?}", game.get_max_tile().1).as_str());
//                         }
//                         break;
//                     }
//                 }   
//             }
//         }
//         if current_points > global_max_points {
//             global_max_points = current_points;
//             global_aplha = current_alpha;
//             global_beta = current_beta;
//             global_gamma = current_gamma;
//             global_delta = current_delta;
//             global_lambda = current_lambda;
//             writeln!(file, " found better solution: alpha is : {}, beta is : {}, gamma is : {}, delta is {}, max wins is : {}", global_aplha, global_beta, global_gamma, global_delta, global_max_points);
//         } else {
//             writeln!(file, " no better solution found");
//         }   
//         writeln!(file, "finished with current alpha is : {}, beta is : {}, gamma is : {}, delta is {}, max wins is : {}", current_alpha, current_beta, current_gamma, current_delta, current_points);
//    }


    let mut no_bettter_solution = 0;
   for _ in 0..500 {
    current_alpha = global_aplha + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_beta = global_beta + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_gamma = global_gamma + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_delta = global_delta + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_lambda = global_lambda + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_points = 0;
    writeln!(file, "var to begin with alpha is : {}, beta is : {}, gamma is : {}, delta is : {}, lambda is : {}", current_alpha, current_beta, current_gamma, current_delta, current_lambda);
    for _ in 0..3 {
        let mut strategy = ExpectimaxStrategy::<BOARD_DIMISION>::new(depth, current_alpha, current_beta, current_gamma, current_delta, current_lambda);
        let mut game: Game<BOARD_DIMISION> = game::Game::new();
        while true {
            if game.check_if_lost() {
                writeln!(file, "resetting game");
                current_points += game.get_max_tile().1;
                if game.get_max_tile().1 >= 2048 {
                    writeln!(file, "game won, max tile is : {}", format!("{:?}", game.get_max_tile().1).as_str());
                }
                break;
            }
            let best_move: Option<Direction> = strategy.calculate_next_move(&game);
            match best_move {
                Some(_) => {
                    let best_move = best_move.unwrap();
                    game.movement(&best_move);
                    if game.get_empty_tiles().len() > 0 {
                        game.new_random_tile();
                    }
                    display::display_game(&mut stdout, &board, &game)?.flush()?;
                }
                None => {
                    writeln!(file, "resetting game");
                    current_points += game.get_max_tile().1;
                    if game.get_max_tile().1 >= 2048 {
                        writeln!(file, "game won, max tile is : {}", format!("{:?}", game.get_max_tile().1).as_str());
                    }
                    break;
                }
            }   
        }
    }
    if current_points > global_max_points {
        global_max_points = current_points;
        global_aplha = current_alpha;
        global_beta = current_beta;
        global_gamma = current_gamma;
        global_delta = current_delta;
        global_lambda = current_lambda;
        writeln!(file, " found better solution: alpha is : {}, beta is : {}, gamma is : {}, delta is {}, max wins is : {}", global_aplha, global_beta, global_gamma, global_delta, global_max_points);
    } else {
        writeln!(file, "no better solution found");
        no_bettter_solution += 1;
        if no_bettter_solution > 10 {
            global_max_points = 0;
            global_aplha = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            global_beta = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            global_gamma = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            global_delta = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            global_lambda = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            no_bettter_solution = 0;
            writeln!(file, "restaring with new values");
        }
    }   
        writeln!(file, "finished with current alpha is : {}, beta is : {}, gamma is : {}, delta is {}, max wins is : {}", current_alpha, current_beta, current_gamma, current_delta, current_points);
    }



   
   
  
    


    // for _ in 0..110 {    
    //     let mut strategy = ExpectimaxStrategy::<BOARD_DIMISION>::new(depth, 0.5, 0.8, 0.6, 0.0, 0.2);
    //     let mut game: Game<BOARD_DIMISION> = game::Game::new();
    //     while true {
    //         let best_move: Option<Direction> = strategy.calculate_next_move(&game);
    //         match best_move {
    //             Some(_) => {
    //                 let best_move = best_move.unwrap();
    //                 game.movement(&best_move);
    //                 if game.get_empty_tiles().len() == 0 {
    //                     writeln!(file, "reseting game, depth is : {}", format!("{:?}", depth).as_str());
    //                     let best_move: Option<Direction> = strategy.calculate_next_move(&game);
    //                     let mut game: Game<BOARD_DIMISION> = game::Game::new();
    //                     break;
    //                 }
    //                 game.new_random_tile();
    //                 writeln!(file, "{}", format!("{:?}", game.data()).as_str()); 
    //                 display::display_game(&mut stdout, &board, &game)?.flush()?;
    //             }
    //             None => {
    //                 writeln!(file, "resetting game");
    //                 let best_move: Option<Direction> = strategy.calculate_next_move(&game);
    //                 let mut game: Game<BOARD_DIMISION> = game::Game::new();
    //                 break;
    //             }
    //         }   
    //     }
    // }
    Ok(())
}

