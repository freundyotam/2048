use crate::game::Game;
use crate::game::Direction;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::f64;
use std::io::Empty;



pub fn expectimax(state: &Game, depth: usize) -> f64 {
    if depth == 0 || state.check_if_lost() {
        return utility(state);
    }

    let mut best_score: f64 = 0.0;
    for step in Direction::iter() {
        let mut state_after_my_turn = state.clone();
        if !state_after_my_turn.movement(step){ // Staying in the same state is not a valid move
            continue;
        }
        let mut expected_value: f64 = 0.0;
        let empty_tiles_list = state_after_my_turn.get_empty_tiles();
        let empty_list_len = empty_tiles_list.len();
        for empty_index in empty_tiles_list {
            let mut state_after_new_tile = state_after_my_turn.clone();
            state_after_new_tile.new_tile(empty_index as usize);
            let score = expectimax(&state_after_new_tile, depth - 1);
            expected_value += (1 / empty_list_len) as f64 * score;
            if (expected_value > best_score){
                best_score = expected_value;
            }
        }
    }
    best_score
}

// pub fn best_move(game: &Game, depth: usize) -> Option<Game> {
//     let possible_moves = game.get_possible_moves();
//     if possible_moves.is_empty() {
//         return None;
//     }

//     let mut best_score = f64::MIN;
//     let mut best_move = None;

//     for next_game in possible_moves {
//         let score = expectimax(&next_game, depth - 1);
//         if score > best_score {
//             best_score = score;
//             best_move = Some(next_game);
//         }
//     }

//     best_move
// }

pub fn utility(state: &Game) -> f64 {
   0.0
}