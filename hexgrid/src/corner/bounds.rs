use crate::{
    corner::position::{CornerHeight, CornerPosition},
    hex::{
        bounds::HexBounds,
        position::{HexPosition, HorizontalDisplacement},
    },
};

#[derive(Debug, Clone)]
pub struct CornerBounds {
    bounds: HexBounds,
}

impl CornerBounds {
    pub fn new(hex_bounds: &HexBounds) -> Self {
        let mut bounds: HexBounds = hex_bounds.clone();
        bounds.expand(bounds.get_top_left() + HexPosition::UP_RIGHT);
        bounds.expand(bounds.get_bottom_right() + HexPosition::RIGHT);
        bounds.expand(bounds.get_bottom_right() + HexPosition::DOWN_LEFT);
        CornerBounds { bounds }
    }

    pub fn get_hex_bounds(&self) -> HexBounds {
        self.bounds.clone()
    }

    pub fn get_top_left(&self) -> CornerPosition {
        let hex_top_left = self.bounds.get_top_left();
        match hex_top_left.horizontal_displacement(HexPosition::ORIGIN) {
            HorizontalDisplacement::Shifted(_) => (hex_top_left + CornerHeight::BOTTOM).into(),
            HorizontalDisplacement::Unshifted(_) => {
                ((hex_top_left + HexPosition::LEFT) + CornerHeight::BOTTOM).into()
            }
        }
    }

    pub fn get_bottom_right(&self) -> CornerPosition {
        let hex_bottom_right = self.bounds.get_bottom_right();

        match hex_bottom_right.horizontal_displacement(HexPosition::ORIGIN) {
            HorizontalDisplacement::Shifted(_) => (hex_bottom_right + CornerHeight::TOP).into(),
            HorizontalDisplacement::Unshifted(_) => {
                ((hex_bottom_right + HexPosition::LEFT) + CornerHeight::TOP).into()
            }
        }
    }

    fn is_invalid_hex(&self, position: HexPosition) -> bool {
        let top_left = self.bounds.get_top_left();
        let bottom_right = self.bounds.get_bottom_right();

        let mut hex1 = top_left;
        let mut hex2 = bottom_right;

        let length = self.bounds.get_width();

        if let HorizontalDisplacement::Unshifted(_) =
            top_left.horizontal_displacement(HexPosition::ORIGIN)
        {
            hex1 = top_left + HexPosition::RIGHT * length;
        }

        if let HorizontalDisplacement::Shifted(_) =
            bottom_right.horizontal_displacement(HexPosition::ORIGIN)
        {
            hex2 = bottom_right + HexPosition::LEFT * length;
        }

        position == hex1 || position == hex2
    }

    pub fn contains(&self, position: CornerPosition) -> bool {
        let top_row: CornerPosition =
            (self.bounds.get_top_left() + CornerHeight::BOTTOM_LEFT).into();
        let bottom_row: CornerPosition =
            (self.bounds.get_bottom_right() + CornerHeight::TOP_LEFT).into();

        if position.is_above(top_row) || position.is_below(bottom_row) {
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

    pub fn area(&self) -> CornerArea<'_> {
        CornerArea::new(self)
    }
}

pub struct CornerArea<'a> {
    parent: &'a CornerBounds,
    position: CornerPosition,
}

impl<'a> CornerArea<'a> {
    fn new(parent: &'a CornerBounds) -> Self {
        let position = parent.get_top_left();
        CornerArea { parent, position }
    }
}

impl<'a> Iterator for CornerArea<'a> {
    type Item = CornerPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_position = self.position;

        if !self.parent.contains(current_position) {
            current_position = match current_position {
                CornerPosition::High(p) => p.go_left().go_down().into(),
                CornerPosition::Low(p) => p.go_down().go_left().into(),
            };

            while self.parent.contains(current_position) {
                current_position = current_position.go_left();
            }

            current_position = current_position.go_right();
        }

        if !current_position.is_below(self.parent.get_bottom_right().go_left()) {
            self.position = current_position.go_right();
            Some(current_position)
        } else {
            None
        }
    }
}
