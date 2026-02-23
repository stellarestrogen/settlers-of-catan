use std::ops::{Index, IndexMut};

use crate::{
    edge::position::EdgePosition,
    hex::{position::HexPosition, table::HexTable},
};

use super::{bounds::EdgeBounds, position::EdgeOrientation};

#[derive(Debug)]
pub struct EdgeTable<T> {
    data: HexTable<(Option<T>, Option<T>, Option<T>)>,
    bounds: EdgeBounds,
}

impl EdgePosition {
    fn structural_owner(&self) -> HexPosition {
        match *self {
            EdgePosition::Even(p) => p + EdgeOrientation::DOWN_RIGHT,
            EdgePosition::Odd(p) => p + EdgeOrientation::UP_RIGHT,
            EdgePosition::Positive(p) => p + EdgeOrientation::GO_RIGHT,
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

    pub fn get(&self, position: EdgePosition) -> Option<&T> {
        if !self.bounds.contains(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, left, bottom) = &self.data.get(hex)?;

        match position {
            EdgePosition::Even(_) => top.as_ref(),
            EdgePosition::Odd(_) => bottom.as_ref(),
            EdgePosition::Positive(_) => left.as_ref(),
        }
    }

    pub fn get_mut(&mut self, position: EdgePosition) -> Option<&mut T> {
        if !self.bounds.contains(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, left, bottom) = self.data.get_mut(hex)?;

        match position {
            EdgePosition::Even(_) => top.as_mut(),
            EdgePosition::Odd(_) => bottom.as_mut(),
            EdgePosition::Positive(_) => left.as_mut(),
        }
    }

    pub fn set(&mut self, position: EdgePosition, data: T) -> Result<(), ()> {
        if !self.bounds.contains(position) {
            return Err(());
        }
        let d = &mut self.data[position.structural_owner()];

        match position {
            EdgePosition::Even(_) => d.0 = Some(data),
            EdgePosition::Odd(_) => d.2 = Some(data),
            EdgePosition::Positive(_) => d.1 = Some(data),
        }

        Ok(())
    }

    pub fn data(&self) -> impl Iterator<Item = &T> {
        self.data.data().flat_map(|(a, b, c)| [a, b, c]).flatten()
    }
}

impl<T> Index<EdgePosition> for EdgeTable<T> {
    type Output = T;

    fn index(&self, index: EdgePosition) -> &Self::Output {
        self.get(index).expect("No data at specified EdgePosition!")
    }
}

impl<T> IndexMut<EdgePosition> for EdgeTable<T> {
    fn index_mut(&mut self, index: EdgePosition) -> &mut Self::Output {
        self.get_mut(index)
            .expect("No data at specified EdgePosition!")
    }
}
