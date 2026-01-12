use std::error::Error;
use std::fmt;

use crate::{
    corner::position::r#type::CornerType, edge::position::r#type::EdgeType,
    hex::position::HexPosition,
};

#[derive(Debug)]
pub enum PositionType {
    Hex { position: HexPosition },
    Corner { position: CornerType },
    Edge { position: EdgeType },
}

#[derive(Debug)]
pub enum TableError {
    OutOfBounds,
    NoData,
}

#[derive(Debug)]
pub struct PositionError {
    error: TableError,
    position: PositionType,
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self.error {
            TableError::OutOfBounds => "is out of bounds!",
            TableError::NoData => "has no data!",
        };

        // i dont like this but i dont know how else to do it
        match self.position {
            PositionType::Hex { position } => {
                write!(f, "{position} {msg}")
            }
            PositionType::Corner { position } => {
                write!(f, "{position} {msg}")
            }
            PositionType::Edge { position } => {
                write!(f, "{position} {msg}")
            }
        }
    }
}

impl Error for PositionError {}