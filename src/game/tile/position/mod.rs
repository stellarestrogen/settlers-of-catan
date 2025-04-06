pub use horizontal_distance::HorizontalDistance;

use crate::game::position::Position;

pub mod horizontal_distance;
pub mod op_add;
pub mod op_mul;
pub mod op_sub;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TilePosition {
    rights: i32,
    downs: i32,
}

impl TilePosition {
    pub const ORIGIN: TilePosition = TilePosition {
        rights: 0,
        downs: 0,
    };

    pub const RIGHT: TilePosition = TilePosition {
        rights: 1,
        downs: 0,
    };

    pub const DOWN_RIGHT: TilePosition = TilePosition {
        rights: 1,
        downs: 1,
    };

    pub const DOWN_LEFT: TilePosition = TilePosition {
        rights: 0,
        downs: 1,
    };

    pub const LEFT: TilePosition = TilePosition {
        rights: -1,
        downs: 0,
    };

    pub const UP_LEFT: TilePosition = TilePosition {
        rights: 0,
        downs: -1,
    };

    pub const UP_RIGHT: TilePosition = TilePosition {
        rights: 1,
        downs: -1,
    };

   
}

impl Position<i32> for TilePosition {
    type HorizontalOutput = HorizontalDistance;
    type VerticalOutput = i32;

    fn horizontal_distance(&self, other: Self) -> Self::HorizontalOutput {
        if self.downs % 2 == other.downs % 2 {
            HorizontalDistance::Unshifted(self.rights - other.rights)
        } else if other.downs % 2 == 1 {
            HorizontalDistance::Shifted(self.rights - other.rights + 1)
        } else {
            HorizontalDistance::Shifted(self.rights - other.rights)
        }
    }

    fn vertical_distance(&self, other: Self) -> Self::VerticalOutput {
        self.downs - other.downs
    }
}