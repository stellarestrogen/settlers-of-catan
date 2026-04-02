pub mod dice;
pub mod edition;
pub mod error;
pub mod hand;
pub mod player;
pub mod transport_segment;

use std::{fmt::Debug, iter, usize::MAX};

use hexgrid::{
    corner::position::CornerPosition, edge::position::EdgePosition, hex::position::HexPosition,
};

use crate::{
    board::Board,
    game::{
        edition::GameEdition,
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
    transports: Vec<(OwnershipToken, Option<EdgePosition>)>,
}

impl Game {
    pub fn new(edition: impl GameEdition, player_count: u32) -> Self {
        let mut players = Vec::with_capacity(player_count as usize);
        let owned_structures = edition.get_start_structures();
        for _ in 0..player_count {
            players.push(Player::new(owned_structures))
        }

        let mut transports = Vec::new();

        for p in players.iter() {
            transports.push((p.token(), None));
        }

        let current_turn = players.get(0).expect("Not enough players!").token();

        Self {
            board: Board::new(edition),
            players,
            current_turn,
            buildings: Vec::new(),
            transports,
        }
    }

    pub fn tick(&mut self, roll: u8) {}

    pub fn get_player(&self, player_number: usize) -> Option<&Player> {
        self.players.get(player_number)
    }

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
        if self.find_transport(position).is_some() {
            return Err(BuildError::StructureAlreadyExists);
        }

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

    /// Big boi!
    ///
    /// Iterates through all placed transports, keeping track when new branches appear, and then when all branches
    /// are dead-ended, find the combination that is the biggest.
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

        if let Some(longest) = self.find_longest_segment(segments.into_iter()) {
            longest.length()
        } else {
            0
        }
    }

    fn find_longest_segment(
        &self,
        mut segments: impl Iterator<Item = TransportSegment> + Clone + Debug,
    ) -> Option<TransportSegment> {
        if segments.clone().count() == 1 {
            return segments.next();
        }

        let mut shortest_overlap: usize = MAX;
        let mut segment_candidates: Vec<(TransportSegment, TransportSegment)> = Vec::new();
        let mut longest_segment = segments.clone().next()?;

        while let Some(segment) = segments.next() {
            if segment.length() > longest_segment.length() {
                longest_segment = segment.clone();
            }

            for other_segment in segments.clone() {
                let overlap = segment.history_overlap(&other_segment).count();
                if overlap < shortest_overlap {
                    shortest_overlap = overlap;
                    segment_candidates.clear();
                    segment_candidates.push((segment.clone(), other_segment.clone()));
                } else if overlap == shortest_overlap {
                    segment_candidates.push((segment.clone(), other_segment.clone()));
                }
            }
        }

        if segment_candidates.len() == 0 {
            return None;
        }

        for (first, second) in segment_candidates.into_iter() {
            let combined = match self.combine_segments(first, second) {
                Some(segment) => segment,
                None => continue,
            };

            if combined.length() > longest_segment.length() {
                longest_segment = combined;
            }
        }

        Some(longest_segment)
    }

    fn combine_segments(
        &self,
        first: TransportSegment,
        second: TransportSegment,
    ) -> Option<TransportSegment> {
        let overlap = first.history_overlap(&second);

        let mut first_history = first.history();
        let mut second_history = second.history();

        for _ in 0..overlap.count() {
            first_history.remove(0);
            second_history.remove(0);
        }

        let pos1 = *first_history.get(0)?;
        let pos2 = *second_history.get(0)?;

        first_history.reverse();

        if let Some(gap) = pos1.find_gap(pos2) {
            first_history.push(gap);
        }

        let combined_segment: Vec<EdgePosition> = first_history
            .into_iter()
            .chain(second_history.into_iter())
            .collect();

        let segment = TransportSegment::from_history(first.owner(), combined_segment);

        if segment.is_continuous()? {
            Some(segment)
        } else {
            None
        }
    }

    fn advance_segments(
        &self,
        segments: impl Iterator<Item = TransportSegment> + Clone + Debug,
    ) -> Vec<TransportSegment> {
        let mut new_segments: Vec<TransportSegment> = Vec::with_capacity(segments.clone().count());

        for segment in segments {
            if segment.is_finished() {
                new_segments.push(segment);
                continue;
            }

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
    ) -> impl Iterator<Item = EdgePosition> + Clone + Debug {
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
            *p = Some(position);
        }
    }

    fn get_last_played_transport(&self, owner: OwnershipToken) -> Option<EdgePosition> {
        if let Some((_, p)) = self.transports.iter().find(|(o, _)| *o == owner).copied() {
            p
        } else {
            None
        }
    }
}
