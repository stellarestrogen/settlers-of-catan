use std::marker::PhantomData;

use crate::{
    edge::{
        Edge,
        position::{EdgeOrientation, EdgePosition, even::Even, positive::Positive},
    },
    hex::position::HexPosition,
};

#[derive(Debug, Clone, Copy)]
pub struct Odd;

impl EdgeOrientation<Odd> {
    pub const GO_RIGHT: EdgeOrientation<Odd> = EdgeOrientation {
        rights: 2,
        downs: 0,
        r#type: PhantomData::<Odd>,
    };

    pub const GO_LEFT: EdgeOrientation<Odd> = EdgeOrientation {
        rights: -2,
        downs: 0,
        r#type: PhantomData::<Odd>,
    };

    pub const BOTTOM_LEFT: EdgeOrientation<Odd> = EdgeOrientation {
        rights: -1,
        downs: 1,
        r#type: PhantomData::<Odd>,
    };

    pub const TOP_RIGHT: EdgeOrientation<Odd> = EdgeOrientation {
        rights: 1,
        downs: -1,
        r#type: PhantomData::<Odd>,
    };

    pub fn go_right(self) -> EdgeOrientation<Even> {
        self + EdgeOrientation::GO_RIGHT
    }

    pub fn go_down_right(self) -> EdgeOrientation<Positive> {
        self + EdgeOrientation::DOWN_RIGHT
    }

    pub fn go_left(self) -> EdgeOrientation<Even> {
        self + EdgeOrientation::GO_LEFT
    }

    pub fn go_up_left(self) -> EdgeOrientation<Positive> {
        self + EdgeOrientation::UP_LEFT
    }

    pub fn neighboring_edges(&self) -> [(EdgeOrientation<Even>, EdgeOrientation<Positive>); 2] {
        [
            (
                *self + EdgeOrientation::GO_LEFT,
                *self + EdgeOrientation::UP_LEFT,
            ),
            (
                *self + EdgeOrientation::GO_RIGHT,
                *self + EdgeOrientation::DOWN_RIGHT,
            ),
        ]
    }
}

impl Into<EdgePosition> for EdgeOrientation<Odd> {
    fn into(self) -> EdgePosition {
        EdgePosition::Odd(self)
    }
}

impl Edge for EdgeOrientation<Odd> {
    fn neighboring_hex(&self) -> [HexPosition; 2] {
        [
            *self + EdgeOrientation::UP_RIGHT,
            *self + EdgeOrientation::DOWN_LEFT,
        ]
    }
}
