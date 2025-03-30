use super::tile::TilePosition;

enum Direction {
    Right,
    Down,
    Left,
    Up
}

pub struct CircularOrbit {
    position: TilePosition,
    shortest: u32,
    longest: u32,
    direction: Direction
}

impl CircularOrbit {
    pub fn new(shortest: u32, longest: u32) -> Self {
        CircularOrbit { 
            position: TilePosition::new(0, 0),
            shortest,
            longest,
            direction: Direction::Right
        }
    }

    fn calc_end_position(&self) -> TilePosition {
        TilePosition::new((self.longest - 1)/2, self.longest - self.shortest)
    }
}

impl Iterator for CircularOrbit {
    type Item = TilePosition;

    fn next(&mut self) -> Option<TilePosition> {
        if self.position == self.calc_end_position() {
            None
        } else {
            
        }
    }
}