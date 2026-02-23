use crate::hex::position::HexPosition;

pub mod bounds;
pub mod iterators;
pub mod position;
pub mod table;

pub trait Edge {
    fn neighboring_hex(&self) -> [HexPosition; 2];
}
