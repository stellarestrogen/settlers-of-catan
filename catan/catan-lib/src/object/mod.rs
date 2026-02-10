pub mod card;
pub mod resource;
pub mod trade;

use hexgrid::{corner::position::Height, edge::position::Valid, hex::position::HexPosition};

use crate::{
    game::structures::StructureType,
    object::{resource::ResourceType, trade::TradeType},
};

#[derive(Clone, Copy)]
pub enum TileType {
    Resource {
        resource: ResourceType,
        roll_number: u32,
    },
    Desert,
    Water,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Road,
    Boat,
}

impl Into<StructureType> for Transport {
    fn into(self) -> StructureType {
        match self {
            Self::Road => StructureType::Road,
            Self::Boat => StructureType::Boat,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Building {
    Settlement,
    City,
}

impl Into<StructureType> for Building {
    fn into(self) -> StructureType {
        match self {
            Self::Settlement => StructureType::Settlement,
            Self::City => StructureType::City,
        }
    }
}

pub struct Robber {
    position: HexPosition,
}

impl Robber {
    pub fn new(position: HexPosition) -> Self {
        Self { position }
    }

    pub fn r#move(&mut self, position: HexPosition) {
        self.position = position
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct TileData {
    r#type: TileType,
}

impl TileData {
    pub fn new(r#type: TileType) -> Self {
        TileData { r#type }
    }

    pub fn get_resource_type(&self) -> TileType {
        self.r#type
    }

    pub fn set_resource_type(&mut self, r#type: TileType) {
        self.r#type = r#type;
    }
}
