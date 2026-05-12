pub mod dice;
pub mod edition;
pub mod error;
pub mod hand;
pub mod player;
pub mod transport_segment;

use std::{fmt::Debug, iter, usize::MAX};

use hexgrid::{
    corner::position::CornerPosition,
    edge::position::{EdgeOrientation, EdgePosition},
    hex::position::HexPosition,
};

use crate::{
    board::Board,
    game::{
        edition::GameEdition,
        error::{BuildError, GameError},
        player::{OwnershipToken, Player},
        transport_segment::TransportSegment,
    },
    object::{
        TileType,
        card::ResourceMap,
        resource::ResourceType,
        structure::{
            OwnedStructures,
            building::{Building, BuildingType},
            transport::{Transport, TransportType},
        },
    },
};

#[derive(Debug)]
pub struct Game {
    // main members
    board: Board,
    players: Vec<Player>,
    current_turn: OwnershipToken,
    turn_number: usize,
    // redundant data for ease of use
    buildings: Vec<(Building, Vec<HexPosition>)>,
    transports: Vec<(OwnershipToken, Option<EdgePosition>)>,
}

impl Game {
    pub fn new(edition: impl GameEdition, player_count: usize) -> Result<Self, GameError> {
        if player_count == 0 {
            return Err(GameError::InsufficientPlayerCount);
        }

        let mut players = Vec::with_capacity(player_count);
        let owned_structures = edition.get_start_structures();
        for _ in 0..player_count {
            players.push(Player::new(owned_structures))
        }

        let mut transports = Vec::new();

        for p in players.iter() {
            transports.push((p.token(), None));
        }

        let current_turn = players.get(0).expect("Not enough players!").token();

        Ok(Self {
            board: Board::new(edition),
            players,
            current_turn,
            turn_number: 0,
            buildings: Vec::new(),
            transports,
        })
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
        for corner in self.board.neighboring_corners(position) {
            if self.find_building(corner).is_some() {
                return Err(BuildError::BuildingIsTooCloseToExisting);
            }
        }

        let hexes = self.board.neighboring_hex_for_corner(position);

        let land_count = hexes
            .filter(|p| self.board.get_tile(*p).get_tile_type() != TileType::Water)
            .count();

        if land_count == 0 {
            return Err(BuildError::BuildingIsOnWater);
        }

        let mut same_ownership = 0;
        let mut different_ownership = 0;
        for edge in self.board.neighboring_edges_for_corner(position) {
            if let Some(transport) = self.find_transport(edge) {
                if transport.owner() == building.owner() {
                    same_ownership += 1;
                } else {
                    different_ownership += 1;
                }
            }
        }

        if different_ownership >= 2 {
            return Err(BuildError::BuildingCutsOffRoad);
        }

        // Exception for first turn, where roads have not been placed yet.
        if same_ownership == 0
            && building.r#type() == BuildingType::Settlement
            && self.turn_number > 0
        {
            return Err(BuildError::BuildingHasNoRoad);
        }

        match (building.r#type(), self.find_building(position)) {
            (BuildingType::Settlement, Some(_)) => Err(BuildError::StructureAlreadyExists),
            (BuildingType::City, Some(b)) => {
                if b.r#type() == BuildingType::City {
                    Err(BuildError::StructureAlreadyExists)
                } else if building.owner() != b.owner() {
                    Err(BuildError::CityUpgradeOwnerMismatch)
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

        let turn_number = self.turn_number;

        let player = self.find_player_mut(building.owner());

        player.play_structure(building.into(), turn_number == 0)?;

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

    pub fn can_play_transport(
        &self,
        transport: Transport,
        position: EdgePosition,
    ) -> Result<(), BuildError> {
        if self.find_transport(position).is_some() {
            return Err(BuildError::StructureAlreadyExists);
        }

        let hexes = self.board.neighboring_hex_for_edge(position);

        let land_count = hexes
            .filter(|p| self.board.get_tile(*p).get_tile_type() != TileType::Water)
            .count();

        match transport.r#type() {
            TransportType::Road => {
                if land_count == 0 {
                    return Err(BuildError::RoadMustNeighborLand);
                }
            }
            TransportType::Boat => {
                if land_count == 2 {
                    return Err(BuildError::BoatMustNeighborWater);
                }
            }
        }

        let neighbor_count = self
            .neighboring_transport(transport.owner(), position)
            .count();

        // Exception for first turn, where there are no neighbors.
        if neighbor_count == 0 && self.turn_number > 0 {
            return Err(BuildError::TransportMustBeContiguous);
        }

        let mut unowned_buildings = self
            .board
            .neighboring_corners_for_edge(position)
            .filter(|c| {
                self.find_building(*c)
                    .is_some_and(|b| b.owner() != transport.owner())
            });

        let mut valid_edges = 0;
        for edge in self.neighboring_transport(transport.owner(), position) {
            for corner in self.board.neighboring_corners_for_edge(edge) {
                match unowned_buildings.find(|c| *c == corner) {
                    Some(_) => valid_edges += 1,
                    None => (),
                }
            }

            if valid_edges > 0 {
                break;
            }
        }

        // Exception for first turn, where there are no neighbors.
        if valid_edges == 0 && self.turn_number > 0 {
            return Err(BuildError::TransportInterruptsBuilding);
        }

        Ok(())
    }

    pub fn play_transport(
        &mut self,
        transport: Transport,
        position: EdgePosition,
    ) -> Result<(), BuildError> {
        self.can_play_transport(transport, position)?;

        let turn_number = self.turn_number;

        let player = self.find_player_mut(transport.owner());

        player.play_structure(transport.into(), turn_number == 0)?;

        self.board
            .set_transport(transport, position)
            .expect("Invalid position!");

        self.update_last_played_transport(transport.owner(), position);

        Ok(())
    }

    fn neighboring_transport(
        &self,
        owner: OwnershipToken,
        position: EdgePosition,
    ) -> impl Iterator<Item = EdgePosition> + Clone + Debug {
        self.board
            .neighboring_edges(position)
            .filter(move |p| self.find_transport(*p).is_some_and(|t| t.owner() == owner))
    }

    pub fn distribute_resources(&mut self, roll: u8) {
        for player in self.players.iter_mut() {
            let resources: ResourceMap = self
                .buildings
                .iter()
                .filter(|(b, _)| b.owner() == player.token())
                .map(|(b, pos)| {
                    pos.iter()
                        .filter_map(|p| Self::determine_resource(&self.board, *b, *p, roll))
                })
                .flatten()
                .collect();

            player.add_resources(resources);
        }
    }

    fn determine_resource(
        board: &Board,
        building: Building,
        resource_tile: HexPosition,
        roll: u8,
    ) -> Option<(ResourceType, u32)> {
        if board.get_tile_roll_number(resource_tile)? == roll.into()
            && !board.has_robber(resource_tile)
        {
            Some((
                board.get_resource_type(resource_tile)?,
                if building.r#type() == BuildingType::Settlement {
                    1
                } else {
                    2
                },
            ))
        } else {
            None
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

        while !Self::all_segments_finished(&segments) {
            segments = self.advance_segments(segments);
        }

        if let Some(longest) = Self::find_longest_segment(segments) {
            longest.length()
        } else {
            0
        }
    }

    fn advance_segments(&self, segments: Vec<TransportSegment>) -> Vec<TransportSegment> {
        let mut new_segments: Vec<TransportSegment> = Vec::with_capacity(segments.len());

        for segment in segments {
            if segment.is_finished() {
                new_segments.push(segment);
                continue;
            }

            let current_position = segment.current_position();

            let mut is_building_blocking = false;

            for corner in self.board.neighboring_corners_for_edge(current_position) {
                if segment.is_corner_behind_current(corner) {
                    continue;
                }

                is_building_blocking = self
                    .find_building(corner)
                    .is_some_and(|b| b.owner() != segment.owner());
            }

            if is_building_blocking {
                let mut new_segment = segment.clone();
                new_segment.finished();
                new_segments.push(new_segment);
                continue;
            }

            let neighboring_transport =
                self.neighboring_transport(segment.owner(), segment.current_position());

            let next_positions: Vec<EdgePosition> =
                segment.next_positions(neighboring_transport).collect();

            if next_positions.len() == 0 {
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

    fn find_longest_segment(segments: Vec<TransportSegment>) -> Option<TransportSegment> {
        let mut candidates: Vec<(TransportSegment, TransportSegment)> =
            Vec::with_capacity(segments.len());

        let mut segments = segments.into_iter();
        let mut longest_segment = segments.clone().next()?;

        let mut shortest_overlap: usize = MAX;

        while let Some(segment) = segments.next() {
            if segment.length() > longest_segment.length() {
                longest_segment = segment.clone();
            }

            for other_segment in segments.clone() {
                let overlap = segment.history_overlap(&other_segment).count();
                if overlap < shortest_overlap {
                    shortest_overlap = overlap;
                    candidates.clear();
                    candidates.push((segment.clone(), other_segment));
                } else if overlap == shortest_overlap {
                    candidates.push((segment.clone(), other_segment));
                }
            }
        }

        for (first, second) in candidates.into_iter() {
            let combined = match Self::combine_segments(first, second) {
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
        first: TransportSegment,
        second: TransportSegment,
    ) -> Option<TransportSegment> {
        let overlap = first.history_overlap(&second);

        let mut first_history = first.history();
        let mut second_history = second.history();

        for position in overlap {
            if let Some(index) = first_history.iter().position(|p| *p == position) {
                first_history.remove(index);
            }

            if let Some(index) = second_history.iter().position(|p| *p == position) {
                second_history.remove(index);
            }
        }

        let pos1 = *first_history.get(0)?;
        let pos2 = *second_history.get(0)?;

        first_history.reverse();

        if let Ok(gap) = pos1.find_gap(pos2) {
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

    fn all_segments_finished(segments: &[TransportSegment]) -> bool {
        for segment in segments {
            if segment.is_finished() {
                continue;
            } else {
                return false;
            }
        }

        return true;
    }
}

#[test]
fn test() {
    let edition = edition::CustomEdition::of_size(3, 5)
        .with_owned_structures(OwnedStructures::new(5, 4, 30, 0))
        .build();

    let mut game = Game::new(edition, 2).unwrap();

    let start = (HexPosition::ORIGIN + HexPosition::DOWN_LEFT) + EdgeOrientation::RIGHT;

    let roads: [EdgePosition; 21] = [
        start.into(),
        start.go_down_left().into(),
        start.go_down_left().go_down_left().into(),
        start.go_down_left().go_down_left().go_down_right().into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_down_right()
            .go_down_left()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .go_up_left()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .go_up_left()
            .go_up_left()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .go_up_left()
            .go_up_left()
            .go_up_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .into(),
    ];

    let player1 = game.get_player(0).unwrap().token();
    game.find_player_mut(player1).add_resources(
        ResourceMap::empty()
            .with_resource(ResourceType::Brick, 100)
            .with_resource(ResourceType::Wood, 100),
    );

    for road in roads {
        game.play_transport(Transport::new(TransportType::Road, player1), road)
            .unwrap()
    }

    let longest_road = game.calculate_longest_road(player1);

    println!("The longest road was calculated to be {:}", longest_road);

    assert_eq!(longest_road, 15);
}
