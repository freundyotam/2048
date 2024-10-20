use crate::game::Game;
use crate::game::Direction;


pub trait Strategy<const N: usize> {
    // Trait method to be implemented by all strategies.
    fn calculate_next_move(&mut self, game: &Game<N>) -> Option<Direction>;
}