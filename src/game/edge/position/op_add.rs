use std::ops::Add;

use super::EdgePosition;


impl Add for EdgePosition {
    type Output = Option<EdgePosition>;

    fn add(self, rhs: Self) -> Self::Output {
        let result = EdgePosition {
            rights: self.rights + rhs.rights,
            downs: self.downs + rhs.downs
        };

        result.is_valid().then_some(result)
    }
}