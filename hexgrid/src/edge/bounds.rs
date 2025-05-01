use crate::hex::{
    bounds::HexPerimeter,
    position::{HexPosition, HorizontalDistance},
};

use super::position::{EdgePosition, Valid};

pub struct EdgeBounds {
    bounds: HexPerimeter,
}

impl EdgeBounds {
    pub fn new(hex_bounds: &HexPerimeter) -> Self {
        let mut bounds: HexPerimeter = hex_bounds.clone();
        bounds.expand(bounds.get_top_left() + HexPosition::UP_RIGHT);
        bounds.expand(bounds.get_bottom_right() + HexPosition::RIGHT + HexPosition::DOWN_LEFT);

        EdgeBounds { bounds }
    }

    fn is_invalid_hex(&self, position: HexPosition) -> bool {
        let top_left = self.bounds.get_top_left();
        let bottom_right = self.bounds.get_bottom_right();

        let mut hex1 = top_left;
        let mut hex2 = bottom_right;

        let length = self.bounds.get_length();

        if let HorizontalDistance::Unshifted(_) = top_left.horizontal_distance(HexPosition::ORIGIN)
        {
            hex1 = top_left + length * HexPosition::RIGHT;
        }

        if let HorizontalDistance::Shifted(_) =
            bottom_right.horizontal_distance(HexPosition::ORIGIN)
        {
            hex2 = bottom_right + length * HexPosition::LEFT;
        }

        position == hex1 || position == hex2
    }

    pub fn contains<T: Valid>(&self, position: EdgePosition<T>) -> bool {
        let top_row = self.bounds.get_top_left() + EdgePosition::BOTTOM_LEFT;
        let bottom_row = self.bounds.get_bottom_right() + EdgePosition::TOP_LEFT;

        if top_row.vertical_distance(position) > 0 || bottom_row.vertical_distance(position) < 0 {
            return false;
        }

        let hex = if let Some(p) = position.as_even() {
            p + EdgePosition::DOWN_RIGHT
        } else if let Some(p) = position.as_positive() {
            p + EdgePosition::GO_RIGHT
        } else if let Some(p) = position.as_odd() {
            p + EdgePosition::UP_RIGHT
        } else {
            unreachable!()
        };

        if self.is_invalid_hex(hex) {
            return false;
        }

        self.bounds.contains(hex)
    }

    pub fn get_hex_bounds(&self) -> HexPerimeter {
        self.bounds.clone()
    }
}
