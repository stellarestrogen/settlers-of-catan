pub mod card;
pub mod resource;
pub mod structure;
pub mod trade;

use hexgrid::hex::{position::HexPosition, table::HexTable};

use crate::object::{
    resource::ResourceType,
    structure::{building::Building, transport::Transport},
    trade::TradeType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Resource {
        resource: ResourceType,
        roll_number: u32,
    },
    Desert,
    Water,
}

impl TileType {
    pub fn get_roll_number(&self) -> Option<u32> {
        match self {
            TileType::Resource {
                resource: _,
                roll_number,
            } => Some(*roll_number),
            _ => None,
        }
    }

    pub fn get_resource_type(&self) -> Option<ResourceType> {
        match self {
            TileType::Resource {
                resource,
                roll_number: _,
            } => Some(*resource),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Robber {
    position: HexPosition,
}

impl Robber {
    pub fn place(tiles: &HexTable<TileData>) -> Self {
        if let Some(desert_tile) = tiles
            .positions()
            .find(|p| tiles.get(*p).unwrap().get_tile_type() == TileType::Desert)
        {
            Self {
                position: desert_tile,
            }
        } else {
            Self {
                position: HexPosition::ORIGIN,
            }
        }
    }

    pub fn r#move(&mut self, position: HexPosition) {
        self.position = position
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CornerData {
    building: Option<Building>,
    trade_type: Option<TradeType>,
}

impl CornerData {
    pub fn new() -> Self {
        CornerData {
            building: None,
            trade_type: None,
        }
    }

    pub fn set_building(&mut self, building: Building) {
        self.building = Some(building)
    }

    pub fn unset_building(&mut self) {
        self.building = None
    }

    pub fn set_trade(&mut self, trade_type: TradeType) {
        self.trade_type = Some(trade_type)
    }

    pub fn unset_trade(&mut self) {
        self.trade_type = None
    }

    pub fn get_building(&self) -> Option<Building> {
        self.building
    }

    pub fn get_trade(&self) -> Option<TradeType> {
        self.trade_type
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeData {
    transport: Transport,
}

impl EdgeData {
    pub fn new(transport: Transport) -> Self {
        EdgeData { transport }
    }

    pub fn get_transport(&self) -> Transport {
        self.transport
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileData {
    r#type: TileType,
}

impl TileData {
    pub fn new(r#type: TileType) -> Self {
        TileData { r#type }
    }

    pub fn get_tile_type(&self) -> TileType {
        self.r#type
    }

    pub fn set_tile_type(&mut self, r#type: TileType) {
        self.r#type = r#type;
    }

    pub fn get_roll_number(&self) -> Option<u32> {
        match self.r#type {
            TileType::Resource {
                resource: _,
                roll_number,
            } => Some(roll_number),
            _ => None,
        }
    }
}
