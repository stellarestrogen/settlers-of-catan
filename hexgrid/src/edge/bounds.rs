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
        bounds.expand_bounds(bounds.get_top_left() + HexPosition::UP_RIGHT);
        bounds
            .expand_bounds(bounds.get_bottom_right() + HexPosition::RIGHT + HexPosition::DOWN_LEFT);

        EdgeBounds { bounds }
    }

    fn is_invalid_hex(&self, position: HexPosition) -> bool {
        let mut hex1 = self.bounds.get_top_left();
        let mut hex2 = self.bounds.get_bottom_right();

        if let HorizontalDistance::Unshifted(_) = self
            .bounds
            .get_top_left()
            .horizontal_distance(HexPosition::ORIGIN)
        {
            hex1 = self.bounds.get_top_left()
                + self
                    .bounds
                    .get_bottom_right()
                    .horizontal_distance(self.bounds.get_top_left())
                    .ceil()
                    .abs()
                    * HexPosition::RIGHT;
        }

        if let HorizontalDistance::Shifted(_) = self
            .bounds
            .get_bottom_right()
            .horizontal_distance(HexPosition::ORIGIN)
        {
            hex2 = self.bounds.get_bottom_right()
                + self
                    .bounds
                    .get_bottom_right()
                    .horizontal_distance(self.bounds.get_top_left())
                    .ceil()
                    .abs()
                    * HexPosition::LEFT;
        }

        position == hex1 || position == hex2
    }

    pub fn check_bounds<T: Valid>(&self, position: EdgePosition<T>) -> bool {
        if (self.bounds.get_top_left() + EdgePosition::BOTTOM_LEFT).vertical_distance(position) > 0
            || (self.bounds.get_bottom_right() + EdgePosition::TOP_LEFT).vertical_distance(position)
                < 0
        {
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

        self.bounds.check_bounds(hex)
    }

    pub fn get_hex_bounds(&self) -> HexPerimeter {
        self.bounds.clone()
    }
}
