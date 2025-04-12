#[derive(Clone, Copy, PartialEq)]
pub enum ResourceType {
    Wood,
    Brick,
    Wheat,
    Sheep,
    Ore,
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

#[derive(Clone, Copy)]
pub struct Edge {
    transport: Option<Transport>
}

impl Edge {
    pub fn new() -> Self {
        Edge {
            transport: None
        }
    }

    pub fn set_transport(&mut self, transport: Transport) {
        self.transport = Some(transport)
    }

    pub fn unset_transport(&mut self) {
        self.transport = None
    }
}

#[derive(Clone)]
pub struct Tile {
    r#type: TileType,
}

impl Tile {
    pub fn new(r#type: TileType) -> Self {
        Tile { r#type }
    }

    pub fn get_resource_type(&self) -> TileType {
        self.r#type
    }

    pub fn set_resource_type(&mut self, r#type: TileType) {
        self.r#type = r#type;
    }
}
