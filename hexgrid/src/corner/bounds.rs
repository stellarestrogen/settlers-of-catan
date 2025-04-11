use crate::hex::{bounds::HexBounds, position::{HexPosition, HorizontalDistance}};

use super::position::{CornerPosition, Height};

pub struct CornerBounds {
    bounds: HexBounds
}

impl CornerBounds {
    pub fn new(hex_bounds: &HexBounds) -> Self {
        let mut bounds: HexBounds = hex_bounds.clone();
        bounds.expand_bounds(bounds.get_top_left() + HexPosition::UP_RIGHT);
        bounds.expand_bounds(bounds.get_bottom_right() + HexPosition::RIGHT + HexPosition::DOWN_LEFT);

        CornerBounds {
            bounds
        }
    }

    pub fn get_hex_bounds(&self) -> HexBounds {
        self.bounds.clone()
    }

    fn is_invalid_hex(&self, position: HexPosition) -> bool {
        let mut hex1 = self.bounds.get_top_left();
        let mut hex2 = self.bounds.get_bottom_right();

        if let HorizontalDistance::Unshifted(_) = self.bounds.get_top_left().horizontal_distance(HexPosition::ORIGIN) {
            hex1 = self.bounds.get_top_left() + self.bounds.get_bottom_right().horizontal_distance(self.bounds.get_top_left()).ceil().abs() * HexPosition::RIGHT;
        }

        if let HorizontalDistance::Shifted(_) = self.bounds.get_bottom_right().horizontal_distance(HexPosition::ORIGIN) {
            hex2 = self.bounds.get_bottom_right() + self.bounds.get_bottom_right().horizontal_distance(self.bounds.get_top_left()).ceil().abs() * HexPosition::LEFT;
        }

        position == hex1 || position == hex2
    
    }

    pub fn check_bounds<H: Height>(&self, position: CornerPosition<H>) -> bool {
        if (self.bounds.get_top_left() + CornerPosition::BOTTOM_LEFT).vertical_distance(position) > 0 ||
        (self.bounds.get_bottom_right() + CornerPosition::TOP_LEFT).vertical_distance(position) < 0 {
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

        self.bounds.check_bounds(hex)

    }

}