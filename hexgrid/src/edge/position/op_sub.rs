use std::marker::PhantomData;
use std::ops::Sub;

use crate::hex::position::HexPosition;

use super::{EdgeOrientation, Even, Negative, Odd, Positive};

macro_rules! edge_sub {
    ($lhs: ty, $rhs: ty, $out: ty) => {
        impl Sub<EdgeOrientation<$rhs>> for EdgeOrientation<$lhs> {
            type Output = EdgeOrientation<$out>;

            fn sub(self, rhs: EdgeOrientation<$rhs>) -> Self::Output {
                EdgeOrientation::<$out> {
                    rights: self.rights - rhs.rights,
                    downs: self.downs - rhs.downs,
                    r#type: PhantomData::<$out>,
                }
            }
        }
    };
}

macro_rules! edge_to_hex {
    ($lhs: ty, $rhs: ty) => {
        impl Sub<EdgeOrientation<$rhs>> for EdgeOrientation<$lhs> {
            type Output = HexPosition;

            fn sub(self, rhs: EdgeOrientation<$rhs>) -> Self::Output {
                let rights = self.rights - rhs.rights;
                let downs = self.downs - rhs.downs;

                if rights.signum() == downs.signum() {
                    HexPosition::DOWN_RIGHT * ((downs - 1) / 2)
                        + HexPosition::RIGHT * ((rights - downs) / 4)
                } else {
                    HexPosition::DOWN_LEFT * ((downs - 1) / 2)
                        + HexPosition::RIGHT * ((rights - downs) / 4)
                }
            }
        }
    };
}

macro_rules! hex_to_edge {
    ($t: ty) => {
        impl Sub<HexPosition> for EdgeOrientation<$t> {
            type Output = EdgeOrientation<$t>;

            fn sub(self, rhs: HexPosition) -> Self::Output {
                let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();

                EdgeOrientation::<$t> {
                    rights: self.rights + (shift * 4.) as i32,
                    downs: self.downs + rhs.vertical_distance(HexPosition::ORIGIN) * 2,
                    r#type: PhantomData::<$t>,
                }
            }
        }

        impl Sub<EdgeOrientation<$t>> for HexPosition {
            type Output = EdgeOrientation<$t>;

            fn sub(self, rhs: EdgeOrientation<$t>) -> Self::Output {
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

hex_to_edge!(Even);
hex_to_edge!(Odd);
hex_to_edge!(Positive);
