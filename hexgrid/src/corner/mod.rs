use crate::hex::position::HexPosition;

pub mod bounds;
pub mod iterators;
pub mod position;
pub mod table;

pub trait Corner {
    fn neighboring_hex(&self) -> [HexPosition; 3];
    fn neighboring_corners(&self) -> Vec<Box<dyn Corner>>;
}
