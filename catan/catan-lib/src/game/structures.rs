use hexgrid::{
    corner::position::{CornerPosition, Height, r#type::CornerType},
    edge::position::{EdgePosition, Valid, r#type::EdgeType},
};

use crate::object::{
    card::{ResourceCard, ResourceMap},
    resource::{RESOURCE_NO, ResourceType},
};

#[derive(Clone)]
pub struct PlayedStructures {
    settlements: Vec<CornerType>,
    cities: Vec<CornerType>,
    roads: Vec<EdgeType>,
    boats: Vec<EdgeType>,
}

impl PlayedStructures {
    pub fn new() -> Self {
        PlayedStructures {
            settlements: Vec::new(),
            cities: Vec::new(),
            roads: Vec::new(),
            boats: Vec::new(),
        }
    }

    pub fn build_settlement<H: Height>(&mut self, position: CornerPosition<H>) {
        self.settlements.push(CornerType::from(position))
    }

    pub fn has_settlement<H: Height>(&self, position: CornerPosition<H>) -> bool {
        if let Some(_) = self
            .settlements
            .iter()
            .find(|p| **p == CornerType::from(position))
        {
            true
        } else {
            false
        }
    }

    /// Tries to find a settlement at the position. If one does not exist, returns Err(()). Otherwise, replace it with a city.
    pub fn build_city<H: Height>(&mut self, position: CornerPosition<H>) -> Result<(), ()> {
        let settlement = self
            .settlements
            .iter()
            .position(|p| &CornerType::from(position) == p)
            .ok_or(())?;
        self.settlements.swap_remove(settlement);
        self.cities.push(CornerType::from(position));
        Ok(())
    }

    pub fn has_city<H: Height>(&self, position: CornerPosition<H>) -> bool {
        if let Some(_) = self
            .cities
            .iter()
            .find(|p| **p == CornerType::from(position))
        {
            true
        } else {
            false
        }
    }

    pub fn build_road<T: Valid>(&mut self, position: EdgePosition<T>) {
        self.roads.push(EdgeType::from(position))
    }

    pub fn has_road<T: Valid>(&self, position: EdgePosition<T>) -> bool {
        if let Some(_) = self.roads.iter().find(|p| **p == EdgeType::from(position)) {
            true
        } else {
            false
        }
    }

    pub fn build_boat<T: Valid>(&mut self, position: EdgePosition<T>) {
        self.boats.push(EdgeType::from(position))
    }

    pub fn has_boat<T: Valid>(&self, position: EdgePosition<T>) -> bool {
        if let Some(_) = self.boats.iter().find(|p| **p == EdgeType::from(position)) {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy)]
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

    pub fn add_structure(&mut self, structure: Structure) {
        match structure {
            Structure::Settlement => self.settlements += 1,
            Structure::City => self.cities += 1,
            Structure::Road => self.roads += 1,
            Structure::Boat => self.boats += 1,
        };
    }

    pub fn remove_structure(&mut self, structure: Structure) {
        match structure {
            Structure::Settlement => self.settlements = self.settlements.saturating_sub(1),
            Structure::City => self.cities = self.cities.saturating_sub(1),
            Structure::Boat => self.boats = self.boats.saturating_sub(1),
            Structure::Road => self.roads = self.roads.saturating_sub(1),
        }

    }

    pub fn get_structure(&self, structure: Structure) -> u32 {
        match structure {
            Structure::Settlement => self.settlements,
            Structure::City => self.cities,
            Structure::Road => self.roads,
            Structure::Boat => self.boats,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Structure {
    Settlement,
    City,
    Road,
    Boat,
}

impl Structure {
    pub fn cost(&self) -> ResourceMap {
        match self {
            Structure::Settlement => ResourceMap::new(1, 1, 1, 1, 0),
            Structure::City => ResourceMap::new(0, 0, 3, 0, 2),
            Structure::Road => ResourceMap::new(1, 1, 0, 0, 0),
            Structure::Boat => ResourceMap::new(1, 0, 0, 1, 0),
        }
    }

    pub fn resource_cost(&self, resource: ResourceType) -> u32 {
        self.cost().get(resource).get_count()
    }
}
