#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod algorithm;
mod board;
mod display;
mod game;
mod strategies;

use crate::strategies::expectimax::ExpectimaxStrategy;
use core::time;
use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::Duration;
use game::{Direction, Game};
use strategies::strategy::Strategy;

use std::fs::File;
use crossterm::terminal;

fn main() -> Result<(), std::io::Error> {
    let stdout_raw = stdout();
    let mut stdout = BufWriter::new(stdout_raw.lock());
    terminal::enable_raw_mode()?;

    // Create a CSV file to save the results
    let file = File::create("results.csv")?;
    let mut csv_writer = BufWriter::new(file);

    // Write the CSV header
    writeln!(csv_writer, "Iteration,Game Iterations,Max Tile,Score,4,8,16,64,128,256,512,1024,2048,4096")?;

    for iteration in 1..=100 {
        let board = board::Board::new();
        
        //Select board dimention
        const BOARD_DIMENSION: usize = 5;

        let mut game: Game<BOARD_DIMENSION> = game::Game::new();
        let mut iterations = 0;
        let mut first_occurrence: HashMap<i32, usize> = HashMap::new();
        
        display::display_game(&mut stdout, &board, &game)?.flush()?;
        let mut strategy = ExpectimaxStrategy::<BOARD_DIMENSION>::new(3);

        loop {
            iterations += 1;
            let best_move = strategy.calculate_next_move(&game);
            if best_move.is_none() {
                break;
            }
            game.movement(&best_move.unwrap());
            let max_tile = game.get_max_tile();
            if !first_occurrence.contains_key(&max_tile) {
                first_occurrence.insert(max_tile, iterations);
            }
            game.new_random_tile();
            display::display_game(&mut stdout, &board, &game)?.flush()?;
        }

        let max_tile = game.get_max_tile();
        let score = game.score();

        // Save the result as a CSV row
        writeln!(csv_writer, "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
         iteration,
          iterations,
           max_tile,
            score,
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
            *first_occurrence.get(&4096).unwrap_or(&0)
        )?;

        // Print the result for monitoring
        println!(
            "Iteration {}: Game Iterations = {}, Max Tile = {}, Score = {}",
            iteration, iterations, max_tile, score
        );

        thread::sleep(Duration::from_millis(1000));
    }

    // Flush the CSV writer to ensure all data is written
    csv_writer.flush()?;

    println!("Results saved to 'results.csv'.");

    Ok(())
}
