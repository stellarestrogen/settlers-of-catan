use crate::corner::iterators::ring::CornerRing;

pub mod ring;
pub mod spiral;


#[test]
fn test_ring() {
    let ring = CornerRing::new(1, 2);
    for position in ring {
        println!("positions: {:?}", position);
    }
}