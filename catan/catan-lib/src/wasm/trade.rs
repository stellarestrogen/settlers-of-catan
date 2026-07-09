use hexgrid::corner::position::CornerPosition;
use serde::Serialize;
use tsify::Tsify;

use crate::{
    object::trade::{TradePort, TradeType},
    wasm::position::WasmCornerPosition,
};

#[derive(Debug, Clone, Copy, Tsify, Serialize)]
pub struct WasmTradePort {
    pub positions: [WasmCornerPosition; 2],
    pub trade: TradeType,
}

impl WasmTradePort {
    pub fn from_trade_port(port: TradePort) -> Self {
        let (p1, p2) = port.get_positions();
        let positions: [WasmCornerPosition; 2] = [
            Into::<CornerPosition>::into(p1).into(),
            Into::<CornerPosition>::into(p2).into(),
        ];

        let trade = port.get_type();

        Self { positions, trade }
    }
}