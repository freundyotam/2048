# 2048 AI in Rust  

This project is an AI implementation for the **2048 game**, built in **Rust**. It uses **Expectimax search**, and includes
evaluation functions, making intelligent moves and maximizing the final score.  

## Features
- **Rust implementation** for high performance and efficiency 
- **AI-controlled gameplay** using the **Expectimax** algorithm  
- **Heuristic-based scoring**, including **snake shape sum**  
- **Flexible board sizes**, supporting **2x2, 3x3, 4x4, and 5x5** configurations  

## How to Run 
1. Install Rust and Cargo:

   For installation guide, check out:
   https://doc.rust-lang.org/cargo/getting-started/installation.html

2. Run the game:
   ```sh
   cargo run


## Project Source Structure

### `main.rs` 
This is the **main file** of the program, where everything starts. It sets up the **game board**, configures the **Algorithm strategy**, and controls the **game loop**.

In this file, using the constant config vars developers can:  
- **Set the board size** (e.g., 3x3, 4x4, or 5x5)  
- **Adjust the Algorithm’s search depth** to control how far ahead it plans moves  
- **Choose how many games to run**

### `game.rs`
This file the game module logic, defining what happens for each step the user decides to take, which moves are allowed, and when the game ends.  

#### This file contains the following:  
- **Handling player moves** → Implements movement functions like `left`, `right`, `up`, and `down`.  
- **Checking for game over** → Determines if no valid moves are left.  
- **Spawning new tiles** → Randomly adds a `2` or `4` after each move.
- **Handling heuristic-based scoring calculations** → Some heuristic needs specific values of the game that this module calculates for them

### `three_d_game.rs`
This file is the same as `game.rs` but for the 3D board. 
Implementing all the board functionality for the algirthm to run on a 3D board.

### `display.rs` 
Handles the **visual aspects** of the game and prints the board nicely to the stdout

### `expectimax_three_d.rs`
The expectimax algorithm implementation for the 3D board

### `board.rs`
Manages the **board setup**, assigns **colors to tiles**, and **displays game results**.

Some of the file's code is **sourced from the resorce project**, which is referenced in the External Resources section at the end of this README.

### `algorithm.rs` 
Implements **movement logic** and **post-move board adjustments** to ensure proper tile behavior

Some of the file's code is **sourced from the resorce project**, which is referenced in the External Resources section at the end of this README.

###  `strategies/mod.rs`
This file serves as a module declaration, making two modules public for access in other files.

The two modules are:
-  `Strategy`: Contains the definitions and implementations related to game movement.
-  `Expectimax`: contain logic for the Expectimax algorithm.

### `strategies/strategy.rs`
This file implements the strategy for game movements, where the Expectimax search is applied to determine the best move based on the game state.

### `strategies/expectimax.rs`
This file implements the **Expectimax Search**. It is responsible for evaluating game states, handling state repeatations, and selecting the optimal move to maximize the score.
#### Key components:
- Expectimax recursive search to simulate multiple future moves.
- Probabilistic handling of tile spawns (90% chance for 2, 10% for 4), with expectation evaluation for each move.
- Scoring functions using heuristics like snake shape sum, max tile to corner, empty tiles and more.
- Cache implementation for repeating states (for further explanation, take a look on page 11 and 24 of our report)



## External Resource

This project incorporates code from the following source: 

[https://github.com/pierrechevalier83/2048-rs](https://github.com/pierrechevalier83/2048-rs/tree/master/src)
