use crate::edge::position::{EdgePosition, Even, Odd, Positive, Valid};

#[derive(Clone, Copy, PartialEq)]
pub enum EdgeType {
    Even { position: EdgePosition<Even> },
    Odd { position: EdgePosition<Odd> },
    Positive { position: EdgePosition<Positive> },
}

impl EdgeType {
    pub fn into<Type: Valid>(position: EdgePosition<Type>) -> Self {
        if let Some(p) = position.as_even() {
            return EdgeType::from_even(p);
        } else if let Some(p) = position.as_odd() {
            return EdgeType::from_odd(p);
        } else if let Some(p) = position.as_positive() {
            return EdgeType::from_positive(p);
        }
        unreachable!()
    }

    pub fn from_even(position: EdgePosition<Even>) -> Self {
        EdgeType::Even { position }
    }

    pub fn from_odd(position: EdgePosition<Odd>) -> Self {
        EdgeType::Odd { position }
    }

    pub fn from_positive(position: EdgePosition<Positive>) -> Self {
        EdgeType::Positive { position }
    }
}
