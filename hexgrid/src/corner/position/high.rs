use std::marker::PhantomData;

use crate::{
    corner::{
        Corner,
        position::{CornerHeight, CornerPosition, low::Low},
    },
    hex::position::HexPosition,
};

#[derive(Debug)]
pub struct High;

impl CornerHeight<High> {
    pub const UP_LEFT: CornerHeight<High> = CornerHeight {
        rights: -1,
        downs: -1,
        height: PhantomData::<High>,
    };

    pub const UP_RIGHT: CornerHeight<High> = CornerHeight {
        rights: 1,
        downs: -1,
        height: PhantomData::<High>,
    };

    pub const DOWN: CornerHeight<High> = CornerHeight {
        rights: 0,
        downs: 2,
        height: PhantomData::<High>,
    };

    pub const BOTTOM_LEFT: CornerHeight<High> = CornerHeight {
        rights: 0,
        downs: 2,
        height: PhantomData::<High>,
    };

    pub const BOTTOM_RIGHT: CornerHeight<High> = CornerHeight {
        rights: 2,
        downs: 2,
        height: PhantomData::<High>,
    };

    pub const TOP: CornerHeight<High> = CornerHeight {
        rights: 1,
        downs: -1,
        height: PhantomData::<High>,
    };

    pub fn go_right(self) -> CornerHeight<Low> {
        self + CornerHeight::DOWN_RIGHT
    }

    pub fn go_left(self) -> CornerHeight<Low> {
        self + CornerHeight::DOWN_LEFT
    }

    pub fn go_up(self) -> CornerHeight<Low> {
        self + CornerHeight::UP
    }

    pub fn neighboring_corners(&self) -> [CornerHeight<Low>; 3] {
        [
            *self + CornerHeight::UP,
            *self + CornerHeight::DOWN_LEFT,
            *self + CornerHeight::DOWN_RIGHT,
        ]
    }
}

impl Into<CornerPosition> for CornerHeight<High> {
    fn into(self) -> CornerPosition {
        CornerPosition::High(self)
    }
}

impl Corner for CornerHeight<High> {
    fn neighboring_hex(&self) -> [HexPosition; 3] {
        [
            *self + CornerHeight::UP_LEFT,
            *self + CornerHeight::UP_RIGHT,
            *self + CornerHeight::DOWN,
        ]
    }

    fn neighboring_corners(&self) -> Vec<Box<dyn Corner>> {
        vec![
            Box::new(*self + CornerHeight::UP),
            Box::new(*self + CornerHeight::DOWN_LEFT),
            Box::new(*self + CornerHeight::DOWN_RIGHT),
        ]
    }
}
