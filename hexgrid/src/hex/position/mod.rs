pub use horizontal_distance::HorizontalDistance;

use crate::position::{HorizontalAxis, Position, VerticalAxis};

pub mod horizontal_distance;
pub mod op_add;
pub mod op_sub;
pub mod op_mul;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HexPosition {
    rights: i32,
    downs: i32,
}

impl HexPosition {
    pub const ORIGIN: HexPosition = HexPosition {
        rights: 0,
        downs: 0,
    };

    pub const RIGHT: HexPosition = HexPosition {
        rights: 1,
        downs: 0,
    };

    pub const DOWN_RIGHT: HexPosition = HexPosition {
        rights: 1,
        downs: 1,
    };

    pub const DOWN_LEFT: HexPosition = HexPosition {
        rights: 0,
        downs: 1,
    };

    pub const LEFT: HexPosition = HexPosition {
        rights: -1,
        downs: 0,
    };

    pub const UP_LEFT: HexPosition = HexPosition {
        rights: 0,
        downs: -1,
    };

    pub const UP_RIGHT: HexPosition = HexPosition {
        rights: 1,
        downs: -1,
    };

}

impl Position<i32> for HexPosition {
    type HorizontalOutput = HorizontalDistance;
    type VerticalOutput = i32;

    fn positive_axes() -> (HorizontalAxis, VerticalAxis) {
        (HorizontalAxis::Right, VerticalAxis::Down)
    }

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