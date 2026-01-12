use crate::object::resource::ResourceType;

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

#[derive(Clone, Copy)]
pub struct ResourceCard {
    resource_type: ResourceType,
    count: u32,
}

impl ResourceCard {
    pub fn new(resource_type: ResourceType, count: u32) -> Self {
        ResourceCard {
            resource_type,
            count,
        }
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn get_resource(&self) -> ResourceType {
        self.resource_type
    }

    pub fn add(&mut self, amt: u32) {
        self.count += amt;
    }

    pub fn sub(&mut self, amt: u32) {
        if self.count <= 0 {
            self.count = 0;
        } else {
            self.count -= amt;
        }
    }
}