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
            Some(self.position)
        } else if self.shortest > 0 && self.longest > 0 {
            self.shortest -= 1;
            self.longest -= 2;
            self.position += HexPosition::RIGHT;
            self.ring = HexRing::new(self.position, self.shortest, self.longest);
            self.ring.next()
        } else {
            None
        }
    }
}