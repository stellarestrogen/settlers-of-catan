use std::marker::PhantomData;
use std::ops::Sub;

use crate::{hex::position::HexPosition, position::Position};

use super::{EdgePosition, Even, Negative, Odd, Positive, Valid};

macro_rules! edge_sub {
    ($lhs: ty, $rhs: ty, $out: ty) => {
        impl Sub<EdgePosition<$rhs>> for EdgePosition<$lhs> {
            type Output = EdgePosition<$out>;

            fn sub(self, rhs: EdgePosition<$rhs>) -> Self::Output {
                EdgePosition::<$out> {
                    rights: self.rights - rhs.rights,
                    downs: self.downs - rhs.downs,
                    r#type: PhantomData::<$out>
                }
            }
        }
    }
}

macro_rules! edge_to_hex {
    ($lhs: ty, $rhs: ty) => {
        impl Sub<EdgePosition<$rhs>> for EdgePosition<$lhs> {
            type Output = HexPosition;

            fn sub(self, rhs: EdgePosition<$rhs>) -> Self::Output {
                let rights = self.rights - rhs.rights;
                let downs = self.downs - rhs.downs;

                if rights.signum() == downs.signum() {
                    HexPosition::DOWN_RIGHT * ((downs - 1)/2) + HexPosition::RIGHT * ((rights - downs)/4)
                } else {
                    HexPosition::DOWN_LEFT * ((downs - 1)/2) + HexPosition::RIGHT * ((rights - downs)/4)
                }
            }
        }
    }
}

impl<Type: Valid> Sub<HexPosition> for EdgePosition<Type> {
    type Output = EdgePosition<Type>;

    fn sub(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();

        EdgePosition::<Type> {
            rights: self.rights + (shift * 4.) as i32,
            downs: self.downs + rhs.vertical_distance(HexPosition::ORIGIN) * 2,
            r#type: PhantomData::<Type>
        }
    }
}

impl<Type: Valid> Sub<EdgePosition<Type>> for HexPosition {
    type Output = EdgePosition<Type>;

    fn sub(self, rhs: EdgePosition<Type>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();

        EdgePosition::<Type> {
            rights: rhs.rights + (shift * 4.) as i32,
            downs: rhs.downs + self.vertical_distance(HexPosition::ORIGIN) * 2,
            r#type: PhantomData::<Type>
        }
    }
}

edge_sub!(Even, Even, Even);
edge_sub!(Even, Odd, Odd);
edge_sub!(Even, Positive, Positive);

edge_sub!(Odd, Even, Odd);
edge_sub!(Odd, Odd, Even);
edge_sub!(Odd, Negative, Positive);

edge_sub!(Positive, Even, Positive);
edge_sub!(Positive, Positive, Even);
edge_sub!(Positive, Negative, Odd);

edge_sub!(Negative, Odd, Positive);
edge_sub!(Negative, Positive, Odd);
edge_sub!(Negative, Negative, Even);

edge_to_hex!(Even, Negative);
edge_to_hex!(Negative, Even);
edge_to_hex!(Odd, Positive);
edge_to_hex!(Positive, Odd);