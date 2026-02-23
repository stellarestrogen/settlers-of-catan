use crate::{
    corner::position::{CornerHeight, CornerPosition},
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

    pub fn contains(&self, position: CornerPosition) -> bool {
        let top_row: CornerPosition = (self.bounds.get_top_left() + CornerHeight::BOTTOM_LEFT).into();
        let bottom_row: CornerPosition = (self.bounds.get_bottom_right() + CornerHeight::TOP_LEFT).into();

        if top_row.vertical_distance(position) > 0 || bottom_row.vertical_distance(position) < 0 {
            return false;
        }

        let hex: HexPosition = match position {
            CornerPosition::High(p) => p + CornerHeight::UP_RIGHT,
            CornerPosition::Low(p) => p + CornerHeight::DOWN_RIGHT,
        };

        if self.is_invalid_hex(hex) {
            return false;
        }

        self.bounds.contains(hex)
    }
}
