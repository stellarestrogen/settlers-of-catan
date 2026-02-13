pub mod building;
pub mod transport;

use crate::{
    game::player::OwnershipToken,
    object::{card::ResourceMap, resource::ResourceType},
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureType {
    Settlement,
    City,
    Road,
    Boat,
}

impl StructureType {
    pub fn cost(&self) -> ResourceMap {
        match self {
            StructureType::Settlement => ResourceMap::new(1, 1, 1, 1, 0),
            StructureType::City => ResourceMap::new(0, 0, 3, 0, 2),
            StructureType::Road => ResourceMap::new(1, 1, 0, 0, 0),
            StructureType::Boat => ResourceMap::new(1, 0, 0, 1, 0),
        }
    }

    pub fn resource_cost(&self, resource: ResourceType) -> u32 {
        self.cost().get(resource).get_count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Structure {
    r#type: StructureType,
    owner: OwnershipToken,
}

impl Structure {
    pub fn new(r#type: StructureType, owner: OwnershipToken) -> Self {
        Self {
            r#type,
            owner,
        }
    }

    pub fn cost(&self) -> ResourceMap {
        self.r#type.cost()
    }

    pub fn resource_cost(&self, resource: ResourceType) -> u32 {
        self.r#type.resource_cost(resource)
    }

    pub fn owner(&self) -> OwnershipToken {
        self.owner
    }

    pub fn r#type(&self) -> StructureType {
        self.r#type
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OwnedStructures {
    settlements: u32,
    cities: u32,
    roads: u32,
    boats: u32,
}

impl OwnedStructures {
    pub fn new(settlements: u32, cities: u32, roads: u32, boats: u32) -> Self {
        Self {
            settlements,
            cities,
            roads,
            boats,
        }
    }

    pub fn add_structure(&mut self, structure: StructureType) {
        match structure {
            StructureType::Settlement => self.settlements += 1,
            StructureType::City => self.cities += 1,
            StructureType::Road => self.roads += 1,
            StructureType::Boat => self.boats += 1,
        };
    }

    pub fn remove_structure(&mut self, structure: StructureType) {
        match structure {
            StructureType::Settlement => self.settlements = self.settlements.saturating_sub(1),
            StructureType::City => self.cities = self.cities.saturating_sub(1),
            StructureType::Boat => self.boats = self.boats.saturating_sub(1),
            StructureType::Road => self.roads = self.roads.saturating_sub(1),
        }
    }

    pub fn get_structure(&self, structure: StructureType) -> u32 {
        match structure {
            StructureType::Settlement => self.settlements,
            StructureType::City => self.cities,
            StructureType::Road => self.roads,
            StructureType::Boat => self.boats,
        }
    }
}