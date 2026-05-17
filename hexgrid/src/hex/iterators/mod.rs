use crate::hex::{iterators::ring::HexRing, position::HexPosition};

pub mod ring;
pub mod spiral;

#[test]
fn ring_test() {
    let ring = HexRing::new(HexPosition::ORIGIN, 3, 5);

    for position in ring {
        println!("position: {:?}", position);
    }
}