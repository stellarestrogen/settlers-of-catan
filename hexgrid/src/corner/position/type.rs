use crate::corner::position::{CornerPosition, Height, High, Low};

#[derive(Clone, Copy, PartialEq)]
pub enum CornerType {
    Low { position: CornerPosition<Low> },
    High { position: CornerPosition<High> },
}

impl CornerType {
    pub fn from<H: Height>(position: CornerPosition<H>) -> Self {
        if let Some(p) = position.as_low() {
            return CornerType::from_low(p);
        } else if let Some(p) = position.as_high() {
            return CornerType::from_high(p);
        }
        unreachable!()
    }

    pub fn from_low(position: CornerPosition<Low>) -> Self {
        CornerType::Low { position }
    }

    pub fn from_high(position: CornerPosition<High>) -> Self {
        CornerType::High { position }
    }

    pub fn get_low(&self) -> Option<CornerPosition<Low>> {
        match *self {
            CornerType::Low { position } => Some(position),
            CornerType::High { position: _ } => None
        }
    }

    pub fn get_high(&self) -> Option<CornerPosition<High>> {
        match *self {
            CornerType::Low { position: _ } => None,
            CornerType::High { position } => Some(position)
        }
    }
}
