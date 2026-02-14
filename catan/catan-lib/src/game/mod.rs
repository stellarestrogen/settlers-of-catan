use hexgrid::{
    corner::position::{CornerPosition, Height},
    edge::position::{EdgePosition, Valid},
};

use crate::{
    board::Board,
    game::{error::BuildError, player::Player},
    object::structure::{building::Building, transport::Transport},
};

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
    pub fn play_building<H: Height>(
        &mut self,
        building: Building,
        position: CornerPosition<H>,
    ) -> Result<(), BuildError> {
        let player = self
            .players
            .iter_mut()
            .find(|p| p.token() == building.owner())
            .expect("Invalid Player ID!");

        player.play_structure(building.into())?;

        self.board
            .set_building(building, position)
            .expect("Invalid position!");

        Ok(())
    }

    pub fn play_transport<T: Valid>(
        &mut self,
        transport: Transport,
        position: EdgePosition<T>,
    ) -> Result<(), BuildError> {
        let player = self
            .players
            .iter_mut()
            .find(|p| p.token() == transport.owner())
            .expect("Invalid Player ID!");

        player.play_structure(transport.into())?;

        self.board
            .set_transport(transport, position)
            .expect("Invalid position!");

        Ok(())
    }
}
