use crate::{corner::position::{CornerPosition, High, Low}, hex::position::HexPosition};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Right,
    DownRight,
    DownLeft,
    Left,
    UpLeft,
    UpRight,
}

impl Iterator for Direction {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        let dir = match *self {
            Direction::Right => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::Left,
            Direction::Left => Direction::UpLeft,
            Direction::UpLeft => Direction::UpRight,
            Direction::UpRight => return None,
        };

        *self = dir;

        Some(dir)
    }
}

#[derive(Clone, Copy)]
pub struct CornerRing {
    positions: (CornerPosition<Low>, CornerPosition<High>),
    shortest: u32,
    longest: u32,
    remaining: u32,
    direction: Direction,
}

impl CornerRing {
    pub fn new(shortest: u32, longest: u32) -> Self {
        CornerRing {
            positions: (
                (HexPosition::ORIGIN + CornerPosition::TOP_LEFT) + CornerPosition::UP_LEFT + CornerPosition::DOWN_LEFT,
                (HexPosition::ORIGIN + CornerPosition::TOP_LEFT) + CornerPosition::UP_LEFT,
            ),
            shortest,
            longest,
            remaining: shortest,
            direction: Direction::Right,
        }
    }

    fn get_remaining(&self) -> u32 {
        match self.direction {
            Direction::Right | Direction::Left => self.shortest,
            Direction::DownRight => self.longest - self.shortest + 1,
            Direction::DownLeft | Direction::UpLeft | Direction::UpRight => {
                self.longest - self.shortest
            }
        }
    }

    fn move_in_direction(&mut self) {
        let low = self.positions.0;
        let high = self.positions.1;

        match self.direction {
            Direction::Right => {
                self.positions = (low.go_right().go_right(), high.go_right().go_right())
            }

            Direction::DownRight => {
                let high = high.go_right().go_down();

                if self.remaining == self.get_remaining() {
                    self.positions = (low.go_right().go_right(), high)
                } else {
                    self.positions = (low.go_down().go_right(), high)
                }
            }

            Direction::DownLeft => {
                self.positions = (low.go_down().go_left(), high.go_left().go_down())
            }

            Direction::Left => {
                let high = high.go_left().go_left();

                if self.remaining == self.get_remaining() {
                    self.positions = (low.go_down().go_left(), high)
                } else {
                    self.positions = (low.go_left().go_left(), high)
                }
            }

            Direction::UpLeft => self.positions = (low.go_left().go_up(), high.go_up().go_left()),

            Direction::UpRight => {
                let high = high.go_up().go_right();

                if self.remaining == self.get_remaining() {
                    self.positions = (low.go_left().go_up(), high)
                } else {
                    self.positions = (low.go_right().go_up(), high)
                }
            }
        }
    }
}

impl Iterator for CornerRing {
    type Item = (CornerPosition<Low>, CornerPosition<High>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            self.direction.next()?;
            self.remaining = self.get_remaining();
        }

        self.move_in_direction();
        self.remaining -= 1;
        Some(self.positions)
    }
}
