use crate::hex::iterators::spiral::HexSpiral;

pub mod ring;
pub mod spiral;

#[test]
fn test_spiral() {
    let spiral = HexSpiral::new(1, 2);
    for position in spiral {
        println!("position: {:?}", position);
    }
}