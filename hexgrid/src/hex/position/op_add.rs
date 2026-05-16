use std::ops::{Add, AddAssign};

use super::HexPosition;

impl Add for HexPosition {
    type Output = HexPosition;

    fn add(self, rhs: HexPosition) -> HexPosition {
        HexPosition {
            rights: self
                .horizontal_displacement(HexPosition::ORIGIN)
                .add(rhs.horizontal_displacement(HexPosition::ORIGIN))
                .ceil(),

            downs: self
                .vertical_displacement(HexPosition::ORIGIN)
                .add(rhs.vertical_displacement(HexPosition::ORIGIN)),
        }
    }
}

impl AddAssign for HexPosition {
    fn add_assign(&mut self, rhs: Self) {
        self.rights = self
            .horizontal_displacement(HexPosition::ORIGIN)
            .add(rhs.horizontal_displacement(HexPosition::ORIGIN))
            .ceil();

        self.downs = self
            .vertical_displacement(HexPosition::ORIGIN)
            .add(rhs.vertical_displacement(HexPosition::ORIGIN));
    }
}
