use std::marker::PhantomData;
use std::ops::{Mul, MulAssign};

use super::{CornerPosition, Low};

macro_rules! scalar_operations {
    ($scalar: ty) => {
        impl Mul<$scalar> for CornerPosition<Low> {
            type Output = CornerPosition<Low>;

            fn mul(self, rhs: $scalar) -> CornerPosition<Low> {
                CornerPosition::<Low> {
                    rights: self.rights * rhs as i32,
                    downs: self.downs * rhs as i32,
                    height: PhantomData::<Low>,
                }
            }
        }

        impl MulAssign<$scalar> for CornerPosition<Low> {
            fn mul_assign(&mut self, rhs: $scalar) {
                self.rights *= rhs as i32;
                self.downs *= rhs as i32;
            }
        }

        impl Mul<CornerPosition<Low>> for $scalar {
            type Output = CornerPosition<Low>;

            fn mul(self, rhs: CornerPosition<Low>) -> CornerPosition<Low> {
                CornerPosition::<Low> {
                    rights: self as i32 * rhs.rights,
                    downs: self as i32 * rhs.downs,
                    height: PhantomData::<Low>,
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
