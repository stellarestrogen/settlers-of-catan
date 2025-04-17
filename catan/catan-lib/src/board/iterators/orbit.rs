use hexgrid::hex::position::HexPosition;

use crate::objects::TileData;

use super::ring::Ring;

#[derive(Clone)]
pub struct CircularOrbit<T: Iterator<Item = TileData> + Clone> {
    position: HexPosition,
    tiles: T,
    shortest: u32,
    longest: u32,
    ring: Ring,
}

impl<T: Iterator<Item = TileData> + Clone> CircularOrbit<T> {
    pub fn new(tiles: T, shortest: u32, longest: u32) -> Self {
        CircularOrbit {
            position: HexPosition::ORIGIN,
            tiles,
            shortest,
            longest,
            ring: Ring::new(HexPosition::ORIGIN, shortest, longest),
        }
    }
}

impl<T: Iterator <Item = TileData> + Clone> Iterator for CircularOrbit<T> {
    type Item = (HexPosition, TileData);

    fn next(&mut self) -> Option<(HexPosition, TileData)> {
        if let Some(next) = self.ring.next() {
            self.position = next;
            Some((next, self.tiles.next()?))
        } else if self.shortest > 0 && self.longest > 0 {
            self.shortest -= 1;
            self.longest -= 2;
            self.position += HexPosition::RIGHT;
            self.ring = Ring::new(self.position, self.shortest, self.longest);
            Some((self.ring.next()?, self.tiles.next()?))
        } else {
            None
        }
    }
}