use hexgrid::{
    corner::position::{CornerPosition, Height, r#type::CornerType},
    edge::position::{EdgePosition, Valid, r#type::EdgeType},
};

use crate::object::resource::{RESOURCE_NO, ResourceType};

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

    pub fn add_structure<H: Height, T: Valid>(&mut self, structure: Structure<H, T>) {
        match structure {
            Structure::Settlement { position: _ } => self.settlements += 1,
            Structure::City { position: _ } => self.cities += 1,
            Structure::Road { position: _ } => self.roads += 1,
            Structure::Boat { position: _ } => self.boats += 1,
        };
    }

    pub fn remove_structure<H: Height, T: Valid>(
        &mut self,
        structure: Structure<H, T>,
    ) -> Result<(), ()> {
        match structure {
            Structure::Settlement { position: _ } => self.settlements.checked_sub(1).ok_or(())?,
            Structure::City { position: _ } => self.cities.checked_sub(1).ok_or(())?,
            Structure::Boat { position: _ } => self.boats.checked_sub(1).ok_or(())?,
            Structure::Road { position: _ } => self.roads.checked_sub(1).ok_or(())?,
        };

        Ok(())
    }

    pub fn get_structure<H: Height, T: Valid>(&self, structure: Structure<H, T>) -> u32 {
        match structure {
            Structure::Settlement { position: _ } => self.settlements,
            Structure::City { position: _ } => self.cities,
            Structure::Road { position: _ } => self.roads,
            Structure::Boat { position: _ } => self.boats,
        }
    }
}

#[derive(PartialEq)]
pub enum Structure<H: Height, T: Valid> {
    Settlement { position: Option<CornerPosition<H>> },
    City { position: Option<CornerPosition<H>> },
    Road { position: Option<EdgePosition<T>> },
    Boat { position: Option<EdgePosition<T>> },
}

impl<H: Height, T: Valid> Structure<H, T> {
    pub fn settlement(position: CornerPosition<H>) -> Self {
        Structure::Settlement {
            position: Some(position),
        }
    }

    pub fn city(position: CornerPosition<H>) -> Self {
        Structure::City {
            position: Some(position),
        }
    }

    pub fn road(position: EdgePosition<T>) -> Self {
        Structure::Road {
            position: Some(position),
        }
    }

    pub fn boat(position: EdgePosition<T>) -> Self {
        Structure::Boat {
            position: Some(position),
        }
    }

    pub fn has_position(&self) -> bool {
        match self {
            Structure::Settlement { position } => position.is_some(),
            Structure::City { position } => position.is_some(),
            Structure::Road { position } => position.is_some(),
            Structure::Boat { position } => position.is_some()
        }
    }

    pub fn cost(&self) -> [(ResourceType, u32); RESOURCE_NO] {
        match self {
            Structure::Settlement { position: _ } => [
                (ResourceType::Wood, 1),
                (ResourceType::Brick, 1),
                (ResourceType::Wheat, 1),
                (ResourceType::Sheep, 1),
                (ResourceType::Ore, 0),
            ],
            Structure::City { position: _ } => [
                (ResourceType::Wood, 0),
                (ResourceType::Brick, 0),
                (ResourceType::Wheat, 2),
                (ResourceType::Sheep, 0),
                (ResourceType::Ore, 3),
            ],
            Structure::Road { position: _ } => [
                (ResourceType::Wood, 1),
                (ResourceType::Brick, 1),
                (ResourceType::Wheat, 0),
                (ResourceType::Sheep, 0),
                (ResourceType::Ore, 0),
            ],
            Structure::Boat { position: _ } => [
                (ResourceType::Wood, 1),
                (ResourceType::Brick, 0),
                (ResourceType::Wheat, 0),
                (ResourceType::Sheep, 1),
                (ResourceType::Ore, 0),
            ],
        }
    }

    pub fn resource_cost(&self, resource: ResourceType) -> u32 {
        let (_, count) = self
            .cost()
            .into_iter()
            .find(|(r, _)| r == &resource)
            .expect("Invalid ResourceType");
        count
    }
}

impl<H: Height, T: Valid> Clone for Structure<H, T> {
    fn clone(&self) -> Self {
        match *self {
            Structure::Settlement { position } => Self::Settlement { position },
            Structure::City { position } => Self::City { position },
            Structure::Road { position } => Self::Road { position },
            Structure::Boat { position } => Self::Boat { position },
        }
    }
}

impl<H: Height, T: Valid> Copy for Structure<H, T> {}
