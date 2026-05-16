use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::object::{TileData, TileType, resource::ResourceType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Tsify, Serialize, Deserialize)]
pub enum TileResourceType {
    Wood,
    Brick,
    Wheat,
    Sheep,
    Ore,
    Desert,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Tsify, Serialize, Deserialize)]
pub struct WasmTileData {
    pub tile_type: TileResourceType,
    pub roll_number: Option<u32>,
}

impl WasmTileData {
    pub fn from_tile_data(tile: TileData) -> Self {
        let (r#type, roll_number) = match tile.get_tile_type() {
            TileType::Resource {
                resource,
                roll_number,
            } => {
                let roll_number = Some(roll_number);
                match resource {
                    ResourceType::Wood => (TileResourceType::Wood, roll_number),
                    ResourceType::Brick => (TileResourceType::Brick, roll_number),
                    ResourceType::Wheat => (TileResourceType::Wheat, roll_number),
                    ResourceType::Sheep => (TileResourceType::Sheep, roll_number),
                    ResourceType::Ore => (TileResourceType::Ore, roll_number),
                }
            }
            TileType::Desert => (TileResourceType::Desert, None),
            TileType::Water => (TileResourceType::Water, None),
        };

        Self {
            tile_type: r#type,
            roll_number,
        }
    }
}
