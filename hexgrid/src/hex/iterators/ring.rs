use crate::hex::position::HexPosition;

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
pub struct HexRing {
    position: HexPosition,
    shortest: u32,
    longest: u32,
    remaining: u32,
    direction_index: u8,
}

impl HexRing {
    pub fn new(position: HexPosition, shortest: u32, longest: u32) -> Self {
        HexRing {
            position: position - HexPosition::RIGHT,
            shortest,
            longest,
            remaining: shortest,
            direction_index: 0,
        }
    }
}

impl Iterator for HexRing {
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
