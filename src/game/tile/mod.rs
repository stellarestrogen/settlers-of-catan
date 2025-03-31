pub mod holder;
pub mod position;

use super::objects::TileType;

#[derive(Clone)]
pub struct Tile {
    r#type: TileType,
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
