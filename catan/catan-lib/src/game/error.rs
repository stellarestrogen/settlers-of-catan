use std::fmt::Debug;

use thiserror::Error;

use crate::{
    game::player::OwnershipToken,
    object::{card::ResourceCard, structure::StructureType},
};

#[derive(Debug, Clone)]
pub enum BuildError {
    InsufficientResources {
        structure: StructureType,
        insufficient_resources: Vec<ResourceCard>,
    },
    StructureAlreadyExists,
    BuildingIsTooCloseToExisting,
    BuildingCutsOffRoad,
    BuildingHasNoRoad,
    BuildingIsOnWater,
    CityRequiresSettlement,
    CityUpgradeOwnerMismatch,
    NoStructures {
        token: OwnershipToken,
        structure: StructureType,
    },
    TransportMustBeContiguous,
    TransportInterruptsBuilding,
    RoadMustNeighborLand,
    BoatMustNeighborWater,
}

#[derive(Error, Debug, Clone, Copy)]
pub enum GameError {
    #[error("Not enough players!")]
    InsufficientPlayerCount,
}

// impl fmt::Display for BuildError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             BuildError::InsufficientResources {
//                 structure,
//                 insufficient_resources,
//             } => {
//                 write!(
//                     f,
//                     "The {:?} structure requires {:?} resources, but the following are insufficient: {:?}",
//                     structure,
//                     structure.cost(),
//                     insufficient_resources
//                 )
//             }
//             BuildError::StructureAlreadyExists => write!(f, ""),
//             BuildError::BuildingIsTooCloseToExisting => write!(f, ""),
//             BuildError::BuildingCutsOffRoad => write!(f, ""),
//             BuildError::BuildingHasNoRoad => write!(f, ""),
//             BuildError::CityRequiresSettlement => write!(f, ""),
//             BuildError::CityUpgradeTokenMismatch => write!(f, ""),
//             BuildError::NoStructures { token, structure } => write!(
//                 f,
//                 "The player with {:?} token has no {:?} structures left!",
//                 token, structure
//             ),
//             BuildError::RoadsMustBeContiguous => write!(f, "")
//         }
//     }
// }
