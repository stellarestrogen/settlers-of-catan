use std::marker::PhantomData;

use crate::{
    corner::position::{CornerHeight, CornerPosition},
    edge::Edge,
    hex::position::HexPosition,
};

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

#[derive(Clone, Copy, PartialEq, Eq)]
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

    /// If `other` does not form a gap with `self`, will return Err.
    pub fn find_gap(&self, other: Self) -> Result<Self, ()> {
        if self.distance(other) != 4 {
            return Err(());
        }

        let rights = ((self.rights() as f32 + other.rights() as f32) / 2.).round_ties_even() as i32;
        let downs = (self.downs() + other.downs()) / 2;

        EdgePosition::from_rights_and_downs(rights, downs)
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

    pub fn neighboring_corners(&self) -> [CornerPosition; 2] {
        match self {
            Self::Even(p) => {
                let [top_left, bottom_right] = p.neighboring_hex();
                [
                    (top_left + CornerHeight::BOTTOM).into(),
                    (bottom_right + CornerHeight::TOP).into(),
                ]
            }
            Self::Odd(p) => {
                let [top_right, bottom_left] = p.neighboring_hex();
                [
                    (top_right + CornerHeight::BOTTOM).into(),
                    (bottom_left + CornerHeight::TOP).into(),
                ]
            }
            Self::Positive(p) => {
                let [left, right] = p.neighboring_hex();
                [
                    (left + CornerHeight::TOP_RIGHT).into(),
                    (right + CornerHeight::BOTTOM_LEFT).into(),
                ]
            }
        }
    }

    pub fn is_neighbor(&self, other: Self) -> bool {
        (self.distance(other)) == 2 && self.rights() != other.rights()
    }

    /// Returns Ok if first and second are both neighbors of self. Otherwise, it returns Err.
    pub fn are_edges_same_side(&self, first: Self, second: Self) -> Result<bool, ()> {
        if !self.is_neighbor(first) || !self.is_neighbor(second) {
            return Err(());
        }

        if first.is_neighbor(second) {
            return Ok(true);
        } else if first.distance(second) == 4 {
            return Ok(false);
        }

        Err(())
    }

    fn distance(&self, other: Self) -> i32 {
        self.horizontal_distance(other).abs() + self.vertical_distance(other).abs()
    }

    fn from_rights_and_downs(rights: i32, downs: i32) -> Result<Self, ()> {
        if rights % 4 == 0 && downs % 2 == 0 {
            Ok(Self::Even(EdgeOrientation {
                rights,
                downs,
                r#type: PhantomData,
            }))
        } else if rights % 4 == 1 && downs % 2 == 1 {
            Ok(Self::Positive(EdgeOrientation {
                rights,
                downs,
                r#type: PhantomData,
            }))
        } else if rights % 4 == 2 && downs % 2 == 0 {
            Ok(Self::Odd(EdgeOrientation {
                rights,
                downs,
                r#type: PhantomData,
            }))
        } else {
            Err(())
        }
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

impl std::fmt::Debug for EdgePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgePosition::Even(p) => {
                write!(
                    f,
                    "EdgePosition::Even ( rights: {:?}, downs: {:?} )",
                    p.rights, p.downs
                )
            }
            EdgePosition::Odd(p) => {
                write!(
                    f,
                    "EdgePosition::Odd ( rights: {:?}, downs: {:?} )",
                    p.rights, p.downs
                )
            }
            EdgePosition::Positive(p) => {
                write!(
                    f,
                    "EdgePosition::Positive ( rights: {:?}, downs: {:?} )",
                    p.rights, p.downs
                )
            }
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
