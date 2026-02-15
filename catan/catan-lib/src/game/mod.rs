pub mod dice;
pub mod edition;
pub mod error;
pub mod hand;
pub mod player;

use hexgrid::{
    corner::position::{CornerPosition, Height},
    edge::position::{EdgePosition, Valid},
};

use crate::{
    board::Board,
    game::{
        dice::Dice, error::BuildError, player::{OwnershipToken, Player}
    },
    object::structure::{building::Building, transport::Transport},
};

#[derive(Debug)]
pub struct Game {
    // main members
    board: Board,
    players: Vec<Player>,
    // redundant data for ease of use
}

impl Game {
    pub fn find_player(&self, token: OwnershipToken) -> &Player {
        self.players.iter().find(|p| p.token() == token).unwrap()
    }

    pub fn find_player_mut(&mut self, token: OwnershipToken) -> &mut Player {
        self.players
            .iter_mut()
            .find(|p| p.token() == token)
            .unwrap()
    }

    pub fn play_building<H: Height>(
        &mut self,
        building: Building,
        position: CornerPosition<H>,
    ) -> Result<(), BuildError> {
        let player = self.find_player_mut(building.owner());

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
        let player = self.find_player_mut(transport.owner());

        player.play_structure(transport.into())?;

        self.board
            .set_transport(transport, position)
            .expect("Invalid position!");

        Ok(())
    }
}
