use std::ops::{Index, IndexMut};

use crate::hex::{position::HexPosition, table::HexTable};

use super::{
    bounds::EdgeBounds,
    position::{EdgePosition, Valid},
};

#[derive(Debug)]
pub struct EdgeTable<T> {
    data: HexTable<(Option<T>, Option<T>, Option<T>)>,
    bounds: EdgeBounds,
}

impl<Type: Valid> EdgePosition<Type> {
    fn structural_owner(&self) -> HexPosition {
        if let Some(p) = self.as_even() {
            p + EdgePosition::DOWN_RIGHT
        } else if let Some(p) = self.as_positive() {
            p + EdgePosition::GO_RIGHT
        } else if let Some(p) = self.as_odd() {
            p + EdgePosition::UP_RIGHT
        } else {
            unreachable!()
        }
    }
}

impl<T> EdgeTable<T> {
    pub fn new(bounds: EdgeBounds) -> Self {
        EdgeTable {
            data: HexTable::new(bounds.get_hex_bounds()),
            bounds,
        }
    }

    pub fn get<Type: Valid>(&self, position: EdgePosition<Type>) -> Option<&T> {
        if !self.bounds.contains(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, left, bottom) = &self.data.get(hex)?;

        if position.is_even() {
            top.as_ref()
        } else if position.is_positive() {
            left.as_ref()
        } else if position.is_odd() {
            bottom.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn get_mut<Type: Valid>(&mut self, position: EdgePosition<Type>) -> Option<&mut T> {
        if !self.bounds.contains(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, left, bottom) = self.data.get_mut(hex)?;

        if position.is_even() {
            top.as_mut()
        } else if position.is_positive() {
            left.as_mut()
        } else if position.is_odd() {
            bottom.as_mut()
        } else {
            unreachable!()
        }
    }

    pub fn set<Type: Valid>(&mut self, position: EdgePosition<Type>, data: T) -> Result<(), ()> {
        if !self.bounds.contains(position) {
            return Err(());
        }
        let d = &mut self.data[position.structural_owner()];

        if position.is_even() {
            d.0 = Some(data);
        } else if position.is_positive() {
            d.1 = Some(data)
        } else if position.is_odd() {
            d.2 = Some(data)
        } else {
            unreachable!()
        }

        Ok(())
    }
}

impl<T, Type: Valid> Index<EdgePosition<Type>> for EdgeTable<T> {
    type Output = T;

    fn index(&self, index: EdgePosition<Type>) -> &Self::Output {
        self.get(index).expect("No data at specified EdgePosition!")
    }
}

impl<T, Type: Valid> IndexMut<EdgePosition<Type>> for EdgeTable<T> {
    fn index_mut(&mut self, index: EdgePosition<Type>) -> &mut Self::Output {
        self.get_mut(index)
            .expect("No data at specified EdgePosition!")
    }
}
