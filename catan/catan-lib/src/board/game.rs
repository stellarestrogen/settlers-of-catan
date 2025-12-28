use std::iter;

use hexgrid::hex::{iterators::spiral::HexSpiral, position::HexPosition};
use rand::seq::SliceRandom;

use crate::{
    objects::{ResourceType, TileData, TradePort, TradeType},
    resource::{ResourceDeck, ResourceDistribution},
    trade::{TradeDistribution, TradePortDeck},
};

const ROLL_ORDER_BASE: [u8; 18] = [11, 3, 6, 5, 4, 9, 10, 8, 4, 11, 12, 9, 10, 8, 3, 6, 2, 5];

const ROLL_ORDER_EXP: [u8; 28] = [
    6, 2, 3, 12, 9, 5, 9, 4, 5, 10, 12, 11, 10, 8, 4, 8, 3, 6, 10, 11, 11, 8, 9, 3, 6, 4, 5, 2,
];

const ROLL_NUMBERS: [u8; 10] = [6, 8, 5, 9, 4, 10, 3, 11, 2, 12];

const TRADE_GAP_BASE: [u32; 9] = [0, 1, 2, 1, 1, 2, 1, 1, 2];

const TRADE_GAP_EXP: [u32; 11] = [0, 1, 1, 1, 1, 1, 1, 1, 3, 1, 2];

pub trait GameEdition {
    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, TileData)> + Clone;
    fn get_trades(&self) -> impl Iterator<Item = TradePort>;
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

        let resource_deck =
            ResourceDeck::new(19, resource_distribution, &mut ROLL_ORDER_BASE.into_iter());

        HexSpiral::new(3, 5).zip(resource_deck)
    }

    fn get_trades(&self) -> impl Iterator<Item = TradePort> {
        TradePortDeck::new(
            3,
            5,
            TradeDistribution::new([
                (TradeType::Resource(ResourceType::Wood), 1),
                (TradeType::Resource(ResourceType::Brick), 1),
                (TradeType::Resource(ResourceType::Wheat), 1),
                (TradeType::Resource(ResourceType::Sheep), 1),
                (TradeType::Resource(ResourceType::Ore), 1),
                (TradeType::Any, 4),
            ]),
            &mut TRADE_GAP_BASE.into_iter(),
        )
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

        let resource_deck =
            ResourceDeck::new(30, resource_distribution, &mut ROLL_ORDER_EXP.into_iter());

        HexSpiral::new(3, 6).zip(resource_deck)
    }

    fn get_trades(&self) -> impl Iterator<Item = TradePort> {
        TradePortDeck::new(
            3,
            6,
            TradeDistribution::new([
                (TradeType::Resource(ResourceType::Wood), 1),
                (TradeType::Resource(ResourceType::Brick), 1),
                (TradeType::Resource(ResourceType::Wheat), 1),
                (TradeType::Resource(ResourceType::Sheep), 2),
                (TradeType::Resource(ResourceType::Ore), 1),
                (TradeType::Any, 5),
            ]),
            &mut TRADE_GAP_EXP.into_iter(),
        )
    }
}

pub struct CustomEdition {
    shortest: u32,
    longest: u32,
    rsrc_distr: ResourceDistribution,
    roll_numbers: Vec<u8>,
    trade_distr: TradeDistribution,
    trade_gaps: Vec<u32>,
}

impl CustomEdition {
    pub fn of_size(shortest: u32, longest: u32) -> CustomEditionBuilder {
        CustomEditionBuilder::of_size(shortest, longest)
    }
}

impl GameEdition for CustomEdition {
    fn get_tiles(&self) -> impl Iterator<Item = (HexPosition, TileData)> + Clone {
        let size: usize = ((self.longest - self.shortest) * 2 + 1) as usize;
        let resource_deck = ResourceDeck::new(
            size,
            self.rsrc_distr.clone(),
            &mut self.roll_numbers.clone().into_iter(),
        );

        HexSpiral::new(self.shortest, self.longest).zip(resource_deck)
    }

    fn get_trades(&self) -> impl Iterator<Item = TradePort> {
        TradePortDeck::new(
            self.shortest,
            self.longest,
            self.trade_distr.clone(),
            &mut self.trade_gaps.clone().into_iter(),
        )
    }
}

pub struct CustomEditionBuilder {
    shortest: u32,
    longest: u32,
    rsrc_distr: ResourceDistribution,
    roll_numbers: Vec<u8>,
    trade_distr: TradeDistribution,
    trade_gaps: Vec<u32>,
}

impl CustomEditionBuilder {
    pub fn build(self) -> CustomEdition {
        CustomEdition {
            shortest: self.shortest,
            longest: self.longest,
            rsrc_distr: self.rsrc_distr,
            roll_numbers: self.roll_numbers,
            trade_distr: self.trade_distr,
            trade_gaps: self.trade_gaps,
        }
    }

    pub fn of_size(shortest: u32, longest: u32) -> CustomEditionBuilder {
        CustomEditionBuilder {
            shortest,
            longest,
            rsrc_distr: Self::default_resource_distribution(shortest, longest),
            roll_numbers: Self::default_roll_numbers(shortest, longest),
            trade_distr: Self::default_trade_distribution(shortest, longest),
            trade_gaps: Self::default_trade_gaps(shortest, longest),
        }
    }

    pub fn with_resource_distribution(
        mut self,
        distr: ResourceDistribution,
    ) -> CustomEditionBuilder {
        self.rsrc_distr = distr;
        self
    }

    pub fn with_roll_numbers(mut self, roll_numbers: Vec<u8>) -> CustomEditionBuilder {
        self.roll_numbers = roll_numbers;
        self
    }

    pub fn with_trade_distribution(mut self, distr: TradeDistribution) -> CustomEditionBuilder {
        self.trade_distr = distr;
        self
    }

    pub fn with_trade_gaps(mut self, gaps: Vec<u32>) -> CustomEditionBuilder {
        self.trade_gaps = gaps;
        self
    }

    fn default_resource_distribution(shortest: u32, longest: u32) -> ResourceDistribution {
        let size: f64 = ((longest - 1) * longest - (shortest - 1) * shortest + longest) as f64;
        ResourceDistribution::new([
            (ResourceType::Wood, (size / 5.).round() as u32),
            (ResourceType::Brick, (size / 6.).round() as u32),
            (ResourceType::Wheat, (size / 5.).round() as u32),
            (ResourceType::Sheep, (size / 5.).round() as u32),
            (ResourceType::Ore, (size / 6.).round() as u32),
        ])
    }

    fn default_roll_numbers(shortest: u32, longest: u32) -> Vec<u8> {
        let size: usize = ((longest - 1) * longest - (shortest - 1) * shortest + longest) as usize;
        let mut roll_numbers: Vec<u8> =
            iter::repeat_n(ROLL_NUMBERS, (size / ROLL_NUMBERS.len()) + 1)
                .flatten()
                .collect();
        roll_numbers.truncate(size);
        roll_numbers.shuffle(&mut rand::rng());

        roll_numbers
    }

    fn default_trade_distribution(shortest: u32, longest: u32) -> TradeDistribution {
        // old unreduced formula.
        // essentially calculates the perimeter of the hex-shaped grid, then uses that to calculate the number of corners on the perimeter.
        // let total_corner_num = ((longest - shortest + 1) * 4 + shortest * 2 - 6) * 2 + 6;

        let total_corner_num = (longest - shortest) * 8 + shortest * 4 + 2;
        let total_trades = (total_corner_num/3) as f64;

        TradeDistribution::new([
            (TradeType::Resource(ResourceType::Wood), (total_trades/10.).ceil() as u32),
            (TradeType::Resource(ResourceType::Brick), (total_trades/10.).round() as u32),
            (TradeType::Resource(ResourceType::Wheat), (total_trades/10.).round() as u32),
            (TradeType::Resource(ResourceType::Sheep), (total_trades/10.).ceil() as u32),
            (TradeType::Resource(ResourceType::Ore), (total_trades/10.).round() as u32),
            (TradeType::Any, (total_trades - ((total_trades/10.).ceil() * 2. + (total_trades/10.).round() * 3.)) as u32),
        ])


    }

    // default behavior is to have all trades spaced by 1.
    fn default_trade_gaps(shortest: u32, longest: u32) -> Vec<u32> {
        let total_corner_num = (longest - shortest) * 8 + shortest * 4 + 2;
        let mut gaps = Vec::<u32>::with_capacity((total_corner_num/3) as usize);
        gaps.push(0);
        gaps.extend(iter::repeat_n(1, (total_corner_num - 1) as usize));
        gaps
    }
}
