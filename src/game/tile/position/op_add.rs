use std::ops::{Add, AddAssign};

use crate::game::position::Position;

use super::TilePosition;

impl Add for TilePosition {
    type Output = TilePosition;

    fn add(self, rhs: TilePosition) -> TilePosition {
        TilePosition {
            rights: self
                .horizontal_distance(TilePosition::ORIGIN)
                .add(rhs.horizontal_distance(TilePosition::ORIGIN))
                .ceil(),

            downs: self
                .vertical_distance(TilePosition::ORIGIN)
                .add(rhs.vertical_distance(TilePosition::ORIGIN)),
        }
    }
}

impl AddAssign for TilePosition {
    fn add_assign(&mut self, rhs: Self) {
        self.rights = self
            .horizontal_distance(TilePosition::ORIGIN)
            .add(rhs.horizontal_distance(TilePosition::ORIGIN))
            .ceil();

        self.downs = self
            .vertical_distance(TilePosition::ORIGIN)
            .add(rhs.vertical_distance(TilePosition::ORIGIN));
    }
}
