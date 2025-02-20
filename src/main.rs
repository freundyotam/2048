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
    const BOARD_DIMISION: usize = 5;
    let game: Game<BOARD_DIMISION> = game::Game::new();
    let depth = 2;
    display::display_game(&mut stdout, &board, &game)?.flush()?;


    //Create file:
    let file = File::create("results_5x5.csv")?;

    // let file = File::create(format!("random_{:?}size{:?}time{:?}.txt", depth, BOARD_DIMISION, chrono::offset::Local::now()).as_str());

    // let file = OpenOptions::new()
    //         .write(true)
    //         .append(true)
    //         .create(true)
    //         .open(format!("snake_{:?}size{:?}time{:?}.txt", depth, BOARD_DIMISION, chrono::offset::Local::now()).as_str())
    //         .unwrap();

    let mut csv_writer = BufWriter::new(file);

    // Write the CSV header
    writeln!(csv_writer, "Game Iterations,Max Tile,Score,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192")?;


    let mut no_bettter_solution = 0;
    for _i in 0..3 {
        
        let mut strategy = ExpectimaxStrategy::<BOARD_DIMISION>::new(depth, 0.0, 0.0,0.0,0.0,0.0);
        let mut game: Game<BOARD_DIMISION> = game::Game::new();
        let mut iterations = 0;

        let mut first_occurrence: HashMap<i32, usize> = HashMap::new();

        loop {
            if game.check_if_lost() {
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
                    break;
                }
            }
            
            iterations += 1;

        }

        let (_, max_tile) = game.get_max_tile();
        let score = game.score();

        // Save the result as a CSV row
        writeln!(csv_writer, "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
          iterations,
           max_tile,
            score,
            *first_occurrence.get(&2).unwrap_or(&0),
            *first_occurrence.get(&4).unwrap_or(&0),
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
    
    Ok(())
}

