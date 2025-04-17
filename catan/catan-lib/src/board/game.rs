use std::iter;

use hexgrid::hex::position::HexPosition;
use rand::seq::SliceRandom;

use crate::{objects::{ResourceType, TileData}, resource::{ResourceDeck, ResourceDistribution}};

use super::iterators::CircularOrbit;

const ROLL_ORDER_BASE: [u8; 18] = [11, 3, 6, 5, 4, 9, 10, 8, 4, 11, 12, 9, 10, 8, 3, 6, 2, 5];

const ROLL_ORDER_EXP: [u8; 28] = [
    6, 2, 3, 12, 9, 5, 9, 4, 5, 10, 12, 11, 10, 8, 4, 8, 3, 6, 10, 11, 11, 8, 9, 3, 6, 4, 5, 2,
];

const ROLL_NUMBERS: [u8; 10] = [
    6, 8, 5, 9, 4, 10, 3, 11, 2, 12
];

pub trait GameEdition {
    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, TileData)> + Clone;
}

pub struct BaseEdition {}

impl GameEdition for BaseEdition {

    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, TileData)> + Clone {

        let resource_distribution = ResourceDistribution::new([
            (ResourceType::Wood, 4),
            (ResourceType::Brick, 3),
            (ResourceType::Wheat, 4),
            (ResourceType::Sheep, 4),
            (ResourceType::Ore, 3),
        ]);

        let resource_deck= ResourceDeck::new( 19, resource_distribution, &mut ROLL_ORDER_BASE.into_iter());

        CircularOrbit::new(resource_deck, 3, 5)
    }
}

pub struct ExpansionEdition {}

impl GameEdition for ExpansionEdition {

    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, TileData)> + Clone {
        let resource_distribution = ResourceDistribution::new([
            (ResourceType::Wood, 6),
            (ResourceType::Brick, 5),
            (ResourceType::Wheat, 6),
            (ResourceType::Sheep, 6),
            (ResourceType::Ore, 5),
        ]);

        let resource_deck= ResourceDeck::new( 30, resource_distribution, &mut ROLL_ORDER_EXP.into_iter());

        CircularOrbit::new(resource_deck, 3, 6)
    }
}

pub struct CustomEdition {
    shortest: u32,
    longest: u32,
    rsrc_distr: ResourceDistribution,
    roll_numbers: Vec<u8>,
}

impl CustomEdition {
    pub fn of_size(shortest: u32, longest: u32) -> CustomEditionBuilder {
        CustomEditionBuilder::of_size(shortest, longest)
    }
}

impl GameEdition for CustomEdition {
    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, TileData)> + Clone {
        let size: usize = ((self.longest - self.shortest) * 2 + 1) as usize;
        let resource_deck = ResourceDeck::new(size, self.rsrc_distr.clone(), &mut self.roll_numbers.clone().into_iter());

        CircularOrbit::new(resource_deck, self.shortest, self.longest)
    }
}

pub struct CustomEditionBuilder {
    shortest: u32,
    longest: u32,
    rsrc_distr: ResourceDistribution,
    roll_numbers: Vec<u8>
}

impl CustomEditionBuilder {
    pub fn build(self) -> CustomEdition {
        CustomEdition {
            shortest: self.shortest,
            longest: self.longest,
            rsrc_distr: self.rsrc_distr,
            roll_numbers: self.roll_numbers
        }
    }

    pub fn of_size(shortest: u32, longest: u32) -> CustomEditionBuilder {
        CustomEditionBuilder { 
            shortest,
            longest,
            rsrc_distr: Self::default_resource_distribution(shortest, longest),
            roll_numbers: Self::default_roll_numbers(shortest, longest)
        }
    }

    pub fn with_resource_distribution(mut self, distr: ResourceDistribution) -> CustomEditionBuilder {
        self.rsrc_distr = distr;
        self
    }

    pub fn with_roll_numbers(mut self, roll_numbers: Vec<u8>) -> CustomEditionBuilder {
        self.roll_numbers = roll_numbers;
        self
    }


    fn default_resource_distribution(shortest: u32, longest: u32) -> ResourceDistribution {
        let size: f64 = ((longest - 1) * longest - (shortest - 1) * shortest + longest) as f64;
        ResourceDistribution::new([
            (ResourceType::Wood, (size/5.).round() as u32),
            (ResourceType::Brick, (size/6.).round() as u32),
            (ResourceType::Wheat, (size/5.).round() as u32),
            (ResourceType::Sheep, (size/5.).round() as u32),
            (ResourceType::Ore, (size/6.).round() as u32)
        ])
    }

    fn default_roll_numbers(shortest: u32, longest: u32) -> Vec<u8> {
        let size: usize = ((longest - 1) * longest - (shortest - 1) * shortest + longest) as usize;
        let mut roll_numbers: Vec<u8> = iter::repeat_n(ROLL_NUMBERS, (size/ROLL_NUMBERS.len()) + 1).flatten().collect();
        roll_numbers.truncate(size);
        roll_numbers.shuffle(&mut rand::rng());

        roll_numbers

    }
}