use super::position::HexPosition;

#[derive(Clone, Debug)]
pub struct HexBounds {
    top_left: HexPosition,
    bottom_right: HexPosition
}

impl HexBounds {
    pub fn new() -> Self {
        HexBounds {
            top_left: HexPosition::ORIGIN,
            bottom_right: HexPosition::ORIGIN
        }
    }

    pub fn get_top_left(&self) -> HexPosition {
        self.top_left
    }

    pub fn get_bottom_right(&self) -> HexPosition {
        self.bottom_right
    }

    pub fn get_length(&self) -> i32 {
        self.bottom_right.horizontal_distance(self.top_left).ceil().abs()
    }

    pub fn get_width(&self) -> i32 {
        self.bottom_right.vertical_distance(self.top_left).abs()
    }

    pub fn check_bounds(&self, position: HexPosition) -> bool {
        position.is_right_or_equal(self.top_left) && 
        position.is_below_or_equal(self.top_left) && 
        position.is_left_or_equal(self.bottom_right) && 
        position.is_above_or_equal(self.bottom_right)
    }

    pub fn expand_bounds(&mut self, position: HexPosition) {
        if position.is_left(self.top_left) {
            self.top_left += HexPosition::LEFT * position.horizontal_distance(self.top_left).ceil().abs();
        } else if position.is_right(self.bottom_right) {
            self.bottom_right += HexPosition::RIGHT * position.horizontal_distance(self.bottom_right).ceil().abs();
        }
        
        if position.is_above(self.top_left) {
            let vertical_distance = position.vertical_distance(self.top_left).abs();
            let pos_offset = position + HexPosition::LEFT * position.horizontal_distance(self.top_left).ceil().abs();
            let shift: f64 = pos_offset.horizontal_distance(self.top_left).into();
            let adjustment = if shift > 0. { HexPosition::UP_RIGHT } else if shift < 0. { HexPosition::UP_LEFT } else { HexPosition::ORIGIN };
            self.top_left += HexPosition::UP_LEFT * (vertical_distance/2) + HexPosition::UP_RIGHT * (vertical_distance/2) + adjustment;

        } else if position.is_below(self.bottom_right) {
            let vertical_distance = position.vertical_distance(self.bottom_right).abs();
            let pos_offset = position + HexPosition::RIGHT * position.horizontal_distance(self.bottom_right).ceil().abs();
            let shift: f64 = pos_offset.horizontal_distance(self.bottom_right).into();
            let adjustment = if shift > 0. { HexPosition::DOWN_RIGHT } else if shift < 0. { HexPosition::DOWN_LEFT } else { HexPosition::ORIGIN };
            self.bottom_right += HexPosition::DOWN_LEFT * (vertical_distance/2) + HexPosition::DOWN_RIGHT * (vertical_distance/2) + adjustment;
        }
    }
}