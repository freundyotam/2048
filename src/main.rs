#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
use std::collections::HashMap;
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
use std::fs::{File, OpenOptions};
use chrono::prelude::*;
use rand::Rng; // 0.8.5

fn main() -> Result<(), std::io::Error>{
    let stdout_raw = stdout();
    let mut stdout = BufWriter::new(stdout_raw.lock());
    crossterm::terminal::enable_raw_mode()?;
    let board = board::Board::new();
    const BOARD_DIMISION: usize = 4;
    let game: Game<BOARD_DIMISION> = game::Game::new();
    let depth = 3;
    display::display_game(&mut stdout, &board, &game)?.flush()?;


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


    let mut no_bettter_solution = 0;
    for _i in 0..33 {


    current_alpha = global_aplha + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_beta = global_beta + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_gamma = global_gamma + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_delta = global_delta + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_lambda = global_lambda + rand::thread_rng().gen_range(-1..1) as f64 / 10.0;
    current_points = 0;
    
    //Create file:
    let file = File::create("results.csv")?;
    let mut csv_writer = BufWriter::new(file);

    // Write the CSV header
    writeln!(csv_writer, "Game Iterations,Max Tile,Score,8,16,32,64,128,256,512,1024,2048,4096,8192")?;


    for _j in 0..3 {
        
        let mut strategy = ExpectimaxStrategy::<BOARD_DIMISION>::new(depth, current_alpha, current_beta, current_gamma, current_delta, current_lambda);
        let mut game: Game<BOARD_DIMISION> = game::Game::new();
        let mut iterations = 0;

        let mut first_occurrence: HashMap<i32, usize> = HashMap::new();

        loop {
            if game.check_if_lost() {
                current_points += game.get_max_tile().1;
                break;
            }
            let best_move: Option<Direction> = strategy.calculate_next_move(&game);
            match best_move {
                Some(_) => {
                    let best_move = best_move.unwrap();
                    game.movement(&best_move);
                    
                    let (_, max_tile) = game.get_max_tile();
                    if !first_occurrence.contains_key(&max_tile) {
                        first_occurrence.insert(max_tile, iterations);
                    }

                    if game.get_empty_tiles().len() > 0 {
                        game.new_random_tile();
                    }
                    display::display_game(&mut stdout, &board, &game)?.flush()?;
                }
                None => {
                    current_points += game.get_max_tile().1;
                    break;
                }
            }
            
            iterations += 1;

        }

        let (_, max_tile) = game.get_max_tile();
        let score = game.score();

        // Save the result as a CSV row
        writeln!(csv_writer, "{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
          iterations,
           max_tile,
            score,
            *first_occurrence.get(&8).unwrap_or(&0),
            *first_occurrence.get(&16).unwrap_or(&0),
            *first_occurrence.get(&32).unwrap_or(&0),
            *first_occurrence.get(&64).unwrap_or(&0),
            *first_occurrence.get(&128).unwrap_or(&0),
            *first_occurrence.get(&256).unwrap_or(&0),
            *first_occurrence.get(&512).unwrap_or(&0),
            *first_occurrence.get(&1024).unwrap_or(&0),
            *first_occurrence.get(&2048).unwrap_or(&0),
            *first_occurrence.get(&4096).unwrap_or(&0),
            *first_occurrence.get(&8192).unwrap_or(&0)
        )?;

    }
    if current_points > global_max_points {
        global_max_points = current_points;
        global_aplha = current_alpha;
        global_beta = current_beta;
        global_gamma = current_gamma;
        global_delta = current_delta;
        global_lambda = current_lambda;
        // writeln!(file, " found better solution: alpha is : {}, beta is : {}, gamma is : {}, delta is {}, max wins is : {}", global_aplha, global_beta, global_gamma, global_delta, global_max_points);
    } else {
        // writeln!(file, "no better solution found");
        no_bettter_solution += 1;
        if no_bettter_solution > 10 {
            global_max_points = 0;
            global_aplha = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            global_beta = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            global_gamma = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            global_delta = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            global_lambda = rand::thread_rng().gen_range(1..10) as f64 / 10.0;
            no_bettter_solution = 0;
            // writeln!(file, "restaring with new values");
        }
    }   
        // writeln!(file, "finished with current alpha is : {}, beta is : {}, gamma is : {}, delta is {}, max wins is : {}", current_alpha, current_beta, current_gamma, current_delta, current_points);
    }

    Ok(())
}

