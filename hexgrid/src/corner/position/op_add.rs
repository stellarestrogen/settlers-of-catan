use std::{marker::PhantomData, ops::Add};

use crate::{hex::position::HexPosition, position::Position};

use super::{CornerPosition, Height, High, Low};

impl Add for CornerPosition<Low> {
    type Output = CornerPosition<High>;

    fn add(self, rhs: Self) -> Self::Output {
        CornerPosition::<High> {
            rights: self.rights + rhs.rights,
            downs: self.downs + rhs.downs,
            height: PhantomData::<High>
        }
    }
}

impl Add for CornerPosition<High> {
    type Output = CornerPosition<Low>;

    fn add(self, rhs: Self) -> Self::Output {
        CornerPosition::<Low> {
            rights: self.rights + rhs.rights,
            downs: self.downs + rhs.downs,
            height: PhantomData::<Low>
        }
    }
}

impl Add<CornerPosition<High>> for CornerPosition<Low> {
    type Output = HexPosition;

    fn add(self, rhs: CornerPosition<High>) -> Self::Output {
        let rights = self.rights + rhs.rights;
        let downs = self.downs + rhs.downs;

        if rights.signum() == downs.signum() {
            (HexPosition::RIGHT + HexPosition::DOWN_RIGHT) * ((downs - 1)/3) + HexPosition::RIGHT * ((rights - downs)/2)
        } else {
            (HexPosition::LEFT + HexPosition::DOWN_LEFT) * ((downs - 1)/3) + HexPosition::RIGHT * ((rights.abs() - downs.abs())/2) + HexPosition::UP_RIGHT
        }
    }
}

impl Add<CornerPosition<Low>> for CornerPosition<High> {
    type Output = HexPosition;

    fn add(self, rhs: CornerPosition<Low>) -> Self::Output {
        let rights = self.rights + rhs.rights;
        let downs = self.downs + rhs.downs;

        if rights.signum() == downs.signum() {
            (HexPosition::RIGHT + HexPosition::DOWN_RIGHT) * ((downs - 1)/3) + HexPosition::RIGHT * ((rights - downs)/2)
        } else {
            (HexPosition::LEFT + HexPosition::DOWN_LEFT) * ((downs - 1)/3) + HexPosition::RIGHT * ((rights.abs() - downs.abs())/2) + HexPosition::UP_RIGHT
        }
    }
}

impl<H: Height> Add<HexPosition> for CornerPosition<H> {
    type Output = CornerPosition<H>;

    fn add(self, rhs: HexPosition) -> Self::Output {
        let shift: f64 = rhs.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: self.rights + (shift * 2.) as i32,
            downs: self.downs + rhs.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<H>
        }
    }
}

impl<H: Height> Add<CornerPosition<H>> for HexPosition {
    type Output = CornerPosition<H>;

    fn add(self, rhs: CornerPosition<H>) -> Self::Output {
        let shift: f64 = self.horizontal_distance(HexPosition::ORIGIN).into();
        CornerPosition {
            rights: rhs.rights + (shift * 2.) as i32,
            downs: rhs.downs + self.vertical_distance(HexPosition::ORIGIN) * 3,
            height: PhantomData::<H>
        }
    }
}