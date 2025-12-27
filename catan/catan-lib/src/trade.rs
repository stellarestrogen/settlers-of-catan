

use crate::{distribution::Distribution, objects::{ResourceType, TradeType}};

pub const TRADE_NO: usize = 6;

static TRADES: [TradeType; TRADE_NO] = [
    TradeType::Resource(ResourceType::Wood),
    TradeType::Resource(ResourceType::Brick),
    TradeType::Resource(ResourceType::Wheat),
    TradeType::Resource(ResourceType::Sheep),
    TradeType::Resource(ResourceType::Ore),
    TradeType::Any,
];

pub type TradeDistribution = Distribution<TradeType, TRADE_NO>;

pub struct TradePortDeck {}
