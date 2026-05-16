use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use crate::hex::bounds::HexArea;

use super::{bounds::HexPerimeter, position::HexPosition};

#[derive(Debug)]
pub struct HexTable<T> {
    data: Vec<Option<T>>,
    bounds: HexPerimeter,
}

impl HexPerimeter {
    fn get_size(&self) -> usize {
        (self.get_length() * self.get_width()) as usize
    }
}

impl<T> HexTable<T> {
    pub fn new(bounds: HexPerimeter) -> Self {
        let mut data = Vec::with_capacity(bounds.get_size());
        data.resize_with(bounds.get_size(), Default::default);

        HexTable { data, bounds }
    }

    pub fn get_bounds(&self) -> &HexPerimeter {
        &self.bounds
    }

    fn calc_index(&self, position: HexPosition) -> Option<usize> {
        if !self.bounds.contains(position) {
            return None;
        }

        let rights: isize = position
            .horizontal_displacement(self.bounds.get_top_left())
            .ceil()
            .try_into()
            .ok()?;

        let downs: isize = position
            .vertical_displacement(self.bounds.get_top_left())
            .try_into()
            .ok()?;

        let length: isize = self.bounds.get_length().try_into().ok()?;

        downs
            .checked_mul(length)?
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

    pub fn set(&mut self, position: HexPosition, data: T) -> Result<(), ()> {
        if let Some(p) = self.calc_index(position) {
            *self.data.get_mut(p).ok_or(())? = Some(data);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn positions(&self) -> HexArea<'_> {
        self.bounds.area()
    }

    pub fn data(&self) -> HexData<'_, T> {
        HexData::new(&self)
    }
}

impl<T: Debug> Index<HexPosition> for HexTable<T> {
    type Output = T;

    fn index(&self, index: HexPosition) -> &T {
        self.get(index).expect("No data at specified HexPosition!")
    }
}

impl<T: Debug> IndexMut<HexPosition> for HexTable<T> {
    fn index_mut(&mut self, index: HexPosition) -> &mut T {
        self.get_mut(index)
            .expect("No data at specified HexPosition!")
    }
}

pub struct HexData<'a, T> {
    parent: &'a HexTable<T>,
    area: HexArea<'a>,
}

impl<'a, T> HexData<'a, T> {
    fn new(parent: &'a HexTable<T>) -> Self {
        let area = parent.bounds.area();
        HexData { parent, area }
    }
}

impl<'a, T> Iterator for HexData<'a, T> {
    type Item = (&'a T, HexPosition);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(position) = self.area.next() {
            if let Some(data) = self.parent.get(position) {
                return Some((data, position));
            }
        }

        None
    }
}
