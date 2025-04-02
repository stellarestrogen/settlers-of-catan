use std::ops::{Mul, MulAssign};

use super::CornerPosition;


macro_rules! scalar_operations {
    ($scalar: ty) => {
        impl Mul<$scalar> for CornerPosition {
            type Output = CornerPosition;

            fn mul(self, rhs: $scalar) -> CornerPosition {
                CornerPosition {
                    rights: self.rights * rhs as i32,
                    downs: self.downs * rhs as i32
                }
            }
        }

        impl MulAssign<$scalar> for CornerPosition {
            fn mul_assign(&mut self, rhs: $scalar) {
                self.rights *= rhs as i32;
                self.downs *= rhs as i32;
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
