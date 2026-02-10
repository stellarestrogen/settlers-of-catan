use core::fmt;
use std::fmt::Debug;

use crate::{
    game::{player::OwnershipToken, structures::StructureType},
    object::card::ResourceMap,
};

#[derive(Debug, Clone, Copy)]
pub enum BuildError {
    InsufficientResources {
        structure: StructureType,
        resources: ResourceMap,
    },
    StructureAlreadyExists,
    CityRequiresSettlement,
    NoStructures {
        token: OwnershipToken,
        structure: StructureType,
    },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::InsufficientResources {
                structure,
                resources,
            } => {
                // TODO!
                write!(f, "The {:?} structure requires", structure)
            }
            BuildError::StructureAlreadyExists => write!(f, ""),
            BuildError::CityRequiresSettlement => write!(f, ""),
            BuildError::NoStructures { token, structure } => write!(
                f,
                "The player with {:?} token has no {:?} structures left!",
                token, structure
            ),
        }
    }
}
