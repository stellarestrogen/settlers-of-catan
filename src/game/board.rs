use super::objects::*;
use super::player::PlayerColor;
use rand::prelude::*;

// number of each resource tile in the base game
const WOOD_TILES_BASE: usize = 4;
const BRICK_TILES_BASE: usize = 3;
const WHEAT_TILES_BASE: usize = 4;
const SHEEP_TILES_BASE: usize = 4;
const ORE_TILES_BASE: usize = 3;
const DESERT_TILES_BASE: usize = 1;
const TOTAL_RESOURCE_TILES_BASE: usize = WOOD_TILES_BASE + BRICK_TILES_BASE + WHEAT_TILES_BASE + SHEEP_TILES_BASE + ORE_TILES_BASE;
const TOTAL_TILES_BASE: usize = TOTAL_RESOURCE_TILES_BASE + DESERT_TILES_BASE;

// roll order in the base game
const ROLL_ORDER_BASE: [i32; TOTAL_RESOURCE_TILES_BASE] = [
    5, 2, 6, 3, 8, 10, 9, 12, 11,
    4, 8, 10, 9, 4, 5, 6, 3, 11
];

// number of each resource tile in the expansion, cumulative with the base game
const WOOD_TILES_EXP: usize = 2;
const BRICK_TILES_EXP: usize = 2;
const WHEAT_TILES_EXP: usize = 2;
const SHEEP_TILES_EXP: usize = 2;
const ORE_TILES_EXP: usize = 2;
const DESERT_TILES_EXP: usize = 1;
const TOTAL_RESOURCE_TILES_EXP: usize = WOOD_TILES_EXP + BRICK_TILES_EXP + WHEAT_TILES_EXP + SHEEP_TILES_EXP + ORE_TILES_EXP;
const TOTAL_TILES_EXP: usize = TOTAL_RESOURCE_TILES_EXP + DESERT_TILES_EXP;

// roll order in the expansion
const ROLL_ORDER: [i32; TOTAL_RESOURCE_TILES_BASE + TOTAL_RESOURCE_TILES_EXP] = [
    2, 5, 4, 6, 3, 9, 8, 11, 11, 10, 6, 3, 8, 4,
    8, 10, 11, 12, 10, 5, 4, 9, 5, 9, 12, 3, 2, 6
];

// number of each building each player starts with
pub const MAX_ROADS: usize = 15;
pub const MAX_SETTLEMENTS: usize = 5;
pub const MAX_CITIES: usize = 4;

pub enum BoardSize {
    Hexagonal { shortest: usize, longest: usize },
}

struct TileDeck {
    tiles: Vec<Tile>,
    total_tiles: usize,
    total_tiles_wood: usize,
    total_tiles_brick: usize,
    total_tiles_wheat: usize,
    total_tiles_sheep: usize,
    total_tiles_ore: usize,
    total_tiles_desert: usize,
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

    pub fn set_total_tiles(&mut self, total: usize) {
        self.total_tiles = total;
    }

    fn calc_total_tiles(&mut self) {
        self.total_tiles = self.total_tiles_wood + self.total_tiles_brick + self.total_tiles_wheat + 
        self.total_tiles_sheep + self.total_tiles_ore + self.total_tiles_ore;
    }

    pub fn create_num_resource_tiles(&mut self, max_tiles: usize) {
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
            self.total_tiles_wood = ((max_tiles as f32)/5.0).round() as usize;
            self.total_tiles_brick = ((max_tiles as f32)/6.0).round() as usize;
            self.total_tiles_wheat = ((max_tiles as f32)/5.0).round() as usize;
            self.total_tiles_sheep = ((max_tiles as f32)/5.0).round() as usize;
            self.total_tiles_ore = ((max_tiles as f32)/6.0).round() as usize;
            self.total_tiles_desert = ((max_tiles as f32)/15.0).floor() as usize;

            // assume all overshoots or undershoots are off-by-one.
            // from my limited testing it never deviates by more than 1, so just add or remove a desert tile.
            self.total_tiles_desert += max_tiles - self.total_tiles;
        }

    }

    fn shuffle(&mut self, mut tiles: Vec<Tile>) {
        let mut tiles_shuffled = Vec::<Tile>::with_capacity(self.total_tiles);
        while tiles.len() > 0 {
            let btwn = rand::distributions::Uniform::new(0, tiles.len());
            let mut rng = rand::thread_rng();
            let rand = btwn.sample(&mut rng);
            tiles_shuffled.push(tiles[rand]);
            tiles.remove(rand);
        }

        self.tiles = tiles_shuffled;
    }

    pub fn generate(&mut self) {
        let mut tiles_ordered = Vec::<Tile>::with_capacity(self.total_tiles);
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

pub struct EdgeHolder {
    edges: Vec<Edge>,
    total_edges: usize,
    row_length_horizontal: usize,
    row_count_horizontal: usize,
    row_length_vertical: usize,
    row_count_vertical: usize,

}

impl EdgeHolder {
    pub fn new() -> Self {
        EdgeHolder {
            edges: Vec::<Edge>::new(),
            total_edges: 0,
            row_length_horizontal: 0,
            row_count_horizontal: 0,
            row_length_vertical: 0,
            row_count_vertical: 0,
        }
    }

    /// sets the length of a row for each vector of edges based on the length of a tile row.
    pub fn set_row_length(&mut self, length: usize) {
        self.row_length_horizontal = length * 2 + 1;
        self.row_length_vertical = length + 1;
    }

    /// sets the number of rows for each vector of edges based on the number of tile rows.
    pub fn set_row_count(&mut self, count: usize) {
        self.row_count_horizontal = count + 1;
        self.row_count_vertical = count;
    }

    pub fn setup(&mut self, length: usize, count: usize) {
        self.set_row_length(length);
        self.set_row_count(count);
        // the `-2` is due to the fact the top and bottom rows of edges (which are always horizontal) have 1 less edge in them.
        self.total_edges = self.row_length_horizontal * self.row_count_horizontal + self.row_length_vertical * self.row_count_vertical - 2;
    }

    /// Generates the array. Edges will be utilized such that horizontal and vertical edges are stored "together"
    /// For example, if you had a 3x5 board, there would be 11 horizontal edges per row, and 6 vertical edges per row, with a total of 6 rows. 
    /// The only exception to this is the first and last row of edges; they contain 1 less edge each due to not having a row above/below them.
    pub fn generate(&mut self, length: usize, count: usize) {
        self.set_row_length(length);
        self.set_row_count(count);
        self.edges.reserve(self.total_edges);

        for _ in 0..self.edges.capacity() {
            self.edges.push(Edge::None);
        }
    
    }
    
    pub fn build_road (&mut self, idx: usize) {
        self.edges[idx] = Edge::Road;
    }

    fn calc_row(&self, idx: usize) -> usize {
        // the first row of horizontal edges having 1 less edge than the rest causes another off-by-one. 
        (idx + 1) / (self.row_length_horizontal + self.row_length_vertical)
    }

    fn calc_row_idx(&self, idx: usize) -> usize {
        if self.calc_row(idx) == 0 && idx < self.row_length_horizontal - 1 { return idx };
        let row_length = self.row_length_horizontal + self.row_length_vertical;
        let edge = (idx + 1) % row_length;
        if edge >= self.row_length_horizontal {
            edge - self.row_length_horizontal
        } else {
            edge
        }
    }

    fn is_edge_vertical(&self, idx: usize) -> bool {
        let row_length = self.row_length_horizontal + self.row_length_vertical;
        // the first row of horizontal edges having 1 less edge than the rest causes an off-by-one. 
        (idx + 1) % row_length >= self.row_length_horizontal
    }

    /// this will only generate valid results if the edge is vertical!
    fn find_adjacent_edges(&self, idx: usize) -> [Option<usize>; 4] {
        let row = self.calc_row(idx);
        let row_idx = self.calc_row_idx(idx);
        let mut adjacent_edges: [Option<usize>; 4] = if row % 2 == 0 {
            [
                Some(idx.saturating_sub(self.row_length_horizontal) + row_idx),
                Some(idx.saturating_sub(self.row_length_horizontal - 1) + row_idx),
                Some(idx + self.row_length_vertical + row_idx),
                Some(idx + (self.row_length_vertical + 1) + row_idx)
            ]
        } else {
            [
                Some(idx.saturating_sub(self.row_length_horizontal + 1) + row_idx),
                Some(idx.saturating_sub(self.row_length_horizontal) + row_idx),
                Some(idx + (self.row_length_vertical - 1) + row_idx),
                Some(idx + self.row_length_vertical + row_idx)
            ]
        };
        
        for edge in 0..adjacent_edges.len() {
            // validity check. all of these edges that are adjacent to the vertical edge must be horizontal. if they are not, set to None to indicate it's invalid.
            // this is because not all vertical edges have 4 adjacent edges. some only have 2 or 3, and any edge produced by the method above will be vertical if it is not actually adjacent.
            if self.is_edge_vertical(adjacent_edges[edge].unwrap()) || adjacent_edges[edge].unwrap() >= self.total_edges { 
                adjacent_edges[edge] = None; 
            } 
        }

        adjacent_edges
    }

    /// determine if 2 edges are adjacent
    pub fn is_edge_adjacent(&self, first: usize, second: usize) -> bool {
        if self.is_edge_vertical(first) && self.is_edge_vertical(second) {
            // 2 vertical edges cannot be adjacent.
            return false; 
        } else if !self.is_edge_vertical(first) && !self.is_edge_vertical(second) {
            // if both edges are horizontal, we simply have to check if they are next to each other.
            return first.abs_diff(second) <= 1
        } else {
            // most complex case, where one edge is vertical and one edge is horizontal
            let (vertical_edge, horizontal_edge) = if self.is_edge_vertical(first) { (first, second) } else { (second, first) };
            
            // find all of the edges that border the vertical edge
            for edge in self.find_adjacent_edges(vertical_edge) {
                // check if any of the edges are equal to the horizontal edge. if so, return true!
                match edge {
                    Some(value) => if value == horizontal_edge { return true; } else { continue; },
                    None => continue,
                }
            }

            false
        }
    }

}

pub struct CornerHolder {
    corners: Vec<Corner>,
    row_length: usize,
    row_count: usize,
}

impl CornerHolder {
    pub fn new() -> Self {
        CornerHolder {
            corners: Vec::<Corner>::new(),
            row_length: 0,
            row_count: 0,
        }
    }

    pub fn setup(&mut self, length: usize, count: usize) {
        self.row_length = length * 2 + 2;
        self.row_count = count + 1;
        // we subtract 2 here because there is 1 less corner in the first and last rows (same reason as the edges).
        self.corners.reserve(self.row_length * self.row_count - 2);

        for _ in 0..self.corners.capacity() {
            self.corners.push(Corner::new());
        }
    }

    pub fn set_building(&mut self, idx: usize, building: Option<Building>) {
        self.corners[idx].set_building(building)
    }

    pub fn set_trade(&mut self, idx: usize, trade: Option<TradeType>) {
        self.corners[idx].set_trade(trade)
    }

    pub fn calc_row(&self, idx: usize) -> usize {
        // off by one due to the missing corner in the first row.
        (idx + 1) / self.row_length
    }

    pub fn calc_row_idx(&self, idx: usize) -> usize {
        if self.calc_row(idx) == 0 {
            idx
        } else {
            // off by one due to the missing corner in the first row.
            (idx + 1) % self.row_length
        }
    }

    fn can_traverse_up(&self, idx: usize) -> bool {
        let row = self.calc_row(idx);
        if row == 0 {
            return false;
        } else if row == self.row_count - 1 || row % 2 == 0 {
            return self.calc_row_idx(idx) % 2 == 1;
        } else {
            return self.calc_row_idx(idx) % 2 == 0;
        }
    }

    fn can_traverse_down(&self, idx: usize) -> bool {
        let row = self.calc_row(idx);
        if row == self.row_count - 1 {
            return false;
        } else if row % 2 == 0 {
            return self.calc_row_idx(idx) % 2 == 0;
        } else {
            return self.calc_row_idx(idx) % 2 == 1;
        }
    }

    pub fn calc_distance(&self, first: usize, second: usize) -> usize {
        let (mut current_row, mut current_row_idx, dest_row, dest_row_idx) = if self.calc_row_idx(first) <= self.calc_row_idx(second) {
            (self.calc_row(first), self.calc_row_idx(first), self.calc_row(second), self.calc_row_idx(second))
        } else {
            (self.calc_row(second), self.calc_row_idx(second), self.calc_row(first), self.calc_row_idx(first))
        };
        let mut distance = 0;
        
        // current_row will always be to the left or on top of the destination.
        // we iterate based on where we are on the board until we reach the destination.
        while current_row != dest_row && current_row_idx != dest_row_idx {
            let current_idx = current_row * self.row_length + current_row_idx - if current_row == 0 { 0 } else { 1 };
            let right: i32 = if current_row_idx > dest_row_idx { -1 } else { 1 };
            
            if current_row < dest_row && self.can_traverse_up(current_idx) {
                current_row -= 1;
                distance += 1;
            } else if current_row > dest_row && self.can_traverse_down(current_idx) {
                current_row +=1;
                distance += 1;
            } else {
                current_row_idx += (1 * right) as usize;
                distance += 1;
            } 
        }
        
        distance
    }

}

/// the idea for the board is to have 1 big vector, with each row containing the same amount of hexes.
/// the resource tiles will be placed (currently) according to how they are in the actual game.
pub struct Board {
    board_tiles: Vec<Tile>,
    corners: Vec<Corner>,
    edges_horizontal: Vec<Edge>,
    edges_vertical: Vec<Edge>,
    max_row_length: usize,
    resource_deck: TileDeck,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board_tiles: Vec::<Tile>::new(),
            corners: Vec::<Corner>::new(),
            edges_horizontal: Vec::<Edge>::new(),
            edges_vertical: Vec::<Edge>::new(),
            max_row_length: 0,
            resource_deck: TileDeck::new(),
        }
    }

    fn clear_board(&mut self) {
        self.board_tiles.clear();
        self.corners.clear();
        self.edges_horizontal.clear();
        self.edges_vertical.clear();
        self.max_row_length = 0;
        self.resource_deck.clear();
    }

    // creating the board
    pub fn create_board(&mut self, size: BoardSize) {
        // first, clear the preexisting board
        self.clear_board();

        match size {
            BoardSize::Hexagonal { shortest, longest } => {
                self.max_row_length = longest;

                // abusing math :3
                let total_resource_tiles = longest + longest*(longest-1) - shortest*(shortest-1);

                // generate the resource tiles
                self.resource_deck.create_num_resource_tiles(total_resource_tiles);
                self.resource_deck.generate();

                self.board_tiles.reserve(total_resource_tiles * total_resource_tiles);

                // this is for every row we create.
                for y in 0..self.max_row_length {
                    let mut tile_resource_count = shortest + y;
                    if y > self.max_row_length/2 {
                        tile_resource_count = shortest + self.max_row_length - y - 1;
                    }
                    // tiles first
                    let mut water_tiles = self.max_row_length - tile_resource_count;

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
                    for _ in 0..tile_resource_count {
                        self.board_tiles.push(self.resource_deck.draw());
                    }

                    // add the other half of the water tiles
                    for _ in 0..water_tiles/2 {
                        self.board_tiles.push(Tile::new(TileType::Water));
                    }
                }

                
            }
        }
    }
}