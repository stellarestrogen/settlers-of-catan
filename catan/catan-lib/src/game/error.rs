use core::fmt;
use std::fmt::Debug;

use crate::{
    game::{player::OwnershipToken, structure::StructureType},
    object::card::ResourceCard,
};

#[derive(Debug, Clone)]
pub enum BuildError {
    InsufficientResources {
        structure: StructureType,
        insufficient_resources: Vec<ResourceCard>,
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
                insufficient_resources,
            } => {
                write!(
                    f,
                    "The {:?} structure requires {:?} resources, but the following are insufficient: {:?}",
                    structure,
                    structure.cost(),
                    insufficient_resources
                )
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
