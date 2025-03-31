use super::tile::{Tile, TilePosition};

#[derive(Clone, Copy)]
enum RingDistance {
    Side,
    Top
}

const DIRECTIONS: [(TilePosition, RingDistance); 6] = [
    (TilePosition::RIGHT, RingDistance::Top),
    (TilePosition::DOWN_RIGHT, RingDistance::Side),
    (TilePosition::DOWN_LEFT, RingDistance::Side),
    (TilePosition::LEFT, RingDistance::Top),
    (TilePosition::UP_LEFT, RingDistance::Side),
    (TilePosition::UP_RIGHT, RingDistance::Side)
];

#[derive(Clone, Copy)]
pub struct Ring {
    position: TilePosition,
    shortest: u32,
    longest: u32,
    remaining: u32,
    direction_index: u8
}

impl Ring {
    pub fn new(position: TilePosition, shortest: u32, longest: u32) -> Self {
        Ring {
            position: position - TilePosition::RIGHT,
            shortest,
            longest,
            remaining: shortest,
            direction_index: 0
        }
    }
}

impl Iterator for Ring {
    type Item = TilePosition;

    fn next(&mut self) -> Option<TilePosition> {
        if self.remaining == 0 {
            self.direction_index += 1;
            if self.direction_index >= 6 {
                return None
            }
            let (dir, ring_dist) = DIRECTIONS[self.direction_index as usize];
            match ring_dist {
                RingDistance::Top => self.remaining = self.shortest - 1,
                RingDistance::Side => self.remaining = self.longest - self.shortest
            }

            if dir == TilePosition::UP_RIGHT {
                self.remaining -= 1;
            }
            
        }

        let (dir, _) = DIRECTIONS[self.direction_index as usize];
        self.position += dir;
        Some(self.position)
    }
}

#[derive(Clone, Copy)]
pub struct CircularOrbit {
    position: TilePosition,
    shortest: u32,
    longest: u32,
    ring: Ring
}

impl CircularOrbit {
    pub fn new(position: TilePosition, shortest: u32, longest: u32) -> Self {
        CircularOrbit {
            position,
            shortest,
            longest,
            ring: Ring::new(position, shortest, longest)
        }
    }
}

impl Iterator for CircularOrbit {
    type Item = TilePosition;

    fn next(&mut self) -> Option<TilePosition> {
        if let Some(next) = self.ring.next() {
            self.position = next;
            Some(next)
        } else if self.shortest > 0 && self.longest > 0 {
            self.shortest -= 1;
            self.longest -= 2;
            self.position += TilePosition::RIGHT;
            self.ring = Ring::new(self.position, self.shortest, self.longest);
            self.ring.next()
        } else {
            None
        }
    }
}