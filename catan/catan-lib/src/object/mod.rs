pub mod resource;
pub mod trade;

use hexgrid::hex::position::HexPosition;

use crate::object::{resource::ResourceType, trade::TradeType};

#[derive(Clone, Copy)]
pub enum TileType {
    Resource {
        resource: ResourceType,
        roll_number: u32,
    },
    Desert,
    Water,
}

#[derive(Clone, PartialEq)]
pub enum DevCardType {
    MoveRobber,
    TakeTwoResources,
    Monopoly,
    VictoryPoint,
    BuildRoads,
}

pub struct DevelopmentCard {
    r#type: DevCardType,
    played: bool,
}

impl DevelopmentCard {
    pub fn new(r#type: DevCardType) -> Self {
        Self {
            r#type,
            played: false,
        }
    }

    pub fn get_type(&self) -> DevCardType {
        self.r#type.clone()
    }

    pub fn is_played(&self) -> bool {
        self.played
    }

    pub fn play(&mut self) {
        self.played = true
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Road,
    Boat,
}

impl Into<Structure> for Transport {
    fn into(self) -> Structure {
        match self {
            Self::Road => Structure::Road,
            Self::Boat => Structure::Boat,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Building {
    Settlement,
    City,
}

impl Into<Structure> for Building {
    fn into(self) -> Structure {
        match self {
            Self::Settlement => Structure::Settlement,
            Self::City => Structure::City,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Structure {
    Settlement,
    City,
    Road,
    Boat,
}

#[derive(Clone, Copy)]
pub struct ResourceCard {
    resource_type: ResourceType,
    count: i32,
}

impl ResourceCard {
    pub fn new(resource_type: ResourceType, count: i32) -> Self {
        ResourceCard {
            resource_type,
            count,
        }
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }

    pub fn get_resource(&self) -> ResourceType {
        self.resource_type
    }

    pub fn add(&mut self, amt: i32) {
        self.count += amt;
    }

    pub fn sub(&mut self, amt: i32) {
        if self.count <= 0 {
            self.count = 0;
        } else {
            self.count -= amt;
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
