use std::ops::{Sub, SubAssign};

use super::CornerPosition;


impl Sub for CornerPosition {
    type Output = CornerPosition;

    fn sub(self, rhs: Self) -> Self::Output {
        CornerPosition {
            rights: self.rights - rhs.rights,
            downs: self.downs - rhs.downs
        }
    }
}

impl SubAssign for CornerPosition {
    fn sub_assign(&mut self, rhs: Self) {
        self.rights -= rhs.rights;
        self.downs -= rhs.downs;
    }
}