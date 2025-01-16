use super::objects::*;

#[derive(Clone)]
#[derive(Copy)]
pub struct Corner {
    building: Option<Building>,
    trade_type: Option<TradeType>
}

impl Corner {
    pub fn new() -> Self {
        Corner {
            building: None,
            trade_type: None,
        }
    }

    pub fn set_building(&mut self, building: Option<Building>) {
        self.building = building
    }

    pub fn set_trade(&mut self, trade: Option<TradeType>) {
        self.trade_type = trade
    }
}

pub struct CornerHolder {
    corners: Vec<Corner>,
    row_length: u32,
    row_count: u32,
}

impl CornerHolder {
    pub fn new() -> Self {
        CornerHolder {
            corners: Vec::<Corner>::new(),
            row_length: 0,
            row_count: 0,
        }
    }

    pub fn clear(&mut self) {
        self.corners.clear();
        self.row_length = 0;
        self.row_count = 0;
    }

    pub fn setup(&mut self, length: u32, count: u32) {
        self.row_length = length * 2 + 2;
        self.row_count = count + 1;
        // we subtract 2 here because there is 1 less corner in the first and last rows (same reason as the edges).
        self.corners.reserve((self.row_length * self.row_count - 2) as usize);

        for _ in 0..self.corners.capacity() {
            self.corners.push(Corner::new());
        }
    }

    pub fn set_building(&mut self, idx: usize, building: Option<Building>) {
        self.corners[idx].set_building(building)
    }

    pub fn set_trade(&mut self, idx: usize, trade: Option<TradeType>) {
        self.corners[idx].set_trade(trade)
    }

    pub fn calc_row(&self, idx: u32) -> u32 {
        // off by one due to the missing corner in the first row.
        (idx + 1) / self.row_length
    }

    pub fn calc_row_idx(&self, idx: u32) -> u32 {
        if self.calc_row(idx) == 0 {
            idx
        } else {
            // off by one due to the missing corner in the first row.
            (idx + 1) % self.row_length
        }
    }

    fn can_traverse_up(&self, idx: u32) -> bool {
        let row = self.calc_row(idx);
        if row == 0 {
            return false;
        } else if row == self.row_count - 1 || row % 2 == 0 {
            return self.calc_row_idx(idx) % 2 == 1;
        } else {
            return self.calc_row_idx(idx) % 2 == 0;
        }
    }

    fn can_traverse_down(&self, idx: u32) -> bool {
        let row = self.calc_row(idx);
        if row == self.row_count - 1 {
            return false;
        } else if row % 2 == 0 {
            return self.calc_row_idx(idx) % 2 == 0;
        } else {
            return self.calc_row_idx(idx) % 2 == 1;
        }
    }

    pub fn calc_distance(&self, first: u32, second: u32) -> u32 {
        let (mut current_row, mut current_row_idx, dest_row, dest_row_idx) = if self.calc_row_idx(first) <= self.calc_row_idx(second) {
            (self.calc_row(first), self.calc_row_idx(first), self.calc_row(second), self.calc_row_idx(second))
        } else {
            (self.calc_row(second), self.calc_row_idx(second), self.calc_row(first), self.calc_row_idx(first))
        };
        let mut distance = 0;
        
        // current_row will always be to the left or on top of the destination.
        // we iterate based on where we are on the board until we reach the destination.
        while current_row != dest_row && current_row_idx != dest_row_idx {
            let current_idx = current_row * self.row_length + current_row_idx - if current_row == 0 { 0 } else { 1 };
            let right = current_row_idx > dest_row_idx;
            
            if current_row < dest_row && self.can_traverse_up(current_idx) {
                current_row -= 1;
                distance += 1;
            } else if current_row > dest_row && self.can_traverse_down(current_idx) {
                current_row +=1;
                distance += 1;
            } else {
                if right { current_row_idx -= 1 } else { current_row_idx += 1 };
                distance += 1;
            } 
        }
        
        distance
    }

}