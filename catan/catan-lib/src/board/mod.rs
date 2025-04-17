pub mod iterators;
pub mod game;

use game::GameEdition;
use hexgrid::{corner::{bounds::CornerBounds, position::{CornerPosition, Height}, table::CornerTable}, edge::{bounds::EdgeBounds, position::{EdgePosition, Valid}, table::EdgeTable}, hex::{bounds::HexBounds, position::HexPosition, table::HexTable}};

use crate::objects::{Building, CornerData, EdgeData, TileData, TileType, TradeType, Transport};

pub struct Board {
    corners: CornerTable<CornerData>,
    edges: EdgeTable<EdgeData>,
    tiles: HexTable<TileData>,
}

impl Board {
    pub fn new(edition: impl GameEdition) -> Self {
        let tiles = Self::create_tiles(edition);
        let bounds = tiles.get_bounds();
        let corners = CornerTable::new(CornerBounds::new(bounds));
        Board {
            corners,
            edges: EdgeTable::new(EdgeBounds::new(bounds)),
            tiles,
        }
    }
    
    pub fn get_tile(&self, position: HexPosition) -> TileData {
        if let Some(r) = self.tiles.get(position) {
            *r
        } else {
            TileData::new(TileType::Water)
        }
    }

    pub fn get_trade<H: Height>(&self, position: CornerPosition<H>) -> Option<TradeType> {
        self.corners.get(position)?.get_trade()
    }

    pub fn get_building<H: Height>(&self, position: CornerPosition<H>) -> Option<Building> {
        self.corners.get(position)?.get_building()
    }

    pub fn get_transport<T: Valid>(&self, position: EdgePosition<T>) -> Option<Transport> {
        Some(self.edges.get(position)?.get_transport())
    }

    pub fn set_building<H: Height>(&mut self, position: CornerPosition<H>, building: Building) -> Result<(), ()> {
        if let Some(data) = self.corners.get_mut(position) {
            data.set_building(building);
            Ok(())
        } else {
            let mut data = CornerData::new();
            data.set_building(building);
            self.corners.set(position, data)
        }
    }

    pub fn set_transport<T: Valid>(&mut self, position: EdgePosition<T>, transport: Transport) -> Result<(), ()> {
        self.edges.set(position, EdgeData::new(transport))
    }
    
    fn create_tiles(edition: impl GameEdition) -> HexTable<TileData> {
        let mut bounds = HexBounds::new();
        let iter = edition.get_tiles();

        for (b, _) in iter.clone() {
            bounds.expand_bounds(b);
        }

        let mut tiles = HexTable::new(bounds.clone());

        for (p, t) in iter {
            tiles.set(p, t).expect("HexPosition is out of bounds!")
        }

        tiles
    }

    
}

