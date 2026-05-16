use std::thread::current;

use crate::{corner::bounds::CornerBounds, edge::bounds::EdgeBounds};

use super::position::{HexPosition, HorizontalDisplacement};

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

    pub fn get_length(&self) -> u32 {
        self.bottom_right
            .horizontal_displacement(self.top_left)
            .ceil()
            .abs() as u32
            + 1
    }

    pub fn get_width(&self) -> u32 {
        self.bottom_right.vertical_displacement(self.top_left).abs() as u32 + 1
    }

    pub fn contains(&self, position: HexPosition) -> bool {
        position.is_right_or_equal_raw(self.top_left)
            && position.is_below_or_equal(self.top_left)
            && position.is_left_or_equal_raw(self.bottom_right)
            && position.is_above_or_equal(self.bottom_right)
    }

    /// Expands the current HexPerimeter to include the newly inserted position. If the given position is in bounds, nothing happens.
    pub fn expand(&mut self, position: HexPosition) {
        if self.contains(position) {
            return;
        }

        let vertical_distance_top_left = position.vertical_displacement(self.top_left).abs();

        let vertical_distance_bottom_right =
            position.vertical_displacement(self.bottom_right).abs();

        if position.is_left_raw(self.top_left) {
            self.top_left +=
                HexPosition::LEFT * position.raw_horizontal_displacement(self.top_left).abs();
        } else if position.is_right_raw(self.bottom_right) {
            self.bottom_right += HexPosition::RIGHT
                * position
                    .raw_horizontal_displacement(self.bottom_right)
                    .abs();
        }

        if position.is_above(self.top_left) {
            let vertical_distance = vertical_distance_top_left;

            let shift = self
                .bottom_right
                .horizontal_displacement(HexPosition::ORIGIN);

            let adjustment = if vertical_distance % 2 == 0 {
                HexPosition::ORIGIN
            } else {
                match shift {
                    HorizontalDisplacement::Shifted(_) => HexPosition::UP_RIGHT,
                    HorizontalDisplacement::Unshifted(_) => HexPosition::UP_LEFT,
                }
            };

            self.top_left += HexPosition::UP_LEFT * (vertical_distance / 2)
                + HexPosition::UP_RIGHT * (vertical_distance / 2)
                + adjustment;
        } else if position.is_below(self.bottom_right) {
            let vertical_distance = vertical_distance_bottom_right;

            let shift = self
                .bottom_right
                .horizontal_displacement(HexPosition::ORIGIN);

            let adjustment = if vertical_distance % 2 == 0 {
                HexPosition::ORIGIN
            } else {
                match shift {
                    HorizontalDisplacement::Shifted(_) => HexPosition::DOWN_RIGHT,
                    HorizontalDisplacement::Unshifted(_) => HexPosition::DOWN_LEFT,
                }
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

    pub fn area(&self) -> HexArea {
        HexArea::new(self)
    }
}

pub struct HexArea<'a> {
    parent: &'a HexPerimeter,
    position: HexPosition,
}

impl<'a> HexArea<'a> {
    fn new(parent: &'a HexPerimeter) -> Self {
        let position = parent.get_top_left();
        HexArea { parent, position }
    }
}

impl<'a> Iterator for HexArea<'a> {
    type Item = HexPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_position = self.position;

        if current_position.is_right_raw(self.parent.get_bottom_right()) {
            let shift = current_position.horizontal_displacement(HexPosition::ORIGIN);

            match shift {
                HorizontalDisplacement::Shifted(_) => current_position += HexPosition::DOWN_RIGHT,
                HorizontalDisplacement::Unshifted(_) => current_position += HexPosition::DOWN_LEFT,
            }

            current_position += HexPosition::LEFT
                * (self
                    .parent
                    .get_top_left()
                    .horizontal_displacement(self.parent.get_bottom_right())
                    .ceil()
                    .abs() + 1);

        }

        if !current_position.is_below(self.parent.get_bottom_right()) {
            self.position = current_position + HexPosition::RIGHT;
            Some(current_position)
        } else {
            None
        }
    }
}
