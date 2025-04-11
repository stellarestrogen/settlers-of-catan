use std::ops::{Add, Mul, Sub};

use num_traits::PrimInt;

pub enum HorizontalAxis {
    Left,
    Right
}

pub enum VerticalAxis {
    Up,
    Down
}

/// The Position trait.
/// 
/// Used to define something as a position in 2D space.
/// 
/// You can define which axes are positive by overriding `positive_axes()` function. The default is that "right" and "up" are positive.
pub trait Position<T: PrimInt>: Add + Sub + Mul<T> + Sized {
    type HorizontalOutput: Into<T>;
    type VerticalOutput: Into<T>;

    fn horizontal_distance(&self, other: Self) -> Self::HorizontalOutput;
    fn vertical_distance(&self, other: Self) -> Self::VerticalOutput;

    fn positive_axes() -> (HorizontalAxis, VerticalAxis) {
        (HorizontalAxis::Right, VerticalAxis::Up)
    }

    fn is_right(&self, other: Self) -> bool {
        let (x, _) = Self::positive_axes();
        match x {
            HorizontalAxis::Left => self.horizontal_distance(other).into() < T::zero(),
            HorizontalAxis::Right => self.horizontal_distance(other).into() > T::zero(),
        }
    }

    fn is_right_or_equal(&self, other: Self) -> bool {
        let (x, _) = Self::positive_axes();
        match x {
            HorizontalAxis::Left => self.horizontal_distance(other).into() <= T::zero(),
            HorizontalAxis::Right => self.horizontal_distance(other).into() >= T::zero(),
        }
    }

    fn is_left(&self, other: Self) -> bool {
        let (x, _) = Self::positive_axes();
        match x {
            HorizontalAxis::Left => self.horizontal_distance(other).into() > T::zero(),
            HorizontalAxis::Right => self.horizontal_distance(other).into() < T::zero(),
        }
    }

    fn is_left_or_equal(&self, other: Self) -> bool {
        let (x, _) = Self::positive_axes();
        match x {
            HorizontalAxis::Left => self.horizontal_distance(other).into() >= T::zero(),
            HorizontalAxis::Right => self.horizontal_distance(other).into() <= T::zero(),
        }
    }

    fn is_below(&self, other: Self) -> bool {
        let (_, y) = Self::positive_axes();
        match y {
            VerticalAxis::Up => self.vertical_distance(other).into() < T::zero(),
            VerticalAxis::Down => self.vertical_distance(other).into() > T::zero(),
        }
    }

    fn is_below_or_equal(&self, other: Self) -> bool {
        let (_, y) = Self::positive_axes();
        match y {
            VerticalAxis::Up => self.vertical_distance(other).into() <= T::zero(),
            VerticalAxis::Down => self.vertical_distance(other).into() >= T::zero(),
        }
    }

    fn is_above(&self, other: Self) -> bool {
        let (_, y) = Self::positive_axes();
        match y {
            VerticalAxis::Up => self.vertical_distance(other).into() > T::zero(),
            VerticalAxis::Down => self.vertical_distance(other).into() < T::zero(),
        }
    }

    fn is_above_or_equal(&self, other: Self) -> bool {
        let (_, y) = Self::positive_axes();
        match y {
            VerticalAxis::Up => self.vertical_distance(other).into() >= T::zero(),
            VerticalAxis::Down => self.vertical_distance(other).into() <= T::zero(),
        }
    }
}

