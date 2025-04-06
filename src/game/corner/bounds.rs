use crate::game::position::Position;

use super::position::CornerPosition;

pub struct CornerBounds {
    top_left: CornerPosition,
    bottom_right: CornerPosition
}

impl CornerBounds {
    pub fn new(top_left: CornerPosition, bottom_right: CornerPosition) -> Self {
        CornerBounds {
            top_left,
            bottom_right
        }
    }

    pub fn get_top_left(&self) -> CornerPosition {
        self.top_left
    }

    pub fn get_bottom_right(&self) -> CornerPosition {
        self.bottom_right
    }

    pub fn check_bounds(&self, position: CornerPosition) -> bool {
        position.is_right(self.top_left) && position.is_below(self.top_left) &&
        position.is_left(self.bottom_right) && position.is_above(self.bottom_right)
    }

    pub fn get_length(&self) -> i32 {
        self.bottom_right.horizontal_distance(self.top_left).abs()
    }

    pub fn get_width(&self) -> i32 {
        self.bottom_right.vertical_distance(self.top_left).abs()
    }

    /// The total size of the bounds. -2 because there are 2 corner positions which do not technically exist,
    /// which are the positions of the bounds themselves.
    pub fn get_size(&self) -> usize {
        (self.get_length() * self.get_width() - 2) as usize
    }
}