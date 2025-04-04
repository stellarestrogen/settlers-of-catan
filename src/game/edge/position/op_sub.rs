use std::ops::Sub;

use super::EdgePosition;


impl Sub for EdgePosition {
    type Output = Option<EdgePosition>;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = EdgePosition {
            rights: self.rights - rhs.rights,
            downs: self.downs - rhs.downs
        };

        result.is_valid().then_some(result)
    }  
}