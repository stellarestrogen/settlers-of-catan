use super::position::EdgePosition;

pub struct EdgeBounds {
    top_left: EdgePosition,
    bottom_right: EdgePosition
}

impl EdgeBounds {
    pub fn new(top_left: EdgePosition, bottom_right: EdgePosition) -> Self {
        EdgeBounds { 
            top_left, 
            bottom_right 
        }
    }

    pub fn get_top_left(&self) -> EdgePosition {
        self.top_left
    }

    pub fn get_bottom_right(&self) -> EdgePosition {
        self.bottom_right
    }

    pub fn check_bounds(&self, position: EdgePosition) -> bool {
        position.is_right(self.top_left) && position.is_below(self.top_left) &&
        position.is_left(self.bottom_right) && position.is_below(self.bottom_right)
    }

    fn get_length(&self) -> i32 {
        self.bottom_right.horizontal_distance(self.top_left)/2
    }

    fn get_width(&self) -> i32 {
        self.bottom_right.vertical_distance(self.top_left)
    }

    pub fn get_size(&self) -> usize {
        ((self.get_length() * self.get_width() - 2) + (self.get_length() + 1)/2 * (self.get_width() - 1)) as usize
    }
}