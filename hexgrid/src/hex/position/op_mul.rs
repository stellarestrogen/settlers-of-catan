use std::ops::{Mul, MulAssign};

use super::HexPosition;

macro_rules! scalar_operations {
    ($scalar: ty) => {
        impl Mul<$scalar> for HexPosition {
            type Output = HexPosition;

            fn mul(self, rhs: $scalar) -> HexPosition {
                HexPosition {
                    rights: self
                        .horizontal_distance(HexPosition::ORIGIN)
                        .mul(rhs as isize)
                        .ceil(),

                    downs: self.vertical_distance(HexPosition::ORIGIN).mul(rhs as i32),
                }
            }
        }

        impl MulAssign<$scalar> for HexPosition {
            fn mul_assign(&mut self, rhs: $scalar) {
                self.rights = self
                    .horizontal_distance(HexPosition::ORIGIN)
                    .mul(rhs as isize)
                    .ceil();

                self.downs = self.vertical_distance(HexPosition::ORIGIN).mul(rhs as i32);
            }
        }

        impl Mul<HexPosition> for $scalar {
            type Output = HexPosition;

            fn mul(self, rhs: HexPosition) -> HexPosition {
                HexPosition {
                    rights: rhs
                        .horizontal_distance(HexPosition::ORIGIN)
                        .mul(self as isize)
                        .ceil(),

                    downs: rhs.vertical_distance(HexPosition::ORIGIN).mul(self as i32),
                }
            }
        }
    };
}

scalar_operations!(isize);
scalar_operations!(usize);
scalar_operations!(i128);
scalar_operations!(u128);
scalar_operations!(i64);
scalar_operations!(u64);
scalar_operations!(i32);
scalar_operations!(u32);
scalar_operations!(i16);
scalar_operations!(u16);
scalar_operations!(i8);
scalar_operations!(u8);
