use super::objects::*;
use rand::prelude::*;
use std::iter;

static RESOURCES: [ResourceType; 5] = [ResourceType::Wood, ResourceType::Brick, ResourceType::Wheat, ResourceType::Sheep, ResourceType::Ore];

const ROLL_ORDER_BASE: [u32; 18] = [
    5, 2, 6, 3, 8, 10, 9, 12, 11,
    4, 8, 10, 9, 4, 5, 6, 3, 11
];

const ROLL_ORDER_EXP: [u32; 28] = [
    2, 5, 4, 6, 3, 9, 8, 11, 11, 10, 6, 3, 8, 4,
    8, 10, 11, 12, 10, 5, 4, 9, 5, 9, 12, 3, 2, 6
];

struct Tile {
    r#type: TileType
}

impl Tile {
    pub fn new(r#type: TileType) -> Self {
        Tile { r#type }
    }

    pub fn get_resource_type(&self) -> TileType {
        self.r#type
    }

}

enum TileBoardType {
    Base,
    Expansion,
    Custom {
        size: u32
    }
}

impl TileBoardType {
    pub fn get_size(&self) -> u32 {
        match self {
            Self::Base => 19,
            Self::Expansion => 30,
            Self::Custom { size} => *size
        }
    }

    fn create_roll_numbers(&self) -> Vec<u32> {
        let mut roll_numbers = Vec::<u32>::with_capacity(self.get_size() as usize);
        match self {
            Self::Base => roll_numbers = ROLL_ORDER_BASE.to_vec(),
            Self::Expansion => roll_numbers = ROLL_ORDER_EXP.to_vec(),
            Self::Custom { .. }=> {
                static ROLLS_RANDOM: [u32; 10] = [6, 8, 5, 9, 4, 10, 3, 11, 2, 12];
                while roll_numbers.len() < self.get_size() as usize {
                    roll_numbers.extend_from_slice(&ROLLS_RANDOM);
                }

                roll_numbers.truncate(self.get_size() as usize);

                roll_numbers.shuffle(&mut rand::thread_rng());
            }
        }
        roll_numbers
    }

    pub fn create_tiles(&self) -> Vec<Tile> {
        let mut resources = Vec::<Option<ResourceType>>::with_capacity(self.get_size() as usize);
        for resource in RESOURCES {
            resources.extend(iter::repeat_n(Some(resource), resource.get_resource_distribution(self.get_size()) as usize));
        }
        resources.truncate(self.get_size() as usize);

        while resources.len() < self.get_size() as usize {
            resources.push(None);
        }

        resources.shuffle(&mut rand::thread_rng());

        let mut roll_numbers = self.create_roll_numbers().into_iter();

        resources.into_iter()
        .map(|resource| resource.map(|rsrc| Tile::new(TileType::Resource { resource: rsrc, roll_number: roll_numbers.next().unwrap() } ))
            .unwrap_or(Tile::new(TileType::Desert)))
        .collect()
        
    }

}

pub struct TileDeck {
    tiles: Vec<Tile>
}

impl TileDeck {
    pub fn new(board_type: TileBoardType) -> Self {
        TileDeck {
            tiles: board_type.create_tiles()
        }
    }

    pub fn draw(&mut self) -> Option<Tile> {
        self.tiles.pop()
    }
    
}