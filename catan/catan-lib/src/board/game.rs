use hexgrid::hex::position::HexPosition;

use crate::{objects::{ResourceType, Tile}, resource::{ResourceDeck, ResourceDistribution}};

use super::draw_iters::CircularOrbit;

const ROLL_ORDER_BASE: [u8; 18] = [11, 3, 6, 5, 4, 9, 10, 8, 4, 11, 12, 9, 10, 8, 3, 6, 2, 5];

const ROLL_ORDER_EXP: [u8; 28] = [
    6, 2, 3, 12, 9, 5, 9, 4, 5, 10, 12, 11, 10, 8, 4, 8, 3, 6, 10, 11, 11, 8, 9, 3, 6, 4, 5, 2,
];

pub trait GameEdition {
    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, Tile)> + Clone;
}

pub struct BaseEdition {}

impl BaseEdition {}

impl GameEdition for BaseEdition {

    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, Tile)> + Clone {

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

impl ExpansionEdition {}

impl GameEdition for ExpansionEdition {

    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, Tile)> + Clone {
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
    pub fn new(
        shortest: u32,
        longest: u32,
        rsrc_distr: ResourceDistribution,
        roll_numbers: Vec<u8>,
    ) -> Self {
        CustomEdition {
            shortest,
            longest,
            rsrc_distr,
            roll_numbers,
        }
    }
}

impl GameEdition for CustomEdition {


    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, Tile)> + Clone {
        let size: usize = ((self.longest - self.shortest) * 2 + 1) as usize;
        let resource_deck = ResourceDeck::new(size, self.rsrc_distr.clone(), &mut self.roll_numbers.clone().into_iter());

        CircularOrbit::new(resource_deck, self.shortest, self.longest)
    }
}
