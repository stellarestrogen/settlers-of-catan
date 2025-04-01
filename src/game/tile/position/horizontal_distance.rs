use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug)]
pub enum HorizontalDistance {
    Unshifted(i32),

    // Represents an implicit -0.5 to the stored i32.
    Shifted(i32),
}

impl HorizontalDistance {
    pub fn ceil(self) -> i32 {
        match self {
            Self::Unshifted(a) => a,
            Self::Shifted(a) => a,
        }
    }

    pub fn abs(self) -> Self {
        match self {
            Self::Unshifted(a) => Self::Unshifted(a.abs()),
            Self::Shifted(a @ ..=0) => Self::Shifted(-a + 1),
            Self::Shifted(a @ 1..) => Self::Shifted(a),
        }
    }
}

impl Into<f64> for HorizontalDistance {
    fn into(self) -> f64 {
        match self {
            Self::Shifted(a) => a as f64 - 0.5,
            Self::Unshifted(a) => a as f64,
        }
    }
}

impl Add for HorizontalDistance {
    type Output = HorizontalDistance;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Unshifted(a), Self::Unshifted(b)) => Self::Unshifted(a + b),
            (Self::Shifted(a), Self::Shifted(b)) => Self::Unshifted((a + b) - 1),
            (Self::Unshifted(a), Self::Shifted(b)) | (Self::Shifted(a), Self::Unshifted(b)) => {
                Self::Shifted(a + b)
            }
        }
    }
}

impl Sub for HorizontalDistance {
    type Output = HorizontalDistance;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Unshifted(a), Self::Unshifted(b)) => Self::Unshifted(a - b),
            (Self::Shifted(a), Self::Unshifted(b)) => Self::Shifted(a - b),
            (Self::Unshifted(a), Self::Shifted(b)) => Self::Shifted((a - b) + 1),
            (Self::Shifted(a), Self::Shifted(b)) => Self::Unshifted(a - b),
        }
    }
}

macro_rules! scalar_operations {
    ($scalar: ty) => {
        impl Add<HorizontalDistance> for $scalar {
            type Output = HorizontalDistance;

            fn add(self, rhs: HorizontalDistance) -> Self::Output {
                match rhs {
                    HorizontalDistance::Unshifted(a) => {
                        HorizontalDistance::Unshifted(a + self as i32)
                    }
                    HorizontalDistance::Shifted(a) => HorizontalDistance::Shifted(a + self as i32),
                }
            }
        }

        impl Add<$scalar> for HorizontalDistance {
            type Output = HorizontalDistance;

            fn add(self, rhs: $scalar) -> Self::Output {
                match self {
                    Self::Unshifted(a) => HorizontalDistance::Unshifted(a + rhs as i32),
                    Self::Shifted(a) => HorizontalDistance::Shifted(a + rhs as i32),
                }
            }
        }

        impl AddAssign<$scalar> for HorizontalDistance {
            fn add_assign(&mut self, rhs: $scalar) {
                match self {
                    Self::Unshifted(a) => *a += rhs as i32,
                    Self::Shifted(a) => *a += rhs as i32,
                }
            }
        }
    };
}

impl Mul<isize> for HorizontalDistance {
    type Output = HorizontalDistance;

    fn mul(self, rhs: isize) -> Self::Output {
        match self {
            Self::Unshifted(a) => Self::Unshifted(a * rhs as i32),
            Self::Shifted(a) if rhs % 2 == 0 => {
                Self::Unshifted((a * rhs as i32) - (rhs / 2) as i32)
            }
            Self::Shifted(a) => Self::Shifted((a * rhs as i32) - (rhs / 2) as i32 + 1),
        }
    }
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
