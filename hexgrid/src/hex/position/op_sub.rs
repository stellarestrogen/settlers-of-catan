use std::ops::{Sub, SubAssign};

use super::HexPosition;

impl Sub for HexPosition {
    type Output = HexPosition;

    fn sub(self, rhs: Self) -> Self::Output {
        HexPosition {
            rights: self
                .horizontal_displacement(HexPosition::ORIGIN)
                .sub(rhs.horizontal_displacement(HexPosition::ORIGIN))
                .ceil(),

            downs: self
                .vertical_displacement(HexPosition::ORIGIN)
                .sub(rhs.vertical_displacement(HexPosition::ORIGIN)),
        }
    }
}

impl SubAssign for HexPosition {
    fn sub_assign(&mut self, rhs: Self) {
        self.rights = self
            .horizontal_displacement(HexPosition::ORIGIN)
            .sub(rhs.horizontal_displacement(HexPosition::ORIGIN))
            .ceil();

        self.downs = self
            .vertical_displacement(HexPosition::ORIGIN)
            .sub(rhs.vertical_displacement(HexPosition::ORIGIN));
    }
}
