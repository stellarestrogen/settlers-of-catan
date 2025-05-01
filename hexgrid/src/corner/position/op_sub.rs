use std::{marker::PhantomData, ops::Sub};

use crate::hex::position::HexPosition;

use super::{Center, CornerPosition, High, Low};

macro_rules! corner_sub {
    ($lhs: ty, $rhs: ty, $out: ty) => {
        impl Sub<CornerPosition<$rhs>> for CornerPosition<$lhs> {
            type Output = CornerPosition<$out>;

            fn sub(self, rhs: CornerPosition<$rhs>) -> Self::Output {
                CornerPosition::<$out> {
                    rights: self.rights - rhs.rights,
                    downs: self.downs - rhs.downs,
                    height: PhantomData::<$out>,
                }
            }
        }
    };
}

macro_rules! corner_to_hex {
    ($lhs: ty, $rhs: ty) => {
        impl Sub<CornerPosition<$rhs>> for CornerPosition<$lhs> {
            type Output = HexPosition;

            fn sub(self, rhs: CornerPosition<$rhs>) -> Self::Output {
                let rights = self.rights - rhs.rights;
                let downs = self.downs - rhs.downs;

                if rights.signum() == downs.signum() {
                    (HexPosition::RIGHT + HexPosition::DOWN_RIGHT) * ((downs - 1) / 3)
                        + HexPosition::RIGHT * ((rights - downs) / 2)
                } else {
                    (HexPosition::LEFT + HexPosition::DOWN_LEFT) * ((downs - 1) / 3)
                        + HexPosition::RIGHT * ((rights.abs() - downs.abs()) / 2)
                        + HexPosition::UP_RIGHT
                }
            }
        }
    };
}

corner_sub!(High, Low, High);
corner_sub!(Low, Low, Low);
corner_sub!(High, High, Low);
corner_sub!(Low, Center, Low);
corner_sub!(Center, High, Low);

corner_to_hex!(High, Center);
corner_to_hex!(Center, Low);
corner_to_hex!(Low, High);

impl Sub<HexPosition> for CornerPosition<High> {
    type Output = CornerPosition<High>;

    fn sub(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: self.rights - (shift * 2.) as i32,
            downs: self.downs - rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<High>,
        }
    }
}

impl Sub<HexPosition> for CornerPosition<Low> {
    type Output = CornerPosition<Low>;

    fn sub(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: self.rights - (shift * 2.) as i32,
            downs: self.downs - rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<Low>,
        }
    }
}

impl Sub<CornerPosition<Low>> for HexPosition {
    type Output = CornerPosition<High>;

    fn sub(self, rhs: CornerPosition<Low>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: rhs.rights - (shift * 2.) as i32,
            downs: rhs.downs - self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<High>,
        }
    }
}

impl Sub<CornerPosition<High>> for HexPosition {
    type Output = CornerPosition<Low>;

    fn sub(self, rhs: CornerPosition<High>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: rhs.rights - (shift * 2.) as i32,
            downs: rhs.downs - self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<Low>,
        }
    }
}
