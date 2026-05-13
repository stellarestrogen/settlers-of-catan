use std::marker::PhantomData;

use crate::{
    edge::{
        Edge,
        position::{EdgeOrientation, EdgePosition, odd::Odd, positive::Positive},
    },
    hex::position::HexPosition,
};

#[derive(Debug, Clone, Copy)]
pub struct Even;

impl EdgeOrientation<Even> {
    pub const TOP_LEFT: EdgeOrientation<Even> = EdgeOrientation {
        rights: 0,
        downs: 0,
        r#type: PhantomData::<Even>,
    };

    pub const BOTTOM_RIGHT: EdgeOrientation<Even> = EdgeOrientation {
        rights: 2,
        downs: 2,
        r#type: PhantomData::<Even>,
    };

    pub fn go_up_right(self) -> EdgeOrientation<Positive> {
        self + EdgeOrientation::UP_RIGHT
    }

    pub fn go_right(self) -> EdgeOrientation<Odd> {
        self + EdgeOrientation::GO_RIGHT
    }

    pub fn go_down_left(self) -> EdgeOrientation<Positive> {
        self + EdgeOrientation::DOWN_LEFT
    }

    pub fn go_left(self) -> EdgeOrientation<Odd> {
        self + EdgeOrientation::GO_LEFT
    }

    pub fn neighboring_edges(&self) -> [(EdgeOrientation<Odd>, EdgeOrientation<Positive>); 2] {
        [
            (
                *self + EdgeOrientation::GO_LEFT,
                *self + EdgeOrientation::UP_RIGHT,
            ),
            (
                *self + EdgeOrientation::GO_RIGHT,
                *self + EdgeOrientation::DOWN_LEFT,
            ),
        ]
    }
}

impl Into<EdgePosition> for EdgeOrientation<Even> {
    fn into(self) -> EdgePosition {
        EdgePosition::Even(self)
    }
}

impl Edge for EdgeOrientation<Even> {
    fn neighboring_hex(&self) -> [HexPosition; 2] {
        [
            *self + EdgeOrientation::UP_LEFT,
            *self + EdgeOrientation::DOWN_RIGHT,
        ]
    }
}
