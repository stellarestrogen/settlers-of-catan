use std::iter;
use std::ops::{Index, IndexMut};

use crate::game::objects::TradeType;

use super::bounds::CornerBounds;
use super::Corner;
use super::position::CornerPosition;

pub struct CornerHolder {
    corners: Vec<Corner>,
    bounds: CornerBounds
}

impl CornerHolder {
    pub fn new(bounds: CornerBounds) -> Self {
        let corners = iter::repeat_n(Corner::new(), bounds.get_size()).collect();
        
        CornerHolder {
            corners,
            bounds
        }
    }

    fn calc_index(&self, position: CornerPosition) -> Option<usize> {
        if !self.bounds.check_bounds(position) {
            return None;
        }

        let rights: isize = position.horizontal_distance(self.bounds.get_top_left()).try_into().ok()?;
        let downs: isize = position.vertical_distance(self.bounds.get_top_left()).try_into().ok()?;
        let length: isize = self.bounds.get_length().try_into().ok()?;

        Some(downs.checked_mul(length)?.checked_add(rights)?.try_into().ok()?)
    }

    /// Gets a mutable reference to the corner referenced by the CornerPosition.
    pub fn get_mut(&mut self, position: CornerPosition) -> Option<&mut Corner> {
        let idx = self.calc_index(position)?;

        // note: used `Option` rather than `Err` because there's only 1 error case and it's obvious what it would be (OOB)
        Some(&mut self.corners[idx])
    }

    /// Gets a reference to the corner referenced by the CornerPosition.
    pub fn get(&self, position: CornerPosition) -> Option<&Corner> {
        // note: used `Option` rather than `Err` because there's only 1 error case and it's obvious what it would be (OOB)
        Some(&self.corners[self.calc_index(position)?])
    }

    /// Sets the trades for the corners.
    pub fn set_trades(&mut self, trades: impl Iterator<Item = (TradeType, CornerPosition)>) -> Result<(), ()> {
        trades
            .map(|(t, p)| Ok(self[p].set_trade(t)))
            .collect::<Result<(), ()>>()
    }
}

impl Index<CornerPosition> for CornerHolder {
    type Output = Corner;

    fn index(&self, index: CornerPosition) -> &Self::Output {
        self.get(index).expect("CornerPosition out of bounds!")
    }
}

impl IndexMut<CornerPosition> for CornerHolder {
    fn index_mut(&mut self, index: CornerPosition) -> &mut Self::Output {
        self.get_mut(index).expect("CornerPosition out of bounds!")
    }
}