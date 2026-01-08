use hexgrid::{
    corner::position::{CornerPosition, Height, r#type::CornerType},
    edge::position::{EdgePosition, Valid, r#type::EdgeType},
};

use crate::object::Structure;

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

    pub fn build_road<T: Valid>(&mut self, position: EdgePosition<T>) {
        self.roads.push(EdgeType::into(position))
    }

    pub fn build_boat<T: Valid>(&mut self, position: EdgePosition<T>) {
        self.boats.push(EdgeType::into(position))
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

    pub fn remove_structure(&mut self, structure: Structure) -> Result<(), ()> {
        match structure {
            Structure::Settlement => self.settlements.checked_sub(1).ok_or(())?,
            Structure::City => self.cities.checked_sub(1).ok_or(())?,
            Structure::Boat => self.boats.checked_sub(1).ok_or(())?,
            Structure::Road => self.roads.checked_sub(1).ok_or(())?,
        };

        Ok(())
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
