use crate::{
    game::player::OwnershipToken,
    object::structure::{Structure, StructureType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    Road,
    Boat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transport {
    r#type: TransportType,
    owner: OwnershipToken,
}

impl Transport {
    pub fn new(r#type: TransportType, owner: OwnershipToken) -> Self {
        Self { r#type, owner }
    }

    pub fn owner(&self) -> OwnershipToken {
        self.owner
    }
}

impl Into<Structure> for Transport {
    fn into(self) -> Structure {
        match self.r#type {
            TransportType::Road => Structure::new(StructureType::Road, self.owner),
            TransportType::Boat => Structure::new(StructureType::Boat, self.owner),
        }
    }
}

impl Into<StructureType> for Transport {
    fn into(self) -> StructureType {
        match self.r#type {
            TransportType::Road => StructureType::Road,
            TransportType::Boat => StructureType::Boat,
        }
    }
}
