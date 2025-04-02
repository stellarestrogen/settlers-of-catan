pub mod bounds;
pub mod holder;
pub mod position;

use super::objects::{Building, TradeType};


#[derive(Clone, Copy)]
pub struct Corner {
    building: Option<Building>,
    trade_type: Option<TradeType>,
}

impl Corner {
    pub fn new() -> Self {
        Corner {
            building: None,
            trade_type: None
        }
    }

    pub fn set_building(&mut self, building: Building) {
        self.building = Some(building)
    }

    pub fn unset_building (&mut self) {
        self.building = None
    }

    pub fn set_trade(&mut self, trade_type: TradeType) {
        self.trade_type = Some(trade_type)
    }

    pub fn unset_trade(&mut self) {
        self. trade_type = None
    }
}