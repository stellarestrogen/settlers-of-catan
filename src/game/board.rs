use super::objects::*;
use super::tile::*;
use super::corner::*;
use super::edge::*;
use super::player::PlayerColor;


// number of each building each player starts with
pub const MAX_ROADS: u32 = 15;
pub const MAX_SETTLEMENTS: u32 = 5;
pub const MAX_CITIES: u32 = 4;

pub enum BoardSize {
    Hexagonal { shortest: u32, longest: u32 },
}


/// the idea for the board is to have 1 big vector, with each row containing the same amount of hexes.
/// the resource tiles will be placed (currently) according to how they are in the actual game.
pub struct Board {
    board_tiles: Vec<Tile>,
    row_length: u32,
    row_count: u32,
    resource_deck: TileDeck,
    edges: EdgeHolder,
    corners: CornerHolder,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board_tiles: Vec::<Tile>::new(),
            row_length: 0,
            row_count: 0,
            resource_deck: TileDeck::new(),
            edges: EdgeHolder::new(),
            corners: CornerHolder::new(),
        }
    }

    fn clear_board(&mut self) {
        self.board_tiles.clear();
        self.corners.clear();
        self.edges.clear();
        self.row_length = 0;
        self.row_count = 0;
        self.resource_deck.clear();
    }

    fn create_tile_board(&mut self, longest: u32, shortest: u32) {
        // abusing math :3
        let total_resource_tiles = longest + longest*(longest-1) - shortest*(shortest-1);

        // generate the resource tiles
        self.resource_deck.create_num_resource_tiles(total_resource_tiles);
        self.resource_deck.generate();

        self.board_tiles.reserve((self.row_length * self.row_count) as usize);

        // this is for every row we create.
        for y in 0..self.row_count {
            // 
            let mut row_resource_count = shortest + y;
            if y > self.row_count/2 {
                row_resource_count = longest + longest - shortest - y;
            }
            // tiles first
            let mut water_tiles = longest - row_resource_count;

            // if the number of water tiles is odd, put the extra tile first.
            if water_tiles % 2 == 1 { 
                self.board_tiles.push(Tile::new(TileType::Water));
                water_tiles -= 1;
            }
            
            // if there are still water tiles to add, split them in half and add the first half.
            for _ in 0..water_tiles/2 {
                self.board_tiles.push(Tile::new(TileType::Water));
            }

            // add the amount of resource tiles
            for _ in 0..row_resource_count {
                self.board_tiles.push(self.resource_deck.draw());
            }

            // add the other half of the water tiles
            for _ in 0..water_tiles/2 {
                self.board_tiles.push(Tile::new(TileType::Water));
            }
        }
    }

    fn create_edges(&mut self, length: u32, count: u32) {
        self.edges.generate(length, count);
    }

    fn create_corners(&mut self, length: u32, count: u32) {
        self.corners.setup(length, count);
    }

    // creating the board
    pub fn create_board(&mut self, size: BoardSize) {
        // first, clear the preexisting board
        self.clear_board();

        match size {
            BoardSize::Hexagonal { shortest, longest } => {
                self.row_length = longest;
                self.row_count = (longest - shortest) * 2 + 1;

                self.create_tile_board(longest, shortest);
                self.create_edges(self.row_length, self.row_count);
                self.create_corners(self.row_length, self.row_count);

                
            }
        }
    }
}