use std::ops::{Index, IndexMut};

use crate::hex::{position::HexPosition, table::HexTable};

use super::{bounds::CornerBounds, position::{CornerPosition, Height}};


pub struct CornerTable<T> {
    data: HexTable<(Option<T>, Option<T>)>,
    bounds: CornerBounds
}

impl<H: Height> CornerPosition<H> {
    fn structural_owner(&self) -> HexPosition {
        if let Some(p) = self.as_low() {
            p + CornerPosition::DOWN_RIGHT
        } else if let Some(p) = self.as_high() {
            p + CornerPosition::UP_RIGHT
        } else {
            unreachable!()
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

    pub fn get<H: Height>(&self, position: CornerPosition<H>) -> Option<&T> {
        if !self.bounds.check_bounds(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, bottom) = &self.data.get(hex)?;

        if position.is_low() {
            top.as_ref()
        } else if position.is_high() {
            bottom.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn get_mut<H: Height>(&mut self, position: CornerPosition<H>) -> Option<&mut T> {
        if !self.bounds.check_bounds(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, bottom) = self.data.get_mut(hex)?;

        if position.is_low() {
            top.as_mut()
        } else if position.is_high() {
            bottom.as_mut()
        } else {
            unreachable!()
        }
    }
}

impl<T, H: Height> Index<CornerPosition<H>> for CornerTable<T> {
    type Output = T;

    fn index(&self, index: CornerPosition<H>) -> &Self::Output {
        self.get(index).expect("No data at specified CornerPosition!")
    }
}

impl<T, H: Height> IndexMut<CornerPosition<H>> for CornerTable<T> {
    fn index_mut(&mut self, index: CornerPosition<H>) -> &mut Self::Output {
        self.get_mut(index).expect("No data at specified CornerPosition!")
    }
}