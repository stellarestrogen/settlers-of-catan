use crate::{corner::bounds::CornerBounds, edge::bounds::EdgeBounds};

use super::position::{HexPosition, HorizontalDistance};

#[derive(Debug, Clone)]
pub struct HexPerimeter {
    top_left: HexPosition,
    bottom_right: HexPosition,
}

impl HexPerimeter {
    pub fn new() -> Self {
        HexPerimeter {
            top_left: HexPosition::ORIGIN,
            bottom_right: HexPosition::ORIGIN,
        }
    }

    pub fn get_top_left(&self) -> HexPosition {
        self.top_left
    }

    pub fn get_bottom_right(&self) -> HexPosition {
        self.bottom_right
    }

    pub fn get_length(&self) -> i32 {
        self.bottom_right
            .horizontal_distance(self.top_left)
            .ceil()
            .abs()
    }

    pub fn get_width(&self) -> i32 {
        self.bottom_right.vertical_distance(self.top_left).abs()
    }

    pub fn contains(&self, position: HexPosition) -> bool {
        position.is_right_or_equal(self.top_left)
            && position.is_below_or_equal(self.top_left)
            && position.is_left_or_equal(self.bottom_right)
            && position.is_above_or_equal(self.bottom_right)
    }

    /// Expands the current HexPerimeter to include the newly inserted position. If the given position is in bounds, nothing happens.
    pub fn expand(&mut self, position: HexPosition) {
        if self.contains(position) { return }

        let horizontal_distance_top_left = position.horizontal_distance(self.top_left).ceil().abs();
        let vertical_distance_top_left = position.vertical_distance(self.top_left).abs();

        let horizontal_distance_bottom_right =
            position.horizontal_distance(self.bottom_right).ceil().abs();

        let vertical_distance_bottom_right = position.vertical_distance(self.bottom_right).abs();

        if position.is_left(self.top_left) {
            self.top_left += HexPosition::LEFT * horizontal_distance_top_left;
        } else if position.is_right(self.bottom_right) {
            self.bottom_right += HexPosition::RIGHT * horizontal_distance_bottom_right;
        }

        if position.is_above(self.top_left) {
            let vertical_distance = vertical_distance_top_left;

            let pos_offset = position + HexPosition::LEFT * horizontal_distance_top_left;

            let shift: f64 = pos_offset.horizontal_distance(self.top_left).into();

            let adjustment = if shift > 0. {
                HexPosition::UP_RIGHT
            } else if shift < 0. {
                HexPosition::UP_LEFT
            } else if shift == 0. {
                HexPosition::ORIGIN
            } else {
                unreachable!()
            };

            self.top_left += HexPosition::UP_LEFT * (vertical_distance / 2)
                + HexPosition::UP_RIGHT * (vertical_distance / 2)
                + adjustment;

        } else if position.is_below(self.bottom_right) {
            let vertical_distance = vertical_distance_bottom_right;

            let pos_offset = position + HexPosition::RIGHT * horizontal_distance_bottom_right;

            let shift: f64 = pos_offset.horizontal_distance(self.bottom_right).into();

            let adjustment = if shift > 0. {
                HexPosition::DOWN_RIGHT
            } else if shift < 0. {
                HexPosition::DOWN_LEFT
            } else if shift == 0. {
                HexPosition::ORIGIN
            } else {
                unreachable!()
            };

            self.bottom_right += HexPosition::DOWN_LEFT * (vertical_distance / 2)
                + HexPosition::DOWN_RIGHT * (vertical_distance / 2)
                + adjustment;
        }
    }

    pub fn corners(&self) -> CornerBounds {
        CornerBounds::new(self)
    }

    pub fn edges(&self) -> EdgeBounds {
        EdgeBounds::new(self)
    }

    pub fn area<'a>(&'a self) -> impl Iterator<Item = HexPosition> {
        HexArea::new(self)
    }
}

pub struct HexArea<'a> {
    parent: &'a HexPerimeter,
    position: HexPosition,
}

impl<'a> HexArea<'a> {
    fn new(parent: &'a HexPerimeter) -> Self {
        HexArea {
            parent,
            position: HexPosition::ORIGIN,
        }
    }
}

impl<'a> Iterator for HexArea<'a> {
    type Item = HexPosition;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += HexPosition::RIGHT;

        if self.position.is_right(self.parent.get_bottom_right()) {
            let shift = self.position.horizontal_distance(HexPosition::ORIGIN);

            match shift {
                HorizontalDistance::Shifted(_) => self.position += HexPosition::DOWN_RIGHT,
                HorizontalDistance::Unshifted(_) => self.position += HexPosition::DOWN_LEFT,
            }

            self.position += HexPosition::LEFT
                * (self
                    .parent
                    .get_top_left()
                    .horizontal_distance(self.parent.get_bottom_right())
                    .ceil()
                    .abs());
        }

        if !self.position.is_below(self.parent.get_bottom_right()) {
            Some(self.position)
        } else {
            None
        }
    }
}
