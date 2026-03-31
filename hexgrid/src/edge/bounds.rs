use crate::{
    edge::position::EdgePosition,
    hex::{
        bounds::HexPerimeter,
        position::{HexPosition, HorizontalDistance},
    },
};

use super::position::EdgeOrientation;

#[derive(Debug, Clone)]
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

    pub fn contains(&self, position: EdgePosition) -> bool {
        let top_row: EdgePosition =
            (self.bounds.get_top_left() + EdgeOrientation::BOTTOM_LEFT).into();
        let bottom_row: EdgePosition =
            (self.bounds.get_bottom_right() + EdgeOrientation::TOP_LEFT).into();

        if top_row.vertical_distance(position) > 0 || bottom_row.vertical_distance(position) < 0 {
            return false;
        }

        let hex = match position {
            EdgePosition::Even(p) => p + EdgeOrientation::DOWN_RIGHT,
            EdgePosition::Odd(p) => p + EdgeOrientation::UP_RIGHT,
            EdgePosition::Positive(p) => p + EdgeOrientation::GO_RIGHT,
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
