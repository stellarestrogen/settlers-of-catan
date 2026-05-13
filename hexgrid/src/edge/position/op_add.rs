use std::marker::PhantomData;
use std::ops::Add;

use crate::hex::position::HexPosition;

use super::{EdgeOrientation, even::Even, negative::Negative, odd::Odd, positive::Positive};

macro_rules! edge_add {
    ($lhs: ty, $rhs: ty, $out: ty) => {
        impl Add<EdgeOrientation<$rhs>> for EdgeOrientation<$lhs> {
            type Output = EdgeOrientation<$out>;

            fn add(self, rhs: EdgeOrientation<$rhs>) -> Self::Output {
                EdgeOrientation::<$out> {
                    rights: self.rights + rhs.rights,
                    downs: self.downs + rhs.downs,
                    r#type: PhantomData::<$out>,
                }
            }
        }
    };
}

macro_rules! edge_to_hex {
    ($lhs: ty, $rhs: ty) => {
        impl Add<EdgeOrientation<$rhs>> for EdgeOrientation<$lhs> {
            type Output = HexPosition;

            fn add(self, rhs: EdgeOrientation<$rhs>) -> Self::Output {
                let rights = self.rights + rhs.rights;
                let downs = self.downs + rhs.downs;

                if rights.signum() == downs.signum() {
                    HexPosition::DOWN_RIGHT * ((downs - 1) / 2)
                        + HexPosition::RIGHT * ((rights - downs) / 4)
                } else {
                    HexPosition::DOWN_LEFT * ((downs - 1) / 2)
                        + HexPosition::RIGHT * ((rights.abs() - downs.abs()) / 4)
                }
            }
        }
    };
}

macro_rules! hex_to_edge {
    ($t: ty) => {
        impl Add<HexPosition> for EdgeOrientation<$t> {
            type Output = EdgeOrientation<$t>;

            fn add(self, rhs: HexPosition) -> Self::Output {
                let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();

                EdgeOrientation::<$t> {
                    rights: self.rights + (shift * 4.) as i32,
                    downs: self.downs + rhs.vertical_distance(HexPosition::ORIGIN) * 2,
                    r#type: PhantomData::<$t>,
                }
            }
        }

        impl Add<EdgeOrientation<$t>> for HexPosition {
            type Output = EdgeOrientation<$t>;

            fn add(self, rhs: EdgeOrientation<$t>) -> Self::Output {
                let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();

                EdgeOrientation::<$t> {
                    rights: rhs.rights + (shift * 4.) as i32,
                    downs: rhs.downs + self.vertical_distance(HexPosition::ORIGIN) * 2,
                    r#type: PhantomData::<$t>,
                }
            }
        }
    };
}

edge_add!(Even, Even, Even);
edge_add!(Even, Odd, Odd);
edge_add!(Even, Positive, Positive);

edge_add!(Odd, Even, Odd);
edge_add!(Odd, Odd, Even);
edge_add!(Odd, Negative, Positive);

edge_add!(Positive, Even, Positive);
edge_add!(Positive, Positive, Even);
edge_add!(Positive, Negative, Odd);

edge_add!(Negative, Odd, Positive);
edge_add!(Negative, Positive, Odd);
edge_add!(Negative, Negative, Even);

edge_to_hex!(Even, Negative);
edge_to_hex!(Negative, Even);
edge_to_hex!(Odd, Positive);
edge_to_hex!(Positive, Odd);

hex_to_edge!(Even);
hex_to_edge!(Odd);
hex_to_edge!(Positive);
