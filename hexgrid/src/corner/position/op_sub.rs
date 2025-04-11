use std::{marker::PhantomData, ops::Sub};

use crate::hex::position::HexPosition;

use super::{CornerPosition, High, Low};

impl Sub for CornerPosition<Low> {
    type Output = CornerPosition<High>;

    fn sub(self, rhs: Self) -> Self::Output {
        CornerPosition::<High> {
            rights: self.rights - rhs.rights,
            downs: self.downs - rhs.downs,
            height: PhantomData::<High>
        }
    }
}

impl Sub for CornerPosition<High> {
    type Output = CornerPosition<Low>;

    fn sub(self, rhs: Self) -> Self::Output {
        CornerPosition::<Low> {
            rights: self.rights - rhs.rights,
            downs: self.downs - rhs.downs,
            height: PhantomData::<Low>
        }
    }
}

impl Sub<CornerPosition<High>> for CornerPosition<Low> {
    type Output = HexPosition;

    fn sub(self, rhs: CornerPosition<High>) -> Self::Output {
        let rights = self.rights - rhs.rights;
        let downs = self.downs - rhs.downs;

        if rights.signum() == downs.signum() {
            (HexPosition::RIGHT + HexPosition::DOWN_RIGHT) * ((downs - 1)/3) + HexPosition::RIGHT * ((rights - downs)/2)
        } else {
            (HexPosition::LEFT + HexPosition::DOWN_LEFT) * ((downs - 1)/3) + HexPosition::RIGHT * ((rights.abs() - downs.abs())/2) + HexPosition::UP_RIGHT
        }
    }
}

impl Sub<CornerPosition<Low>> for CornerPosition<High> {
    type Output = HexPosition;

    fn sub(self, rhs: CornerPosition<Low>) -> Self::Output {
        let rights = self.rights - rhs.rights;
        let downs = self.downs - rhs.downs;

        if rights.signum() == downs.signum() {
            (HexPosition::RIGHT + HexPosition::DOWN_RIGHT) * ((downs - 1)/3) + HexPosition::RIGHT * ((rights - downs)/2)
        } else {
            (HexPosition::LEFT + HexPosition::DOWN_LEFT) * ((downs - 1)/3) + HexPosition::RIGHT * ((rights.abs() - downs.abs())/2) + HexPosition::UP_RIGHT
        }
    }
}