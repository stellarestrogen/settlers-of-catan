use std::marker::PhantomData;

use crate::{corner::Corner, hex::position::HexPosition};

pub mod op_add;
pub mod op_mul;
pub mod op_sub;

#[derive(Debug)]
pub struct Low;

#[derive(Debug)]
pub struct High;

#[derive(Debug)]
pub struct Center;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerPosition {
    High(CornerHeight<High>),
    Low(CornerHeight<Low>),
}

impl CornerPosition {
    pub fn position(&self) -> &dyn Corner {
        match self {
            Self::High(p) => p,
            Self::Low(p) => p,
        }
    }

    pub fn horizontal_distance(&self, other: Self) -> i32 {
        self.rights() - other.rights()
    }

    pub fn vertical_distance(&self, other: Self) -> i32 {
        self.downs() - other.downs()
    }

    pub fn neighboring_hex(&self) -> [HexPosition; 3] {
        self.position().neighboring_hex()
    }

    pub fn neighboring_corners(&self) -> [CornerPosition; 3] {
        let p: Vec<CornerPosition> = match self {
            Self::High(p) => p
                .neighboring_corners()
                .into_iter()
                .map(Into::<CornerPosition>::into)
                .collect(),
            Self::Low(p) => p
                .neighboring_corners()
                .into_iter()
                .map(Into::<CornerPosition>::into)
                .collect(),
        };

        p.try_into()
            .expect("Neighboring Corners is the incorrect size!")
    }

    fn rights(&self) -> i32 {
        match self {
            Self::High(p) => p.rights,
            Self::Low(p) => p.rights,
        }
    }

    fn downs(&self) -> i32 {
        match self {
            Self::High(p) => p.downs,
            Self::Low(p) => p.downs,
        }
    }
}

#[derive(Debug)]
pub struct CornerHeight<H> {
    rights: i32,
    downs: i32,
    height: PhantomData<H>,
}

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

impl CornerHeight<Center> {
    pub const DOWN_LEFT: CornerHeight<Center> = CornerHeight {
        rights: -1,
        downs: 1,
        height: PhantomData::<Center>,
    };

    pub const DOWN_RIGHT: CornerHeight<Center> = CornerHeight {
        rights: 1,
        downs: 1,
        height: PhantomData::<Center>,
    };

    pub const UP: CornerHeight<Center> = CornerHeight {
        rights: 0,
        downs: -2,
        height: PhantomData::<Center>,
    };
}

impl<H> Clone for CornerHeight<H> {
    fn clone(&self) -> Self {
        CornerHeight::<H> {
            rights: self.rights,
            downs: self.downs,
            height: PhantomData::<H>,
        }
    }
}

impl<H> Copy for CornerHeight<H> {}

impl<H> PartialEq for CornerHeight<H> {
    fn eq(&self, other: &Self) -> bool {
        self.rights == other.rights && self.downs == other.downs && self.height == other.height
    }
}

impl<H> Eq for CornerHeight<H> {}
