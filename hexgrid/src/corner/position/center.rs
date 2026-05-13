use std::marker::PhantomData;

use crate::corner::position::CornerHeight;

#[derive(Debug)]
pub struct Center;

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