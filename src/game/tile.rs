use super::objects::*;
use rand::prelude::*;

// number of each resource tile in the base game
const WOOD_TILES_BASE: u32 = 4;
const BRICK_TILES_BASE: u32 = 3;
const WHEAT_TILES_BASE: u32 = 4;
const SHEEP_TILES_BASE: u32 = 4;
const ORE_TILES_BASE: u32 = 3;
const DESERT_TILES_BASE: u32 = 1;
const TOTAL_RESOURCE_TILES_BASE: u32 = WOOD_TILES_BASE + BRICK_TILES_BASE + WHEAT_TILES_BASE + SHEEP_TILES_BASE + ORE_TILES_BASE;
const TOTAL_TILES_BASE: u32 = TOTAL_RESOURCE_TILES_BASE + DESERT_TILES_BASE;

// roll order in the base game
const ROLL_ORDER_BASE: [u32; TOTAL_RESOURCE_TILES_BASE as usize] = [
    5, 2, 6, 3, 8, 10, 9, 12, 11,
    4, 8, 10, 9, 4, 5, 6, 3, 11
];

// number of each resource tile in the expansion, cumulative with the base game
const WOOD_TILES_EXP: u32 = 2;
const BRICK_TILES_EXP: u32 = 2;
const WHEAT_TILES_EXP: u32 = 2;
const SHEEP_TILES_EXP: u32 = 2;
const ORE_TILES_EXP: u32 = 2;
const DESERT_TILES_EXP: u32 = 1;
const TOTAL_RESOURCE_TILES_EXP: u32 = WOOD_TILES_EXP + BRICK_TILES_EXP + WHEAT_TILES_EXP + SHEEP_TILES_EXP + ORE_TILES_EXP;
const TOTAL_TILES_EXP: u32 = TOTAL_RESOURCE_TILES_EXP + DESERT_TILES_EXP;

// roll order in the expansion
const ROLL_ORDER: [u32; (TOTAL_RESOURCE_TILES_BASE + TOTAL_RESOURCE_TILES_EXP) as usize] = [
    2, 5, 4, 6, 3, 9, 8, 11, 11, 10, 6, 3, 8, 4,
    8, 10, 11, 12, 10, 5, 4, 9, 5, 9, 12, 3, 2, 6
];

#[derive(Clone)]
#[derive(Copy)]
pub struct Tile {
    tile_type: TileType,
    roll_number: u32,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            roll_number: 0
        }
    }

    pub fn get_tile_type(&self) -> TileType {
        self.tile_type
    }

    pub fn get_roll_num(&self) -> u32 {
        self.roll_number
    }

    pub fn get_tile_resource(&self) -> Option<ResourceType> {
        match self.tile_type {
            TileType::Resource(resource) => Some(resource),
            _ => None
        }
    }

    pub fn set_roll_num(&mut self, roll: u32) {
        self.roll_number = roll;
    }
}

pub struct TileDeck {
    tiles: Vec<Tile>,
    total_tiles: u32,
    total_tiles_wood: u32,
    total_tiles_brick: u32,
    total_tiles_wheat: u32,
    total_tiles_sheep: u32,
    total_tiles_ore: u32,
    total_tiles_desert: u32,
}

impl TileDeck {
    pub fn new() -> Self {
        TileDeck {
            tiles: Vec::<Tile>::new(),
            total_tiles: 0,
            total_tiles_wood: 0,
            total_tiles_brick: 0,
            total_tiles_wheat: 0,
            total_tiles_sheep: 0,
            total_tiles_ore: 0,
            total_tiles_desert: 0,
        }
        
    }

    pub fn clear(&mut self) {
        self.tiles.clear();
        self.total_tiles = 0;
        self.total_tiles_wood = 0;
        self.total_tiles_brick = 0;
        self.total_tiles_wheat = 0;
        self.total_tiles_sheep = 0;
        self.total_tiles_ore = 0;
        self.total_tiles_desert = 0;
    }

    pub fn set_total_tiles(&mut self, total: u32) {
        self.total_tiles = total;
    }

    fn calc_total_tiles(&mut self) {
        self.total_tiles = self.total_tiles_wood + self.total_tiles_brick + self.total_tiles_wheat + 
        self.total_tiles_sheep + self.total_tiles_ore + self.total_tiles_ore;
    }

    pub fn create_num_resource_tiles(&mut self, max_tiles: u32) {
        self.set_total_tiles(max_tiles);
        // this prioritizes the base number of each tiles in the real life board game, rather than using percentages to determine the number of each tile.
        // this will only work for the sizes of board that matches the base game and the base game + expansion. 
        if max_tiles == TOTAL_TILES_BASE {
            self.total_tiles_wood = WOOD_TILES_BASE;
            self.total_tiles_brick = BRICK_TILES_BASE;
            self.total_tiles_wheat = WHEAT_TILES_BASE;
            self.total_tiles_sheep = SHEEP_TILES_BASE;
            self.total_tiles_ore = ORE_TILES_BASE;
            self.total_tiles_desert = DESERT_TILES_BASE;

        } else if max_tiles == TOTAL_TILES_BASE + TOTAL_TILES_EXP {
            self.total_tiles_wood = WOOD_TILES_BASE + WOOD_TILES_EXP;
            self.total_tiles_brick = BRICK_TILES_BASE + BRICK_TILES_EXP;
            self.total_tiles_wheat = WHEAT_TILES_BASE + WHEAT_TILES_EXP;
            self.total_tiles_sheep = SHEEP_TILES_BASE + SHEEP_TILES_EXP;
            self.total_tiles_ore = ORE_TILES_BASE + ORE_TILES_EXP;
            self.total_tiles_desert = DESERT_TILES_BASE + DESERT_TILES_EXP;

        } else {
            // this produces the exact same results as the constants (for their sizes), but might as well avoid these calculations since most games will probably not use custom board size.
            self.total_tiles_wood = ((max_tiles as f32)/5.0).round() as u32;
            self.total_tiles_brick = ((max_tiles as f32)/6.0).round() as u32;
            self.total_tiles_wheat = ((max_tiles as f32)/5.0).round() as u32;
            self.total_tiles_sheep = ((max_tiles as f32)/5.0).round() as u32;
            self.total_tiles_ore = ((max_tiles as f32)/6.0).round() as u32;
            self.total_tiles_desert = ((max_tiles as f32)/15.0).floor() as u32;

            self.calc_total_tiles();
            // assume all overshoots or undershoots are off-by-one.
            // from my limited testing it never deviates by more than 1, so just add or remove a desert tile.
            if max_tiles > self.total_tiles { 
                self.total_tiles_desert += 1 
            } else { 
                self.total_tiles_desert -= 1;
            };
        }

    }

    /// takes an array of tiles and shuffles them.
    fn shuffle(&mut self, mut tiles: Vec<Tile>) {
        let mut tiles_shuffled = Vec::<Tile>::with_capacity(self.total_tiles as usize);
        while tiles.len() > 0 {
            let btwn = rand::distributions::Uniform::new(0, tiles.len());
            let mut rng = rand::thread_rng();
            let rand = btwn.sample(&mut rng);
            tiles_shuffled.push(tiles[rand]);
            tiles.remove(rand);
        }

        self.tiles = tiles_shuffled;
    }

    /// creates the resource tiles based on the amount calculated in `create_num_resource_tiles`.
    pub fn generate(&mut self) {
        let mut tiles_ordered = Vec::<Tile>::with_capacity(self.total_tiles as usize);
        for _ in 0..self.total_tiles_wood {
            tiles_ordered.push(Tile::new(TileType::Resource(ResourceType::Wood)));
        }

        for _ in 0..self.total_tiles_brick {
            tiles_ordered.push(Tile::new(TileType::Resource(ResourceType::Brick)));
        }

        for _ in 0..self.total_tiles_wheat {
            tiles_ordered.push(Tile::new(TileType::Resource(ResourceType::Wheat)));
        }

        for _ in 0..self.total_tiles_sheep {
            tiles_ordered.push(Tile::new(TileType::Resource(ResourceType::Sheep)));
        }

        for _ in 0..self.total_tiles_ore {
            tiles_ordered.push(Tile::new(TileType::Resource(ResourceType::Ore)));
        }

        for _ in 0..self.total_tiles_desert {
            tiles_ordered.push(Tile::new(TileType::Desert));
        }
        
        self.shuffle(tiles_ordered);

    }

    pub fn draw(&mut self) -> Tile {
        self.tiles.pop().unwrap_or(panic!("No tiles left to draw!\n"));
    }
}