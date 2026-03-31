pub mod dice;
pub mod edition;
pub mod error;
pub mod hand;
pub mod player;
pub mod transport_segment;

use std::iter;

use hexgrid::{
    corner::position::CornerPosition, edge::position::EdgePosition, hex::position::HexPosition,
};

use crate::{
    board::Board,
    game::{
        error::BuildError,
        player::{OwnershipToken, Player},
        transport_segment::TransportSegment,
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
    current_turn: OwnershipToken,
    // redundant data for ease of use
    buildings: Vec<(Building, Vec<HexPosition>)>,
    transports: Vec<(OwnershipToken, EdgePosition)>,
}

impl Game {
    pub fn tick(&mut self, roll: u8) {}

    pub fn find_player(&self, token: OwnershipToken) -> &Player {
        self.players.iter().find(|p| p.token() == token).unwrap()
    }

    pub fn find_player_mut(&mut self, token: OwnershipToken) -> &mut Player {
        self.players
            .iter_mut()
            .find(|p| p.token() == token)
            .unwrap()
    }

    pub fn next_turn(&mut self) {
        self.current_turn = self
            .players
            .iter()
            .cycle()
            .skip_while(|p| p.token() != self.current_turn)
            .next()
            .unwrap()
            .token();
    }

    pub fn find_building(&self, position: CornerPosition) -> Option<Building> {
        self.board.get_building(position)
    }

    pub fn can_play_building(
        &self,
        building: Building,
        position: CornerPosition,
    ) -> Result<(), BuildError> {
        // todo!
        // check if the building is connected to at least 1 road of the same ownership, and
        // that it is not able to cut off 2 roads of different ownership (only applies to settlements)

        for p in self.board.neighboring_corners(position) {
            if self.find_building(p).is_some() {
                return Err(BuildError::BuildingIsTooCloseToExisting);
            }
        }

        match (building.r#type(), self.find_building(position)) {
            (BuildingType::Settlement, Some(_)) => Err(BuildError::StructureAlreadyExists),
            (BuildingType::City, Some(b)) => {
                if b.r#type() == BuildingType::City {
                    Err(BuildError::StructureAlreadyExists)
                } else if building.owner() != b.owner() {
                    Err(BuildError::CityUpgradeTokenMismatch)
                } else {
                    Ok(())
                }
            }
            (BuildingType::Settlement, None) => Ok(()),
            (BuildingType::City, None) => Err(BuildError::CityRequiresSettlement),
        }
    }

    pub fn play_building(
        &mut self,
        building: Building,
        position: CornerPosition,
    ) -> Result<(), BuildError> {
        self.can_play_building(building, position)?;

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

    pub fn find_transport(&self, position: EdgePosition) -> Option<Transport> {
        self.board.get_transport(position)
    }

    pub fn play_transport(
        &mut self,
        transport: Transport,
        position: EdgePosition,
    ) -> Result<(), BuildError> {
        // todo!
        // figure out if the road to be built is contiguous, and account for edge cases
        self.find_transport(position)
            .ok_or(BuildError::StructureAlreadyExists)?;

        let player = self.find_player_mut(transport.owner());

        player.play_structure(transport.into())?;

        self.board
            .set_transport(transport, position)
            .expect("Invalid position!");

        self.update_last_played_transport(transport.owner(), position);

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
                        if self.board.get_tile_roll_number(*p)? == roll.into()
                            && !self.board.has_robber(*p)
                        {
                            Some((
                                self.board.get_resource_type(*p)?,
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

    pub fn calculate_longest_road(&self, owner: OwnershipToken) -> u32 {
        let last_road = match self.get_last_played_transport(owner) {
            Some(p) => p,
            None => return 0,
        };

        let segment = TransportSegment::new(owner, last_road);

        let mut segments: Vec<TransportSegment> = iter::once(segment).collect();

        while !self.all_segments_finished(segments.clone().into_iter()) {
            segments = self.advance_segments(segments.into_iter());
        }

        // the 2 longest segments with the least overlap combined is the longest road


        0
    }



    fn advance_segments(
        &self,
        segments: impl Iterator<Item = TransportSegment> + Clone,
    ) -> Vec<TransportSegment> {
        let mut new_segments: Vec<TransportSegment> = Vec::with_capacity(segments.clone().count());

        for segment in segments {
            let neighboring_transport =
                self.neighboring_transport(segment.owner(), segment.current_position());
            let next_positions = segment.next_positions(neighboring_transport);

            if next_positions.clone().count() == 0 {
                let mut new_segment = segment.clone();
                new_segment.finished();
                new_segments.push(new_segment);
                continue;
            }

            for position in next_positions {
                let mut new_segment = segment.clone();
                new_segment.update(position);
                new_segments.push(new_segment);
            }
        }

        new_segments
    }

    fn neighboring_transport(
        &self,
        owner: OwnershipToken,
        position: EdgePosition,
    ) -> impl Iterator<Item = EdgePosition> + Clone {
        self.board.neighboring_edges(position).filter(move |p| {
            self.board
                .get_transport(*p)
                .is_some_and(|t| t.owner() == owner)
        })
    }

    fn all_segments_finished(
        &self,
        segments: impl Iterator<Item = TransportSegment> + Clone,
    ) -> bool {
        for segment in segments {
            if segment.is_finished() {
                continue;
            } else {
                return false;
            }
        }

        return true;
    }

    fn update_last_played_transport(&mut self, owner: OwnershipToken, position: EdgePosition) {
        if let Some((_, p)) = self.transports.iter_mut().find(|(o, _)| *o == owner) {
            *p = position;
        }
    }

    fn get_last_played_transport(&self, owner: OwnershipToken) -> Option<EdgePosition> {
        if let Some((_, p)) = self.transports.iter().find(|(o, _)| *o == owner).copied() {
            Some(p)
        } else {
            None
        }
    }
}
