use hexgrid::hex::position::HexPosition;

use crate::objects::Tile;

#[derive(Clone, Copy)]
enum RingDistance {
    Side,
    Top,
}

const DIRECTIONS: [(HexPosition, RingDistance); 6] = [
    (HexPosition::RIGHT, RingDistance::Top),
    (HexPosition::DOWN_RIGHT, RingDistance::Side),
    (HexPosition::DOWN_LEFT, RingDistance::Side),
    (HexPosition::LEFT, RingDistance::Top),
    (HexPosition::UP_LEFT, RingDistance::Side),
    (HexPosition::UP_RIGHT, RingDistance::Side),
];

#[derive(Clone, Copy)]
pub struct Ring {
    position: HexPosition,
    shortest: u32,
    longest: u32,
    remaining: u32,
    direction_index: u8,
}

impl Ring {
    pub fn new(position: HexPosition, shortest: u32, longest: u32) -> Self {
        Ring {
            position: position - HexPosition::RIGHT,
            shortest,
            longest,
            remaining: shortest,
            direction_index: 0,
        }
    }
}

impl Iterator for Ring {
    type Item = HexPosition;

    fn next(&mut self) -> Option<HexPosition> {
        if self.remaining == 0 {
            self.direction_index += 1;
            if self.direction_index >= 6 {
                return None;
            }
            let (dir, ring_dist) = DIRECTIONS[self.direction_index as usize];
            match ring_dist {
                RingDistance::Top => self.remaining = self.shortest - 1,
                RingDistance::Side => self.remaining = self.longest - self.shortest,
            }

            if dir == HexPosition::UP_RIGHT {
                self.remaining -= 1;
            }
        }

        let (dir, _) = DIRECTIONS[self.direction_index as usize];
        self.position += dir;
        Some(self.position)
    }
}

#[derive(Clone)]
pub struct CircularOrbit<T: Iterator<Item = Tile> + Clone> {
    position: HexPosition,
    tiles: T,
    shortest: u32,
    longest: u32,
    ring: Ring,
}

impl<T: Iterator<Item = Tile> + Clone> CircularOrbit<T> {
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

impl<T: Iterator <Item = Tile> + Clone> Iterator for CircularOrbit<T> {
    type Item = (HexPosition, Tile);

    fn next(&mut self) -> Option<(HexPosition, Tile)> {
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