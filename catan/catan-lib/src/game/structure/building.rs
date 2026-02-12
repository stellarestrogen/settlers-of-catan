use crate::game::{player::OwnershipToken, structure::{Structure, StructureType}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    Settlement,
    City,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Building {
    r#type: BuildingType,
    owner: OwnershipToken,
}

impl Building {
    pub fn new(r#type: BuildingType, owner: OwnershipToken) -> Self {
        Self { r#type, owner }
    }

    pub fn owner(&self) -> OwnershipToken {
        self.owner
    }
}

impl Into<Structure> for Building {
    fn into(self) -> Structure {
        match self.r#type {
            BuildingType::Settlement => Structure::new(StructureType::Settlement, self.owner),
            BuildingType::City => Structure::new(StructureType::City, self.owner),
        }
    }
}

impl Into<StructureType> for Building {
    fn into(self) -> StructureType {
        match self.r#type {
            BuildingType::Settlement => StructureType::Settlement,
            BuildingType::City => StructureType::City,
        }
    }
}