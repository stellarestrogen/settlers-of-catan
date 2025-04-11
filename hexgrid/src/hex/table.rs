use std::ops::{Index, IndexMut};

use super::{bounds::HexBounds, position::HexPosition};


pub struct HexTable<T> {
    data: Vec<Option<T>>,
    bounds: HexBounds
}

impl HexBounds {
    fn get_size(&self) -> usize {
        (self.get_length() * self.get_width()) as usize
    }
}

impl<T> HexTable<T> {
    pub fn new(bounds: HexBounds) -> Self {
        let mut data = Vec::with_capacity(bounds.get_size());
        data.resize_with(bounds.get_size(), Default::default);
        
        HexTable {
            data,
            bounds
        }
    }

    fn calc_index(&self, position: HexPosition) -> Option<usize> {
        if !self.bounds.check_bounds(position) {
            return None
        }

        let rights: isize = position
            .horizontal_distance(self.bounds.get_top_left())
            .ceil()
            .try_into()
            .ok()?;

        let downs: isize = position
            .vertical_distance(self.bounds.get_top_left())
            .try_into()
            .ok()?;

        let length: isize = self.bounds.get_length().try_into().ok()?;

        downs.checked_mul(length)?
            .checked_add(rights)?
            .try_into()
            .ok()
    }

    pub fn get(&self, position: HexPosition) -> Option<&T> {
        self.data[self.calc_index(position)?].as_ref()
    }

    pub fn get_mut(&mut self, position: HexPosition) -> Option<&mut T> {
        let idx = self.calc_index(position)?;
        self.data[idx].as_mut()
    }
}

impl<T> Index<HexPosition> for HexTable<T> {
    type Output = T;

    fn index(&self, index: HexPosition) -> &T {
        self.get(index).expect("No data at specified HexPosition!")
    }
}

impl<T> IndexMut<HexPosition> for HexTable<T> {
    fn index_mut(&mut self, index: HexPosition) -> &mut T {
        self.get_mut(index)
            .expect("No data at specified HexPosition!")
    }
}
