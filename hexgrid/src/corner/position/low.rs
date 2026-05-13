use std::marker::PhantomData;

use crate::{
    corner::{
        Corner,
        position::{CornerHeight, CornerPosition, high::High},
    },
    hex::position::HexPosition,
};

#[derive(Debug)]
pub struct Low;

impl CornerHeight<Low> {
    pub const TOP_LEFT: CornerHeight<Low> = CornerHeight {
        rights: 0,
        downs: 0,
        height: PhantomData::<Low>,
    };

    pub const TOP_RIGHT: CornerHeight<Low> = CornerHeight {
        rights: 2,
        downs: 0,
        height: PhantomData::<Low>,
    };

    pub const BOTTOM: CornerHeight<Low> = CornerHeight {
        rights: 1,
        downs: 3,
        height: PhantomData::<Low>,
    };

    pub fn go_right(self) -> CornerHeight<High> {
        self + CornerHeight::UP_RIGHT
    }

    pub fn go_left(self) -> CornerHeight<High> {
        self + CornerHeight::UP_LEFT
    }

    pub fn go_down(self) -> CornerHeight<High> {
        self + CornerHeight::DOWN
    }

    pub fn neighboring_corners(&self) -> [CornerHeight<High>; 3] {
        [
            *self + CornerHeight::DOWN,
            *self + CornerHeight::UP_LEFT,
            *self + CornerHeight::UP_RIGHT,
        ]
    }
}

impl Into<CornerPosition> for CornerHeight<Low> {
    fn into(self) -> CornerPosition {
        CornerPosition::Low(self)
    }
}

impl Corner for CornerHeight<Low> {
    fn neighboring_hex(&self) -> [HexPosition; 3] {
        [
            *self + CornerHeight::UP,
            *self + CornerHeight::DOWN_LEFT,
            *self + CornerHeight::DOWN_RIGHT,
        ]
    }

    fn neighboring_corners(&self) -> Vec<Box<dyn Corner>> {
        vec![
            Box::new(*self + CornerHeight::DOWN),
            Box::new(*self + CornerHeight::UP_LEFT),
            Box::new(*self + CornerHeight::UP_RIGHT),
        ]
    }
}
