use std::ops::Add;

use crate::object::resource::{self, RESOURCE_NO, ResourceType};

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

    pub fn is_played_and(&self, c: impl FnOnce(&Self) -> bool) -> bool {
        self.played && c(self)
    }

    pub fn is_victory_point(&self) -> bool {
        self.r#type == DevCardType::VictoryPoint
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceCard {
    resource_type: ResourceType,
    count: u32,
}

impl ResourceCard {
    pub fn of_type(resource_type: ResourceType) -> Self {
        ResourceCard {
            resource_type,
            count: 0,
        }
    }

    pub fn with_count(self, count: u32) -> Self {
        Self {
            resource_type: self.resource_type,
            count,
        }
    }

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
        self.count = self.count.saturating_add(amt)
    }

    pub fn sub(&mut self, amt: u32) {
        self.count = self.count.saturating_sub(amt)
    }
}

impl FromIterator<ResourceCard> for [ResourceCard; RESOURCE_NO] {
    fn from_iter<T: IntoIterator<Item = ResourceCard>>(iter: T) -> Self {
        let mut c = Vec::<ResourceCard>::new();

        for i in iter {
            c.push(i);
        }

        c.try_into().expect("Something has gone horribly wrong...")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceMap {
    wood: ResourceCard,
    brick: ResourceCard,
    wheat: ResourceCard,
    sheep: ResourceCard,
    ore: ResourceCard,
}

impl ResourceMap {
    pub fn new(wood: u32, brick: u32, wheat: u32, sheep: u32, ore: u32) -> Self {
        Self {
            wood: ResourceCard::of_type(ResourceType::Wood).with_count(wood),
            brick: ResourceCard::of_type(ResourceType::Brick).with_count(brick),
            wheat: ResourceCard::of_type(ResourceType::Wheat).with_count(wheat),
            sheep: ResourceCard::of_type(ResourceType::Sheep).with_count(sheep),
            ore: ResourceCard::of_type(ResourceType::Ore).with_count(ore),
        }
    }

    pub fn empty() -> Self {
        ResourceMap::new(0, 0, 0, 0, 0)
    }

    pub fn set_resource_count(&mut self, resource: ResourceType, count: u32) {
        match resource {
            ResourceType::Wood => self.wood = self.wood.with_count(count),
            ResourceType::Brick => self.brick = self.brick.with_count(count),
            ResourceType::Wheat => self.wheat = self.wheat.with_count(count),
            ResourceType::Sheep => self.sheep = self.sheep.with_count(count),
            ResourceType::Ore => self.ore = self.ore.with_count(count),
        }
    }

    pub fn with_resource(mut self, resource: ResourceType, count: u32) -> Self {
        self.set_resource_count(resource, count);
        self
    }

    pub fn get(&self, resource: ResourceType) -> ResourceCard {
        match resource {
            ResourceType::Wood => self.wood,
            ResourceType::Brick => self.brick,
            ResourceType::Wheat => self.wheat,
            ResourceType::Sheep => self.sheep,
            ResourceType::Ore => self.ore,
        }
    }
}
