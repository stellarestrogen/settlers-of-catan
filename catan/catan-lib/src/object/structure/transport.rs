use hexgrid::edge::{
    position::{EdgePosition, Valid},
    table::EdgeTable,
};

use crate::{
    game::player::OwnershipToken,
    object::{
        EdgeData,
        structure::{Structure, StructureType},
    },
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

pub trait TransportStore {
    fn set_transport<T: Valid>(
        &mut self,
        position: EdgePosition<T>,
        transport: Transport,
    ) -> Result<(), ()>;
    fn get_transport<T: Valid>(&self, position: EdgePosition<T>) -> Option<Transport>;
    fn transports(&self) -> impl Iterator<Item = Transport>;
}

impl TransportStore for EdgeTable<EdgeData> {
    fn set_transport<T: Valid>(
        &mut self,
        position: EdgePosition<T>,
        transport: Transport,
    ) -> Result<(), ()> {
        self.set(position, EdgeData::new(transport))
    }
    fn get_transport<T: Valid>(&self, position: EdgePosition<T>) -> Option<Transport> {
        Some(self.get(position)?.get_transport())
    }
    fn transports(&self) -> impl Iterator<Item = Transport> {
        self.data().map(|d| d.get_transport())
    }
}
