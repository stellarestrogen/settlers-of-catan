use std::ops::{Index, IndexMut};

use crate::{hex::{position::HexPosition, table::HexTable}, position::Position};

use super::{bounds::CornerBounds, position::CornerPosition};


pub struct CornerTable<T> {
    data: HexTable<(Option<T>, Option<T>)>,
    bounds: CornerBounds
}

impl CornerPosition {
    pub(in super) fn structural_owner(&self) -> HexPosition {
        let top_left_corner = if self.calc_distance(CornerPosition::EMPTY) % 2 == 0 { *self } else { *self + CornerPosition::UP };
        let rights = top_left_corner.horizontal_distance(CornerPosition::EMPTY);
        let downs = top_left_corner.vertical_distance(CornerPosition::EMPTY);

        if rights.signum() == downs.signum() {
            HexPosition::DOWN_RIGHT * downs + ((rights - downs)/2) * HexPosition::RIGHT
        } else {
            HexPosition::DOWN_LEFT * downs + ((rights + downs)/2) * HexPosition::RIGHT
        }

    }
}

impl<T> CornerTable<T> {
    fn new(bounds: CornerBounds) -> Self {
        CornerTable {
            data: HexTable::new(bounds.get_hex_bounds()),
            bounds
        }
    }

    pub fn get(&self, position: CornerPosition) -> Option<&T> {
        if !self.bounds.check_bounds(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, bottom) = &self.data.get(hex)?;

        if position.calc_distance(CornerPosition::EMPTY) % 2 == 0 {
            top.as_ref()
        } else {
            bottom.as_ref()
        }
    }

    pub fn get_mut(&mut self, position: CornerPosition) -> Option<&mut T> {
        if !self.bounds.check_bounds(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, bottom) = self.data.get_mut(hex)?;

        if position.calc_distance(CornerPosition::EMPTY) % 2 == 0 {
            top.as_mut()
        } else {
            bottom.as_mut()
        }
    }
}

impl<T> Index<CornerPosition> for CornerTable<T> {
    type Output = T;

    fn index(&self, index: CornerPosition) -> &Self::Output {
        self.get(index).expect("No data at specified CornerPosition!")
    }
}

impl<T> IndexMut<CornerPosition> for CornerTable<T> {
    fn index_mut(&mut self, index: CornerPosition) -> &mut Self::Output {
        self.get_mut(index).expect("No data at specified CornerPosition!")
    }
}