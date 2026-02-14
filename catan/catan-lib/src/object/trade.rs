use std::iter;

use hexgrid::corner::{
    iterators::ring::CornerRing,
    position::{CornerPosition, Height, High, Low},
    table::CornerTable,
};
use rand::seq::SliceRandom;

use crate::{
    distribution::Distribution,
    game::edition::GameEdition,
    object::{CornerData, resource::ResourceType},
};

const TRADE_NO: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TradeType {
    Resource(ResourceType),
    Any,
}

#[derive(Clone, Copy)]
pub struct TradePort {
    positions: (CornerPosition<Low>, CornerPosition<High>),
    r#type: TradeType,
}

impl TradePort {
    pub fn new(
        r#type: TradeType,
        low_position: CornerPosition<Low>,
        high_position: CornerPosition<High>,
    ) -> Self {
        Self {
            positions: (low_position, high_position),
            r#type,
        }
    }

    pub fn get_positions(&self) -> (CornerPosition<Low>, CornerPosition<High>) {
        self.positions
    }

    pub fn get_type(&self) -> TradeType {
        self.r#type
    }
}

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
                .zip(
                    Self::trade_positions(distribution.size(), shortest, longest, trade_gaps)
                        .into_iter(),
                )
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

pub trait TradeStore {
    fn with_trades(self, edition: &impl GameEdition) -> Self;
    fn set_trades(&mut self, trade_port: TradePort) -> Result<(), ()>;
    fn set_trade<H: Height>(
        &mut self,
        position: CornerPosition<H>,
        trade: TradeType,
    ) -> Result<(), ()>;
    fn get_trade<H: Height>(&self, position: CornerPosition<H>) -> Option<TradeType>;
}

impl TradeStore for CornerTable<CornerData> {
    fn with_trades(mut self, edition: &impl GameEdition) -> Self {
        let trades = edition.get_trades();
        for trade in trades.into_iter() {
            self.set_trades(trade)
                .expect("CornerPosition is out of bounds!");
        }

        self
    }

    fn set_trades(&mut self, trade_port: TradePort) -> Result<(), ()> {
        let (p1, p2) = trade_port.get_positions();
        let trade = trade_port.get_type();

        self.set_trade(p1, trade)?;
        self.set_trade(p2, trade)?;
        Ok(())
    }

    fn set_trade<H: Height>(
        &mut self,
        position: CornerPosition<H>,
        trade: TradeType,
    ) -> Result<(), ()> {
        if let Some(data) = self.get_mut(position) {
            data.set_trade(trade);
            Ok(())
        } else {
            let mut data = CornerData::new();
            data.set_trade(trade);
            self.set(position, data)
        }
    }

    fn get_trade<H: Height>(&self, position: CornerPosition<H>) -> Option<TradeType> {
        self.get(position)?.get_trade()
    }
}
