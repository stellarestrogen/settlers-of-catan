use crate::game::position::{HorizontalAxis, Position, VerticalAxis};

pub mod op_add;
pub mod op_mul;
pub mod op_sub;

#[derive(Clone, Copy)]
pub struct EdgePosition {
    rights: i32,
    downs: i32
}

impl EdgePosition {
    pub const EMPTY: EdgePosition = EdgePosition {
        rights: 0,
        downs: 0
    };

    pub const RIGHT: EdgePosition = EdgePosition {
        rights: 2,
        downs: 0
    };

    pub const DOWN_RIGHT: EdgePosition = EdgePosition {
        rights: 1,
        downs: 1
    };

    pub const DOWN: EdgePosition = EdgePosition {
        rights: 0,
        downs: 2
    };

    pub const DOWN_LEFT: EdgePosition = EdgePosition {
        rights: -1,
        downs: 1
    };

    pub const LEFT: EdgePosition = EdgePosition {
        rights: -2,
        downs: 0
    };

    pub const UP_LEFT: EdgePosition = EdgePosition {
        rights: -1,
        downs: -1
    };

    pub const UP: EdgePosition = EdgePosition {
        rights: 0,
        downs: -2
    };

    pub const UP_RIGHT: EdgePosition = EdgePosition {
        rights: 1,
        downs: -1
    };

    fn is_valid(&self) -> bool {
        (self.rights % 2 == 0 && self.downs % 2 == 0)||(self.rights % 2 == 1 && self.downs % 2 == 1 && (self.rights + self.downs) % 4 == 0)
    }

    pub fn calc_distance(&self, other: Self) -> i32 {
        (self.horizontal_distance(other).abs() + self.vertical_distance(other).abs())/2
    }

    pub fn calc_adjacent_edges(&self) -> Vec<EdgePosition> {
        let edges = [
            *self + EdgePosition::RIGHT,
            *self + EdgePosition::DOWN_RIGHT,
            *self + EdgePosition::DOWN_LEFT,
            *self + EdgePosition::LEFT,
            *self + EdgePosition::UP_LEFT,
            *self + EdgePosition::UP_RIGHT
        ];

        edges.into_iter()
        .filter_map(|r| r )
        .collect()

    }
}

impl Position<i32> for EdgePosition {
    type HorizontalOutput = i32;
    type VerticalOutput = i32;

    fn positive_axes() -> (HorizontalAxis, VerticalAxis) {
        (HorizontalAxis::Right, VerticalAxis::Down)
    }

    fn horizontal_distance(&self, other: Self) -> Self::HorizontalOutput {
        self.rights - other.rights
    }

    fn vertical_distance(&self, other: Self) -> Self::VerticalOutput {
        self.downs - other.downs
    }
}