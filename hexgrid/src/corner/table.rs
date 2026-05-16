use std::ops::{Index, IndexMut};

use crate::{
    corner::position::CornerPosition,
    hex::{position::HexPosition, table::HexTable},
};

use super::{bounds::CornerBounds, position::CornerHeight};

#[derive(Debug)]
pub struct CornerTable<T> {
    data: HexTable<(Option<T>, Option<T>)>,
    bounds: CornerBounds,
}

impl CornerPosition {
    fn structural_owner(&self) -> HexPosition {
        match *self {
            CornerPosition::High(p) => p + CornerHeight::UP_RIGHT,
            CornerPosition::Low(p) => p + CornerHeight::DOWN_RIGHT,
        }
    }
}

impl<T> CornerTable<T> {
    pub fn new(bounds: CornerBounds) -> Self {
        let data = HexTable::new(bounds.get_hex_bounds());
        CornerTable { data, bounds }
    }

    pub fn get_bounds(&self) -> &CornerBounds {
        &self.bounds
    }

    pub fn get(&self, position: CornerPosition) -> Option<&T> {
        if !self.bounds.contains(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, bottom) = &self.data.get(hex)?;

        match position {
            CornerPosition::High(_) => bottom.as_ref(),
            CornerPosition::Low(_) => top.as_ref(),
        }
    }

    pub fn get_mut(&mut self, position: CornerPosition) -> Option<&mut T> {
        if !self.bounds.contains(position) {
            return None;
        }

        let hex = position.structural_owner();

        let (top, bottom) = self.data.get_mut(hex)?;

        match position {
            CornerPosition::High(_) => bottom.as_mut(),
            CornerPosition::Low(_) => top.as_mut(),
        }
    }

    pub fn set(&mut self, position: CornerPosition, data: T) -> Result<(), ()> {
        if !self.bounds.contains(position) {
            return Err(());
        }

        if self.data.get_mut(position.structural_owner()).is_none() {
            self.data.set(position.structural_owner(), (None, None))?;
        }

        let d = self.data.get_mut(position.structural_owner()).ok_or(())?;

        match position {
            CornerPosition::Low(_) => d.0 = Some(data),
            CornerPosition::High(_) => d.1 = Some(data),
        }

        Ok(())
    }

    pub fn data(&self) -> impl Iterator<Item = &T> {
        self.data.data().flat_map(|((a, b), _)| [a, b]).flatten()
    }
}

impl<T> Index<CornerPosition> for CornerTable<T> {
    type Output = T;

    fn index(&self, index: CornerPosition) -> &Self::Output {
        self.get(index)
            .expect("No data at specified CornerPosition!")
    }
}

impl<T> IndexMut<CornerPosition> for CornerTable<T> {
    fn index_mut(&mut self, index: CornerPosition) -> &mut Self::Output {
        self.get_mut(index)
            .expect("No data at specified CornerPosition!")
    }
}
