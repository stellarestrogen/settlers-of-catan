use hexgrid::corner::{
    position::{CornerPosition, Height},
    table::CornerTable,
};

use crate::{
    game::player::OwnershipToken,
    object::{
        CornerData,
        structure::{Structure, StructureType},
    },
};

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

    pub fn r#type(&self) -> BuildingType {
        self.r#type
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

pub trait BuildingStore {
    fn set_building<H: Height>(
        &mut self,
        position: CornerPosition<H>,
        building: Building,
    ) -> Result<(), ()>;
    fn get_building<H: Height>(&self, position: CornerPosition<H>) -> Option<Building>;
    fn buildings(&self) -> impl Iterator<Item = Building>;
}

impl BuildingStore for CornerTable<CornerData> {
    fn set_building<H: Height>(
        &mut self,
        position: CornerPosition<H>,
        building: Building,
    ) -> Result<(), ()> {
        if let Some(data) = self.get_mut(position) {
            data.set_building(building);
            Ok(())
        } else {
            let mut data = CornerData::new();
            data.set_building(building);
            self.set(position, data)
        }
    }

    fn get_building<H: Height>(&self, position: CornerPosition<H>) -> Option<Building> {
        self.get(position)?.get_building()
    }

    fn buildings(&self) -> impl Iterator<Item = Building> {
        self.data().flat_map(|d| d.get_building())
    }
}
