use std::{marker::PhantomData, ops::Sub};

use crate::hex::position::HexPosition;

use super::{CornerHeight, center::Center, high::High, low::Low};

macro_rules! corner_sub {
    ($lhs: ty, $rhs: ty, $out: ty) => {
        impl Sub<CornerHeight<$rhs>> for CornerHeight<$lhs> {
            type Output = CornerHeight<$out>;

            fn sub(self, rhs: CornerHeight<$rhs>) -> Self::Output {
                CornerHeight::<$out> {
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
        impl Sub<CornerHeight<$rhs>> for CornerHeight<$lhs> {
            type Output = HexPosition;

            fn sub(self, rhs: CornerHeight<$rhs>) -> Self::Output {
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

impl Sub<HexPosition> for CornerHeight<High> {
    type Output = CornerHeight<High>;

    fn sub(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerHeight {
            rights: self.rights - (shift * 2.) as i32,
            downs: self.downs - rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<High>,
        }
    }
}

impl Sub<HexPosition> for CornerHeight<Low> {
    type Output = CornerHeight<Low>;

    fn sub(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerHeight {
            rights: self.rights - (shift * 2.) as i32,
            downs: self.downs - rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<Low>,
        }
    }
}

impl Sub<CornerHeight<Low>> for HexPosition {
    type Output = CornerHeight<High>;

    fn sub(self, rhs: CornerHeight<Low>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerHeight {
            rights: rhs.rights - (shift * 2.) as i32,
            downs: rhs.downs - self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<High>,
        }
    }
}

impl Sub<CornerHeight<High>> for HexPosition {
    type Output = CornerHeight<Low>;

    fn sub(self, rhs: CornerHeight<High>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerHeight {
            rights: rhs.rights - (shift * 2.) as i32,
            downs: rhs.downs - self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<Low>,
        }
    }
}
