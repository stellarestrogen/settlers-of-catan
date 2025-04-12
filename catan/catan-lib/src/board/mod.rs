pub mod draw_iters;
pub mod game;

use game::GameEdition;
use hexgrid::{corner::{bounds::CornerBounds, table::CornerTable}, edge::{bounds::EdgeBounds, table::EdgeTable}, hex::{bounds::HexBounds, table::HexTable}};

use crate::objects::{Corner, Edge, Tile};

pub struct Board {
    corners: CornerTable<Corner>,
    edges: EdgeTable<Edge>,
    tiles: HexTable<Tile>,
}

impl Board {
    pub fn new(edition: impl GameEdition) -> Self {
        let tiles = Self::create_tiles(edition);
        let bounds = tiles.get_bounds();
        Board {
            corners: CornerTable::new(CornerBounds::new(bounds)),
            edges: EdgeTable::new(EdgeBounds::new(bounds)),
            tiles,
        }
    }

    fn fill_tiles(tiles: &mut HexTable<Tile>) {
        
    }

    fn create_tiles(edition: impl GameEdition) -> HexTable<Tile> {
        let mut bounds = HexBounds::new();
        let iter = edition.get_tiles();

        for (b, _) in iter.clone() {
            bounds.expand_bounds(b);
        }

        let mut tiles = HexTable::new(bounds);

        for (p, t) in iter {
            tiles.set(p, t).expect("HexPosition is out of bounds!")
        }

        tiles
    }
}