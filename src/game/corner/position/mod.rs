pub mod op_add;
pub mod op_sub;
pub mod op_mul;

/// A CornerPosition is the distance rightwards and downwards from the "origin" corner, which the top-leftmost corner.
/// Going right adds 1 to `rights`, going left subtracts 1.
/// Going down adds 1 to `downs`, going up subtracts 1.
#[derive(Clone, Copy)]
pub struct CornerPosition {
    rights: i32,
    downs: i32
}

impl CornerPosition {
    pub const EMPTY: CornerPosition = CornerPosition {
        rights: 0,
        downs: 0
    };

    pub const RIGHT: CornerPosition = CornerPosition {
        rights: 1,
        downs: 0
    };

    pub const DOWN: CornerPosition = CornerPosition {
        rights: 0,
        downs: 1
    };

    pub const LEFT: CornerPosition = CornerPosition {
        rights: -1,
        downs: 0
    };

    pub const UP: CornerPosition = CornerPosition {
        rights: 0,
        downs: -1
    };

    pub fn horizontal_distance(&self, other: CornerPosition) -> i32 {
        self.rights - other.rights
    }

    pub fn vertical_distance(&self, other: CornerPosition) -> i32 {
        self.downs - other.downs
    }

    pub fn is_right(&self, other: CornerPosition) -> bool {
        self.horizontal_distance(other) > 0
    }

    pub fn is_left(&self, other: CornerPosition) -> bool {
        self.horizontal_distance(other) < 0
    }

    pub fn is_below(&self, other: CornerPosition) -> bool {
        self.vertical_distance(other) > 0
    }

    pub fn is_above(&self, other: CornerPosition) -> bool {
        self.vertical_distance(other) < 0
    }

    pub fn calc_distance(&self, other: CornerPosition) -> i32 {
        let distance = other - *self;
        let mut distance = distance.rights.abs() + distance.downs.abs();

        if self.rights == other.rights {
            distance = distance * 2 - if distance % 2 == 0 { 0 } else { 1 };
        }

        distance
    }
}