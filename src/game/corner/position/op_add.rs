use std::ops::{Add, AddAssign};

use super::CornerPosition;

impl Add for CornerPosition {
    type Output = CornerPosition;

    fn add(self, rhs: Self) -> Self::Output {
        CornerPosition {
            rights: self.rights + rhs.rights,
            downs: self.downs + rhs.downs
        }
    }
}

impl AddAssign for CornerPosition {
    fn add_assign(&mut self, rhs: Self) {
        self.rights += rhs.rights;
        self.downs += rhs.downs;
    }
}