use std::ops::{Sub, SubAssign};

use crate::game::position::Position;

use super::TilePosition;

impl Sub for TilePosition {
    type Output = TilePosition;

    fn sub(self, rhs: Self) -> Self::Output {
        TilePosition {
            rights: self
                .horizontal_distance(TilePosition::ORIGIN)
                .sub(rhs.horizontal_distance(TilePosition::ORIGIN))
                .ceil(),

            downs: self
                .vertical_distance(TilePosition::ORIGIN)
                .sub(rhs.vertical_distance(TilePosition::ORIGIN)),
        }
    }
}

impl SubAssign for TilePosition {
    fn sub_assign(&mut self, rhs: Self) {
        self.rights = self
            .horizontal_distance(TilePosition::ORIGIN)
            .sub(rhs.horizontal_distance(TilePosition::ORIGIN))
            .ceil();

        self.downs = self
            .vertical_distance(TilePosition::ORIGIN)
            .sub(rhs.vertical_distance(TilePosition::ORIGIN));
    }
}
