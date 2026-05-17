use crate::hex::position::HexPosition;

use super::ring::HexRing;

#[derive(Clone)]
pub struct HexSpiral {
    position: HexPosition,
    shortest: u32,
    longest: u32,
    ring: HexRing,
}

impl HexSpiral {
    pub fn new(shortest: u32, longest: u32) -> Self {
        HexSpiral {
            position: HexPosition::ORIGIN,
            shortest,
            longest,
            ring: HexRing::new(HexPosition::ORIGIN, shortest, longest),
        }
    }
}

impl Iterator for HexSpiral {
    type Item = HexPosition;

    fn next(&mut self) -> Option<HexPosition> {
        if let Some(next) = self.ring.next() {
            self.position = next;
            return Some(self.position);
        } 
        if self.shortest > 1 {
            self.shortest = self.shortest.saturating_sub(1);
            self.position += HexPosition::RIGHT;
        } else {
            self.position += HexPosition::DOWN_RIGHT;
        }
        self.longest = self.longest.checked_sub(2)?;
        self.ring = HexRing::new(self.position, self.shortest, self.longest);
        self.ring.next()


    }
}
