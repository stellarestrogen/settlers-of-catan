use std::iter;

use hexgrid::corner::{
    iterators::ring::CornerRing,
    position::{CornerPosition, High, Low},
};
use rand::seq::SliceRandom;

use crate::{
    distribution::Distribution,
    objects::{ResourceType, TradePort, TradeType},
};

const TRADE_NO: usize = 6;

static TRADES: [TradeType; TRADE_NO] = [
    TradeType::Resource(ResourceType::Wood),
    TradeType::Resource(ResourceType::Brick),
    TradeType::Resource(ResourceType::Wheat),
    TradeType::Resource(ResourceType::Sheep),
    TradeType::Resource(ResourceType::Ore),
    TradeType::Any,
];

pub type TradeDistribution = Distribution<TradeType, TRADE_NO>;

pub struct TradePortDeck {
    trades: Vec<TradePort>,
}

impl TradePortDeck {
    pub fn new(
        hex_shortest: u32,
        hex_longest: u32,
        distribution: TradeDistribution,
        trade_gaps: &mut impl Iterator<Item = u32>,
    ) -> Self {
        let longest = hex_longest * 2 + 1;
        let shortest = hex_shortest * 2 + 1;

        Self {
            trades: Self::create_trades(distribution.clone())
                .into_iter()
                .zip(Self::trade_positions(distribution.size(), shortest, longest, trade_gaps).into_iter())
                .map(|(t, (p1, p2))| TradePort::new(t, p1, p2))
                .collect(),
        }
    }

    fn create_trades(distribution: TradeDistribution) -> Vec<TradeType> {
        let mut trades = Vec::<TradeType>::with_capacity(distribution.size());
        for trade in TRADES {
            trades.extend(iter::repeat_n(trade, distribution.for_obj(trade) as usize));
        }
        trades.truncate(distribution.size());

        trades.shuffle(&mut rand::rng());

        trades
    }

    fn trade_positions(
        size: usize,
        shortest: u32,
        longest: u32,
        trade_gaps: &mut impl Iterator<Item = u32>,
    ) -> Vec<(CornerPosition<Low>, CornerPosition<High>)> {
        let mut trades = Vec::<(CornerPosition<Low>, CornerPosition<High>)>::with_capacity(size);
        let mut offset = false;
        let mut ring = CornerRing::new(shortest, longest).into_iter();

        for gap in trade_gaps {
            if gap % 2 == 1 {
                offset = !offset
            }

            if offset == true {
                let high = ring
                    .nth((gap / 2) as usize)
                    .expect("CornerRing is empty!")
                    .1;
                let low = ring.next().expect("CornerRing is empty!").0;
                trades.push((low, high));
            } else {
                trades.push(ring.nth((gap / 2) as usize).expect("CornerRing is empty!"));
            }
        }

        trades
    }
}

impl Iterator for TradePortDeck {
    type Item = TradePort;

    fn next(&mut self) -> Option<Self::Item> {
        self.trades.pop()
    }
}