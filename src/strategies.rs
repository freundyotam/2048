use crate::game::Game;
use crate::game::Direction;
use strum::IntoEnumIterator;
use std::f64;


pub fn expectimax(state: &Game, depth: usize) -> (f64, Option<Direction>) {
    if depth == 0 || state.check_if_lost() {
        return (utility(state), None);
    }

    let mut best_score: f64 = 0.0;
    let mut best_move = None;
    for step in Direction::iter() {
        let mut state_after_my_turn = state.clone();
        if !state_after_my_turn.movement(&step){ // Staying in the same state is not a valid move
            continue;
        }
        let mut expected_value: f64 = 0.0;
        let empty_tiles_list = state_after_my_turn.get_empty_tiles();
        let empty_list_len = empty_tiles_list.len();
        for empty_index in empty_tiles_list {
            let mut state_after_new_tile = state_after_my_turn.clone();
            state_after_new_tile.new_tile(empty_index as usize);
            let (score, _) = expectimax(&state_after_new_tile, depth - 1);
            expected_value += (1.0 / empty_list_len as f64) as f64 * score;
            if expected_value > best_score {
                best_score = expected_value;
                best_move = Some(step.clone());
            }
        }
    }
    (best_score, best_move)
}


pub fn utility(state: &Game) -> f64 {
   (state.get_tiles_sum() as f64) * 0.0  + (state.get_max_tile() as f64) * 1.0
}