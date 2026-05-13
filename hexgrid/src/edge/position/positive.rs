use std::marker::PhantomData;

use crate::{
    edge::{
        Edge,
        position::{EdgeOrientation, EdgePosition, even::Even, odd::Odd},
    },
    hex::position::HexPosition,
};

#[derive(Debug, Clone, Copy)]
pub struct Positive;

impl EdgeOrientation<Positive> {
    pub const DOWN_LEFT: EdgeOrientation<Positive> = EdgeOrientation {
        rights: -1,
        downs: 1,
        r#type: PhantomData::<Positive>,
    };

    pub const UP_RIGHT: EdgeOrientation<Positive> = EdgeOrientation {
        rights: 1,
        downs: -1,
        r#type: PhantomData::<Positive>,
    };

    pub const LEFT: EdgeOrientation<Positive> = EdgeOrientation {
        rights: -1,
        downs: 1,
        r#type: PhantomData::<Positive>,
    };

    pub const RIGHT: EdgeOrientation<Positive> = EdgeOrientation {
        rights: 3,
        downs: 1,
        r#type: PhantomData::<Positive>,
    };

    pub fn go_up_right(self) -> EdgeOrientation<Even> {
        self + EdgeOrientation::UP_RIGHT
    }

    pub fn go_down_right(self) -> EdgeOrientation<Odd> {
        self + EdgeOrientation::DOWN_RIGHT
    }

    pub fn go_up_left(self) -> EdgeOrientation<Odd> {
        self + EdgeOrientation::UP_LEFT
    }

    pub fn go_down_left(self) -> EdgeOrientation<Even> {
        self + EdgeOrientation::DOWN_LEFT
    }

    pub fn neighboring_edges(&self) -> [(EdgeOrientation<Even>, EdgeOrientation<Odd>); 2] {
        [
            (
                *self + EdgeOrientation::UP_RIGHT,
                *self + EdgeOrientation::UP_LEFT,
            ),
            (
                *self + EdgeOrientation::DOWN_LEFT,
                *self + EdgeOrientation::DOWN_RIGHT,
            ),
        ]
    }
}

impl Into<EdgePosition> for EdgeOrientation<Positive> {
    fn into(self) -> EdgePosition {
        EdgePosition::Positive(self)
    }
}

impl Edge for EdgeOrientation<Positive> {
    fn neighboring_hex(&self) -> [HexPosition; 2] {
        [
            *self + EdgeOrientation::GO_LEFT,
            *self + EdgeOrientation::GO_RIGHT,
        ]
    }
}
