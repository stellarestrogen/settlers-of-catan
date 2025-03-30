use super::objects::{Building, TradeType};
use std::iter;

#[derive(Clone, Copy)]
pub struct Corner {
    building: Option<Building>,
    trade_type: Option<TradeType>,
}

impl Corner {
    pub fn new() -> Self {
        Corner {
            building: None,
            trade_type: None
        }
    }

    pub fn set_building(&mut self, building: Building) {
        self.building = Some(building)
    }

    pub fn unset_building (&mut self) {
        self.building = None
    }

    pub fn set_trade(&mut self, trade_type: TradeType) {
        self.trade_type = Some(trade_type)
    }

    pub fn unset_trade(&mut self) {
        self. trade_type = None
    }
}

/// A CornerPosition is the distance rightwards and downwards from the "origin" corner, which the top-leftmost corner.
/// Going right adds 1 to `rights`, going left subtracts 1.
/// Going down adds 1 to `downs`, going up subtracts 1.
#[derive(Clone, Copy)]
struct CornerPosition {
    rights: i32,
    downs: i32
}

impl CornerPosition {
    pub fn new(rights: i32, downs: i32) -> Self {
        CornerPosition {
            rights,
            downs
        }
    }

    pub fn get_rights(&self) -> i32 {
        self.rights
    }

    pub fn get_downs(&self) -> i32 {
        self.downs
    }

    pub fn calc_distance(&self, other: CornerPosition) -> i32 {
        let mut distance = (self.get_rights() - other.get_rights()).abs() + (self.get_downs() - other.get_downs()).abs();

        if self.get_rights() == other.get_rights() {
            distance = distance * 2 - if distance % 2 == 0 { 0 } else { 1 };
        }

        distance
    }
}

pub struct CornerHolder {
    corners: Vec<Corner>,
    length: u32,
    width: u32
}

impl CornerHolder {
    pub fn new(length: u32, width: u32) -> Self {
        let corners = iter::repeat_n(Corner::new(), (length * width - 2) as usize).collect();
        
        CornerHolder {
            corners,
            length,
            width
        }
    }

    fn calc_index(&self, position: CornerPosition) -> Option<usize> {
        let downs: isize = position.get_downs().try_into().ok()?;
        let rights: isize = position.get_rights().try_into().ok()?;
        let width: isize = self.width.try_into().ok()?;
        let length: isize = self.length.try_into().ok()?;

        let idx: usize = downs.checked_mul(length)?.checked_add(rights)?.try_into().ok()?;

        if downs < width && rights < length.checked_sub(2)? && idx < self.get_size() {
            Some(idx)
        } else {
            None
        }
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
    pub fn set_trades(&mut self, trades: Vec<(TradeType, CornerPosition)>) -> Result<(), ()> {
        trades.into_iter()
            .map(|(t, p)| Ok(self.get_mut(p).ok_or(())?.set_trade(t)))
            .collect::<Result<(), ()>>()
    }

    /// Total number of corners. We subtract 2 because the first and last row have 1 less corner than the other rows.
    fn get_size(&self) -> usize {
        (self.length * self.width - 2) as usize
    }
}