use crate::{board::Board, game::player::Player};

pub mod edition;
pub mod error;
pub mod hand;
pub mod player;
pub mod structure;

#[derive(Debug)]
pub struct Game {
    // main members
    board: Board,
    players: Vec<Player>,
    // redundant data for ease of use
}

impl Game {}
