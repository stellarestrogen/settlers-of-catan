use core::fmt;

pub use horizontal_distance::HorizontalDistance;

pub mod horizontal_distance;
pub mod op_add;
pub mod op_mul;
pub mod op_sub;

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

    pub fn horizontal_distance(&self, other: Self) -> HorizontalDistance {
        if self.downs % 2 == other.downs % 2 {
            HorizontalDistance::Unshifted(self.rights - other.rights)
        } else if other.downs % 2 == 1 {
            HorizontalDistance::Shifted(self.rights - other.rights + 1)
        } else {
            HorizontalDistance::Shifted(self.rights - other.rights)
        }
    }

    pub fn vertical_distance(&self, other: Self) -> i32 {
        self.downs - other.downs
    }

    pub fn is_right(&self, other: Self) -> bool {
        self.horizontal_distance(other).ceil() > 0
    }

    pub fn is_right_or_equal(&self, other: Self) -> bool {
        self.horizontal_distance(other).ceil() >= 0
    }

    pub fn is_left(&self, other: Self) -> bool {
        self.horizontal_distance(other).ceil() < 0
    }

    pub fn is_left_or_equal(&self, other: Self) -> bool {
        self.horizontal_distance(other).ceil() <= 0
    }

    pub fn is_below(&self, other: Self) -> bool {
        self.vertical_distance(other) > 0
    }

    pub fn is_below_or_equal(&self, other: Self) -> bool {
        self.vertical_distance(other) >= 0
    }

    pub fn is_above(&self, other: Self) -> bool {
        self.vertical_distance(other) < 0
    }

    pub fn is_above_or_equal(&self, other: Self) -> bool {
        self.vertical_distance(other) <= 0
    }
}

impl fmt::Display for HexPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HexPosition ({}, {})", self.rights, self.downs)
    }
}