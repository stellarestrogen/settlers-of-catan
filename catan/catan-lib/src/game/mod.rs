pub mod dice;
pub mod edition;
pub mod error;
pub mod hand;
pub mod player;

use hexgrid::{
    corner::position::{CornerPosition, Height},
    edge::position::{EdgePosition, Valid},
    hex::position::HexPosition,
};

use crate::{
    board::Board,
    game::{
        error::BuildError,
        player::{OwnershipToken, Player},
    },
    object::{
        card::ResourceMap,
        structure::{
            building::{Building, BuildingType},
            transport::Transport,
        },
    },
};

#[derive(Debug)]
pub struct Game {
    // main members
    board: Board,
    players: Vec<Player>,
    // redundant data for ease of use
    buildings: Vec<(Building, Vec<HexPosition>)>,
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

        self.buildings.push((
            building,
            self.board.neighboring_hex_for_corner(position).collect(),
        ));

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

    pub fn distribute_resources(&mut self, roll: u8) {
        for player in self.players.iter_mut() {
            let resources: ResourceMap = self
                .buildings
                .iter()
                .filter(|(b, _)| b.owner() == player.token())
                .filter_map(|(b, pos)| {
                    Some(pos.iter().filter_map(|p| {
                        if self.board.get_tile(*p).get_roll_number()? == roll.into()
                            && !self.board.has_robber(*p)
                        {
                            Some((
                                self.board
                                    .get_tile(*p)
                                    .get_tile_type()
                                    .get_resource_type()?,
                                if b.r#type() == BuildingType::Settlement {
                                    1
                                } else {
                                    2
                                },
                            ))
                        } else {
                            None
                        }
                    }))
                })
                .flatten()
                .collect();

            player.add_resources(resources);
        }
    }
}
