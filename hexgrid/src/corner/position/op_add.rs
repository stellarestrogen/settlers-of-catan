use std::{marker::PhantomData, ops::Add};

use crate::hex::position::HexPosition;

use super::{Center, CornerPosition, High, Low};

macro_rules! corner_add {
    ($lhs: ty, $rhs: ty, $out: ty) => {
        impl Add<CornerPosition<$rhs>> for CornerPosition<$lhs> {
            type Output = CornerPosition<$out>;

            fn add(self, rhs: CornerPosition<$rhs>) -> Self::Output {
                CornerPosition::<$out> {
                    rights: self.rights + rhs.rights,
                    downs: self.downs + rhs.downs,
                    height: PhantomData::<$out>,
                }
            }
        }
    };
}

macro_rules! corner_to_hex {
    ($lhs: ty, $rhs: ty) => {
        impl Add<CornerPosition<$rhs>> for CornerPosition<$lhs> {
            type Output = HexPosition;

            fn add(self, rhs: CornerPosition<$rhs>) -> Self::Output {
                let rights = self.rights + rhs.rights;
                let downs = self.downs + rhs.downs;

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

corner_add!(Low, High, High);
corner_add!(High, Low, High);
corner_add!(Low, Low, Low);
corner_add!(Center, High, Low);
corner_add!(High, Center, Low);

corner_to_hex!(High, High);
corner_to_hex!(Low, Center);
corner_to_hex!(Center, Low);

impl Add<HexPosition> for CornerPosition<High> {
    type Output = CornerPosition<High>;

    fn add(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: self.rights + (shift * 2.) as i32,
            downs: self.downs + rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<High>,
        }
    }
}

impl Add<HexPosition> for CornerPosition<Low> {
    type Output = CornerPosition<Low>;

    fn add(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: self.rights + (shift * 2.) as i32,
            downs: self.downs + rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<Low>,
        }
    }
}

impl Add<CornerPosition<High>> for HexPosition {
    type Output = CornerPosition<High>;

    fn add(self, rhs: CornerPosition<High>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: rhs.rights + (shift * 2.) as i32,
            downs: rhs.downs + self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<High>,
        }
    }
}

impl Add<CornerPosition<Low>> for HexPosition {
    type Output = CornerPosition<Low>;

    fn add(self, rhs: CornerPosition<Low>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: rhs.rights + (shift * 2.) as i32,
            downs: rhs.downs + self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<Low>,
        }
    }
}
