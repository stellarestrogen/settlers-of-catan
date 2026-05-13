use std::marker::PhantomData;

use crate::{
    corner::{
        Corner,
        position::{high::High, low::Low},
    },
    edge::position::{EdgeOrientation, EdgePosition},
    hex::position::HexPosition,
};

pub mod center;
pub mod high;
pub mod low;
pub mod op_add;
pub mod op_mul;
pub mod op_sub;

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

        // let p: Vec<CornerPosition> = self.position().neighboring_corners().into_iter().map(Into::<CornerPosition>::into).collect();

        p.try_into()
            .expect("Neighboring Corners is the incorrect size!")
    }

    pub fn neighboring_edges(&self) -> [EdgePosition; 3] {
        match self {
            Self::High(p) => {
                let [top_left, top_right, down] = p.neighboring_hex();
                [
                    (top_left + EdgeOrientation::RIGHT).into(),
                    (top_right + EdgeOrientation::BOTTOM_LEFT).into(),
                    (down + EdgeOrientation::TOP_LEFT).into(),
                ]
            }
            Self::Low(p) => {
                let [up, down_left, down_right] = p.neighboring_hex();
                [
                    (up + EdgeOrientation::BOTTOM_RIGHT).into(),
                    (down_left + EdgeOrientation::RIGHT).into(),
                    (down_right + EdgeOrientation::TOP_RIGHT).into(),
                ]
            }
        }
    }

    pub fn is_neighbor(&self, other: Self) -> bool {
        (self.horizontal_distance(other).abs() + self.vertical_distance(other).abs()) == 2
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

impl<H> Copy for CornerHeight<H> {}

impl<H> PartialEq for CornerHeight<H> {
    fn eq(&self, other: &Self) -> bool {
        self.rights == other.rights && self.downs == other.downs && self.height == other.height
    }
}

impl<H> Eq for CornerHeight<H> {}
