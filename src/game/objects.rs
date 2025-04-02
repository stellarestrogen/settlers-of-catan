use super::player::PlayerColor;

#[derive(Clone, Copy, PartialEq)]
pub enum ResourceType {
    Wood,
    Brick,
    Wheat,
    Sheep,
    Ore,
}

impl ResourceType {
    pub fn get_resource_distribution(&self, size: impl Into<f64>) -> u32 {
        match self {
            Self::Wood => (size.into()/5.0).round() as u32,
            Self::Brick => (size.into()/6.0).round() as u32,
            Self::Wheat => (size.into()/5.0).round() as u32,
            Self::Sheep => (size.into()/5.0).round() as u32,
            Self::Ore => (size.into()/6.0).round() as u32
        }
    }
}

#[derive(Clone)]
pub struct ResourceDistribution {
    distribution: [(ResourceType, u32); 5]
}

impl ResourceDistribution {
    pub fn new(distribution: [(ResourceType, u32); 5]) -> Self {
        ResourceDistribution {
            distribution
        }
    }

    pub fn for_resource(&self, resource: ResourceType) -> u32 {
        let default = (resource, 0);
        let (_, d) = self.distribution.iter().find(|(rsrc, _)| rsrc == &resource).unwrap_or(&default);
        *d
    }
}

#[derive(Clone, Copy)]
pub enum TileType {
    Resource { resource: ResourceType, roll_number: u32 },
    Desert,
    Water,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum Transport {
    Road,
    Boat
}

#[derive(Clone, Copy)]
pub enum Building {
   Settlement,
   City,
   None,
}

#[derive(Clone, Copy)]
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