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
    OutOfBounds { r#type: PositionType },
    NoData { r#type: PositionType },
}
