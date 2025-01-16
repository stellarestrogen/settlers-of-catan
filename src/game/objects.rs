use super::player::PlayerColor;

#[derive(Clone)]
#[derive(Copy)]

pub enum ResourceType {
    Wood,
    Brick,
    Wheat,
    Sheep,
    Ore,
}

#[derive(Clone)]
#[derive(Copy)]
pub enum TileType {
    Resource(ResourceType),
    Desert,
    Water,
}

#[derive(Copy)]
#[derive(Clone)]
pub enum TradeType {
    Resource(ResourceType),
    Any,
}

#[derive(Clone)]
pub enum DevCardType {
    MoveRobber,
    TakeTwoResources,
    Monopoly,
    VictoryPoint(String),
    BuildRoads,
}

#[derive(Clone)]
#[derive(Copy)]
pub enum Edge {
    Road,
    None,
}

#[derive(Clone)]
#[derive(Copy)]
pub enum Building {
   Settlement,
   City,
   None,
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Corner {
    building: Option<Building>,
    trade_type: Option<TradeType>
}

impl Corner {
    pub fn new() -> Self {
        Corner {
            building: None,
            trade_type: None,
        }
    }

    pub fn set_building(&mut self, building: Option<Building>) {
        self.building = building
    }

    pub fn set_trade(&mut self, trade: Option<TradeType>) {
        self.trade_type = trade
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Tile {
    tile_type: TileType,
    roll_number: i32,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            roll_number: 0
        }
    }

    pub fn get_tile_type(&self) -> TileType {
        self.tile_type
    }

    pub fn get_roll_num(&self) -> i32 {
        self.roll_number
    }

    pub fn get_tile_resource(&self) -> Option<ResourceType> {
        match self.tile_type {
            TileType::Resource(resource) => Some(resource),
            _ => None
        }
    }

    pub fn set_roll_num(&mut self, roll: i32) {
        self.roll_number = roll;
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct ResourceCard {
    resource_type: ResourceType,
    count: i32
}

impl ResourceCard {
    pub fn new(resource_type: ResourceType, count: i32) -> Self {
        ResourceCard {
            resource_type,
            count
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
        if self.count <=0  { 
            self.count = 0;
        } else {
            self.count -= amt;
        }

    }
}