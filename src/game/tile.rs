use std::{iter, ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}};

use super::objects::TileType;

#[derive(Clone)]
pub struct Tile {
    r#type: TileType
}

impl Tile {
    pub fn new(r#type: TileType) -> Self {
        Tile { r#type }
    }

    pub fn get_resource_type(&self) -> TileType {
        self.r#type
    }

    pub fn set_resource_type(&mut self, r#type: TileType) {
        self.r#type = r#type;
    }

}

pub enum HorizontalDistance {
    Shifted(i32),
    Unshifted(i32),
}

impl HorizontalDistance {
    pub fn ceil(self) -> i32 {
        match self {
            Self::Shifted(a) => a,
            Self::Unshifted(a) => a,
        }
    }
}

impl Into<f64> for HorizontalDistance {
    fn into(self) -> f64 {
        match self {
            Self::Shifted(a) => a as f64 - 0.5,
            Self::Unshifted(a) => a as f64
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TilePosition {
    rights: i32,
    downs: i32
}

impl TilePosition {
    pub const ORIGIN: TilePosition = TilePosition { rights: 0, downs: 0 };
    pub const RIGHT: TilePosition = TilePosition { rights: 1, downs: 0 };
    pub const DOWN_RIGHT: TilePosition = TilePosition { rights: 1, downs: 1 };
    pub const DOWN_LEFT: TilePosition = TilePosition { rights: 0, downs: 1 };
    pub const LEFT: TilePosition = TilePosition { rights: -1, downs: 0 };
    pub const UP_LEFT: TilePosition = TilePosition { rights: 0, downs: -1 };
    pub const UP_RIGHT: TilePosition = TilePosition { rights: 1, downs: -1 };

    pub fn horizontal_distance(&self, other: TilePosition) -> HorizontalDistance {
        let dist = (self.rights - other.rights).abs();
        if self.rights % 2 == other.rights % 2 {
            HorizontalDistance::Unshifted(dist)
        } else {
            HorizontalDistance::Shifted(dist)
        }
    }

    pub fn signed_horizontal_distance(&self, other: TilePosition) -> HorizontalDistance {
        let dist = self.rights - other.rights;
        if self.rights % 2 == other.rights % 2 {
            HorizontalDistance::Unshifted(dist)
        } else {
            HorizontalDistance::Shifted(dist)
        }
    }

    pub fn vertical_distance(&self, other: TilePosition) -> i32 {
        (self.downs - other.downs).abs()
    }

    pub fn signed_vertical_distance(&self, other: TilePosition) -> i32 {
        self.downs - other.downs
    }

}

impl Add for TilePosition {
    type Output = TilePosition;

    fn add(self, rhs: TilePosition) -> TilePosition {
        let adjustment_lhs = if self.downs % 2 == 1 && self.rights > 0 { -1 } else { 0 };
        let adjustment_rhs = if rhs.downs % 2 == 1 && rhs.rights > 0 { -1 } else { 0 };
        TilePosition {
            rights: self.rights + adjustment_lhs + rhs.rights + adjustment_rhs, 
            downs: self.downs + rhs.downs
        }
    }
}

impl AddAssign for TilePosition {
    fn add_assign(&mut self, rhs: TilePosition) {
        let adjustment_lhs = if self.downs % 2 == 1 && self.rights > 0 { -1 } else { 0 };
        let adjustment_rhs = if rhs.downs % 2 == 1 && rhs.rights > 0 { -1 } else { 0 };
        self.rights += adjustment_lhs + rhs.rights + adjustment_rhs; 
        self.downs += rhs.downs;
        
    }
}

impl Sub for TilePosition {
    type Output = TilePosition;

    fn sub(self, rhs: TilePosition) -> TilePosition {
        let adjustment_lhs = if self.downs % 2 == 1 && self.rights > 0 { -1 } else { 0 };
        let adjustment_rhs = if rhs.downs % 2 == 1 && rhs.rights > 0 { -1 } else { 0 };
        TilePosition {
            rights: self.rights + adjustment_lhs - (rhs.rights + adjustment_rhs), 
            downs: self.downs - rhs.downs
        }
    }
}

impl SubAssign for TilePosition {
    fn sub_assign(&mut self, rhs: TilePosition) {
        let adjustment_lhs = if self.downs % 2 == 1 && self.rights > 0 { -1 } else { 0 };
        let adjustment_rhs = if rhs.downs % 2 == 1 && rhs.rights > 0 { -1 } else { 0 };
        self.rights += adjustment_lhs;
        self.rights -= rhs.rights + adjustment_rhs;
        self.downs -= rhs.downs;
        
    }
}

macro_rules! scalar_operations {
    ($scalar: ty) => {
        impl Mul<$scalar> for TilePosition {
            type Output = TilePosition;
        
            fn mul(self, rhs: $scalar) -> TilePosition {
                let adjustment_lhs: i32 = if self.downs % 2 == 1 { -(rhs as i32)/2 } else { 0 };
        
                TilePosition {
                    rights: self.rights * rhs as i32 + adjustment_lhs,
                    downs: self.downs * rhs as i32
                }
            }
        }

        impl MulAssign<$scalar> for TilePosition {
            fn mul_assign(&mut self, rhs: $scalar) {
                let adjustment_lhs: i32 = if self.downs % 2 == 1 { -(rhs as i32)/2 } else { 0 };
                self.rights *= rhs as i32;
                self.rights += adjustment_lhs;
                self.downs *= rhs as i32;
            }
        }
    };
}

scalar_operations!(isize);
scalar_operations!(usize);
scalar_operations!(i128);
scalar_operations!(u128);
scalar_operations!(i64);
scalar_operations!(u64);
scalar_operations!(i32);
scalar_operations!(u32);
scalar_operations!(i16);
scalar_operations!(u16);
scalar_operations!(i8);
scalar_operations!(u8);

/// Holds the entire "game board" tiles, including water tiles.
pub struct TileHolder {
    tiles: Vec<Tile>,
    length: u32,
    width: u32
}

impl TileHolder {
    pub fn new(length: u32, width: u32) -> Self {
        let mut tiles = Vec::with_capacity((length * width) as usize);
        tiles.extend(iter::repeat_n(Tile::new(TileType::Water), (length * width) as usize));

        TileHolder {
            tiles,
            length,
            width
        }
    }

    fn calc_index(&self, position: TilePosition) -> Option<usize> {
        let rights: isize = position.signed_horizontal_distance(TilePosition::ORIGIN).ceil().try_into().ok()?;
        let downs: isize = position.signed_vertical_distance(TilePosition::ORIGIN).try_into().ok()?;
        let length: isize = self.length.try_into().ok()?;
        let width: isize = self.width.try_into().ok()?;

        if rights < length && downs < width && rights >= 0 && downs >= 0 {
            downs.checked_mul(length)?.checked_add(rights)?.try_into().ok()
        } else {
            None
        }
    }

    pub fn get(&self, position: TilePosition) -> Option<&Tile> {
        Some(&self.tiles[self.calc_index(position)?])
    }

    pub fn get_mut(&mut self, position: TilePosition) -> Option<&mut Tile> {
        let idx = self.calc_index(position)?;
        Some(&mut self.tiles[idx])
    }
}

impl Index<TilePosition> for TileHolder {
    type Output = Tile;

    fn index(&self, index: TilePosition) -> &Tile {
        self.get(index).expect("Indexed TileHolder out of bounds!")
    }
}

impl IndexMut<TilePosition> for TileHolder {
    fn index_mut(&mut self, index: TilePosition) -> &mut Tile {
        self.get_mut(index).expect("Indexed TileHolder out of bounds!")
    }
}