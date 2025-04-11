use std::marker::PhantomData;

use std::ops::{Mul, MulAssign};

use super::{EdgePosition, Even};

macro_rules! scalar_operations {
    ($scalar: ty) => {
        impl Mul<$scalar> for EdgePosition<Even> {
            type Output = EdgePosition<Even>;

            fn mul(self, rhs: $scalar) -> Self::Output {
                EdgePosition::<Even> {
                    rights: self.rights * rhs as i32,
                    downs: self.downs * rhs as i32,
                    r#type: PhantomData::<Even>
                }
            }
        }

        impl MulAssign<$scalar> for EdgePosition<Even> {
            fn mul_assign(&mut self, rhs: $scalar) {
                self.rights *= rhs as i32;
                self.downs *= rhs as i32;
            }
        }

        impl Mul<EdgePosition<Even>> for $scalar {
            type Output = EdgePosition<Even>;

            fn mul(self, rhs: EdgePosition<Even>) -> EdgePosition<Even> {
                EdgePosition::<Even> {
                    rights: rhs.rights * self as i32,
                    downs: rhs.downs * self as i32,
                    r#type: PhantomData::<Even>
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