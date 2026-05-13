use std::marker::PhantomData;

use crate::edge::position::EdgeOrientation;

#[derive(Debug, Clone, Copy)]
pub struct Negative;

impl EdgeOrientation<Negative> {
    pub const DOWN_RIGHT: EdgeOrientation<Negative> = EdgeOrientation {
        rights: 1,
        downs: 1,
        r#type: PhantomData::<Negative>,
    };

    pub const UP_LEFT: EdgeOrientation<Negative> = EdgeOrientation {
        rights: -1,
        downs: -1,
        r#type: PhantomData::<Negative>,
    };
}
