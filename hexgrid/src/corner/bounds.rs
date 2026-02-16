use crate::{
    corner::position::{CornerPosition, Height},
    hex::{
        bounds::HexPerimeter,
        position::{HexPosition, HorizontalDistance},
    },
};

#[derive(Debug, Clone)]
pub struct CornerBounds {
    bounds: HexPerimeter,
}

impl CornerBounds {
    pub fn new(hex_bounds: &HexPerimeter) -> Self {
        let mut bounds: HexPerimeter = hex_bounds.clone();
        bounds.expand(bounds.get_top_left() + HexPosition::UP_RIGHT);
        bounds.expand(bounds.get_bottom_right() + HexPosition::RIGHT + HexPosition::DOWN_LEFT);

        CornerBounds { bounds }
    }

    pub fn get_hex_bounds(&self) -> HexPerimeter {
        self.bounds.clone()
    }

    fn is_invalid_hex(&self, position: HexPosition) -> bool {
        let top_left = self.bounds.get_top_left();
        let bottom_right = self.bounds.get_bottom_right();

        let mut hex1 = top_left;
        let mut hex2 = bottom_right;

        let length = self.bounds.get_length();

        if let HorizontalDistance::Unshifted(_) = top_left.horizontal_distance(HexPosition::ORIGIN)
        {
            hex1 = top_left + HexPosition::RIGHT * length;
        }

        if let HorizontalDistance::Shifted(_) =
            bottom_right.horizontal_distance(HexPosition::ORIGIN)
        {
            hex2 = bottom_right + HexPosition::LEFT * length;
        }

        position == hex1 || position == hex2
    }

    pub fn contains<H: Height>(&self, position: CornerPosition<H>) -> bool {
        let top_row = self.bounds.get_top_left() + CornerPosition::BOTTOM_LEFT;
        let bottom_row = self.bounds.get_bottom_right() + CornerPosition::TOP_LEFT;

        if top_row.vertical_distance(position) > 0 || bottom_row.vertical_distance(position) < 0 {
            return false;
        }

        let hex = if let Some(p) = position.as_low() {
            p + CornerPosition::DOWN_RIGHT
        } else if let Some(p) = position.as_high() {
            p + CornerPosition::UP_RIGHT
        } else {
            unreachable!()
        };

        if self.is_invalid_hex(hex) {
            return false;
        }

        self.bounds.contains(hex)
    }
}
