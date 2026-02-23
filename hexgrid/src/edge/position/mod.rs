use std::marker::PhantomData;

use crate::{edge::Edge, hex::position::HexPosition};

pub mod op_add;
pub mod op_mul;
pub mod op_sub;

#[derive(Debug, Clone, Copy)]
pub struct Even;

#[derive(Debug, Clone, Copy)]
pub struct Odd;

#[derive(Debug, Clone, Copy)]
pub struct Positive;

#[derive(Debug, Clone, Copy)]
pub struct Negative;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgePosition {
    Even(EdgeOrientation<Even>),
    Odd(EdgeOrientation<Odd>),
    Positive(EdgeOrientation<Positive>),
}

impl EdgePosition {
    pub fn position(&self) -> &dyn Edge {
        match self {
            Self::Even(p) => p,
            Self::Odd(p) => p,
            Self::Positive(p) => p,
        }
    }

    pub fn horizontal_distance(&self, other: Self) -> i32 {
        self.rights() - other.rights()
    }

    pub fn vertical_distance(&self, other: Self) -> i32 {
        self.downs() - other.downs()
    }

    pub fn neighboring_hex(&self) -> [HexPosition; 2] {
        self.position().neighboring_hex()
    }

    pub fn neighboring_edges(&self) -> [EdgePosition; 4] {
        let p: Vec<EdgePosition> = match self {
            Self::Even(p) => p
                .neighboring_edges()
                .into_iter()
                .flat_map(|(a, b)| [a.into(), b.into()])
                .collect(),
            Self::Odd(p) => p
                .neighboring_edges()
                .into_iter()
                .flat_map(|(a, b)| [a.into(), b.into()])
                .collect(),
            Self::Positive(p) => p
                .neighboring_edges()
                .into_iter()
                .flat_map(|(a, b)| [a.into(), b.into()])
                .collect(),
        };

        p.try_into()
            .expect("Neighboring Edges is the incorrect size!")
    }

    fn rights(&self) -> i32 {
        match self {
            Self::Even(p) => p.rights,
            Self::Odd(p) => p.rights,
            Self::Positive(p) => p.rights,
        }
    }

    fn downs(&self) -> i32 {
        match self {
            Self::Even(p) => p.downs,
            Self::Odd(p) => p.downs,
            Self::Positive(p) => p.downs,
        }
    }
}

#[derive(Debug)]
pub struct EdgeOrientation<T> {
    rights: i32,
    downs: i32,
    r#type: PhantomData<T>,
}

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

impl<T> Clone for EdgeOrientation<T> {
    fn clone(&self) -> Self {
        EdgeOrientation::<T> {
            rights: self.rights,
            downs: self.downs,
            r#type: self.r#type,
        }
    }
}

impl<T> Copy for EdgeOrientation<T> {}

impl<T> PartialEq for EdgeOrientation<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rights == other.rights && self.downs == other.downs && self.r#type == other.r#type
    }
}

impl<T> Eq for EdgeOrientation<T> {}
