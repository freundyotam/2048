# 2048 AI in Rust 🎮🦀  

This project is an AI implementation for the **2048 game**, built in **Rust**. It uses **Expectimax search**, and includes
evaluation functions, making intelligent moves and maximizing the final score.  

## Features 🚀  
- **Rust implementation** for high performance and efficiency 
- **AI-controlled gameplay** using the **Expectimax** algorithm  
- **Heuristic-based scoring**, including **snake shape sum**  
- **Flexible board sizes**, supporting **2x2, 3x3, 4x4, and 5x5** configurations  

## How to Run 🛠️  
1. Install Rust:

   Visit:
   https://www.rust-lang.org/tools/install

   Or run this command:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/2048.git
   cd 2048

3. Run the game:
   ```sh
   cargo run


## Project Source Structure 📂  

### `main.rs` 🏠  
This is the **main file** of the program, where everything starts. It sets up the **game board**, configures the **AI strategy**, and controls the **game loop**.  

In this file, developers can:  
- **Set the board size** (e.g., 3x3, 4x4, or 5x5)  
- **Adjust the AI’s search depth** to control how far ahead it plans moves  
- **Choose how many games to run**

### `game.rs` 🎲  
This file contains the **core game logic**, defining how the board works, which moves are allowed, and when the game ends.  

#### Key functionalities:  
- **Handling player and AI moves** → Implements movement functions like `left`, `right`, `up`, and `down`.  
- **Checking for game over** → Determines if no valid moves are left.  
- **Spawning new tiles** → Randomly adds a `2` or `4` after each move.
- **Handling heuristic-based scoring calculations**

**Most of the file's code is taken from our main contributor, a link is appeared in the end**

### `display.rs` 🖥️  
Handles the **visual aspects** of the game, including the **header, footer, and board rendering**.
**Most of the file's code is taken from our contributor, a link is appeared in the end**

### `board.rs` 🔲  
Manages the **board setup**, assigns **colors to tiles**, and **displays game results**.
**Most of the file's code is taken from our contributor, a link is appeared in the end**

### `algorithm.rs` 🧩  
Implements **movement logic** and **post-move board adjustments** to ensure proper tile behavior.
**Most of the file's code is taken from our main contributor, a link is appeared in the end**

###  `strategies/mod.rs` 🎨
This file serves as a module declaration, making two modules public for access in other files.

The two modules are:
-  `Strategy`: Contains the definitions and implementations related to game movement.
-  `Expectimax`: contain logic for the Expectimax algorithm.

### `strategies/strategy.rs` 🤖
This file implements the strategy for game movements, where the Expectimax search is applied to determine the best move based on the game state.

### `strategies/expectimax.rs` 🧠
This file implements the **Expectimax Search**. It is responsible for evaluating game states and selecting the optimal move to maximize the score.
#### Key components:
- Expectimax recursive search to simulate multiple future moves.
- Probabilistic handling of tile spawns (90% chance for 2, 10% for 4), with expectation evaluation for each move.
- Scoring functions using heuristics like snake shape sum, max tile to corner, empty tiles and more.



## Contributor 🤝
We downloaded this github repositpry bellow:

https://github.com/pierrechevalier83/2048-rs
