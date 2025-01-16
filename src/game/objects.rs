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
pub enum Building {
   Settlement,
   City,
   None,
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