use crate::{board::Board, game::player::Player, object::structure::Structure};

pub mod edition;
pub mod error;
pub mod hand;
pub mod player;

#[derive(Debug)]
pub struct Game {
    // main members
    board: Board,
    players: Vec<Player>,
    // redundant data for ease of use
}

impl Game {
    pub fn build_structure(&mut self, structure: Structure) {
        
    }
}
