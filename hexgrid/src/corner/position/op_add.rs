use std::{marker::PhantomData, ops::Add};

use crate::hex::position::HexPosition;

use super::{CornerHeight, center::Center, high::High, low::Low};

macro_rules! corner_add {
    ($lhs: ty, $rhs: ty, $out: ty) => {
        impl Add<CornerHeight<$rhs>> for CornerHeight<$lhs> {
            type Output = CornerHeight<$out>;

            fn add(self, rhs: CornerHeight<$rhs>) -> Self::Output {
                CornerHeight::<$out> {
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
        impl Add<CornerHeight<$rhs>> for CornerHeight<$lhs> {
            type Output = HexPosition;

            fn add(self, rhs: CornerHeight<$rhs>) -> Self::Output {
                let rights = self.rights + rhs.rights;
                let downs = self.downs + rhs.downs;

                (HexPosition::RIGHT + HexPosition::DOWN_RIGHT) * ((downs - 1) / 3)
                    + HexPosition::RIGHT * ((rights - downs) / 2)
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

impl Add<HexPosition> for CornerHeight<High> {
    type Output = CornerHeight<High>;

    fn add(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerHeight {
            rights: self.rights + (shift * 2.) as i32,
            downs: self.downs + rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<High>,
        }
    }
}

impl Add<HexPosition> for CornerHeight<Low> {
    type Output = CornerHeight<Low>;

    fn add(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerHeight {
            rights: self.rights + (shift * 2.) as i32,
            downs: self.downs + rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<Low>,
        }
    }
}

impl Add<CornerHeight<High>> for HexPosition {
    type Output = CornerHeight<High>;

    fn add(self, rhs: CornerHeight<High>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerHeight {
            rights: rhs.rights + (shift * 2.) as i32,
            downs: rhs.downs + self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<High>,
        }
    }
}

impl Add<CornerHeight<Low>> for HexPosition {
    type Output = CornerHeight<Low>;

    fn add(self, rhs: CornerHeight<Low>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerHeight {
            rights: rhs.rights + (shift * 2.) as i32,
            downs: rhs.downs + self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<Low>,
        }
    }
}
