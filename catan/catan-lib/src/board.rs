use hexgrid::{
    corner::{
        bounds::CornerBounds,
        position::{CornerPosition, Height},
        table::CornerTable,
    },
    edge::{
        bounds::EdgeBounds,
        position::{EdgePosition, Valid},
        table::EdgeTable,
    },
    hex::{bounds::HexPerimeter, position::HexPosition, table::HexTable},
};

use crate::{
    game::edition::GameEdition,
    object::{
        CornerData, EdgeData, Robber, TileData, TileType,
        structure::{
            building::{Building, BuildingStore},
            transport::{Transport, TransportStore},
        },
        trade::{TradeStore, TradeType},
    },
};

#[derive(Debug)]
pub struct Board {
    corners: CornerTable<CornerData>,
    edges: EdgeTable<EdgeData>,
    tiles: HexTable<TileData>,
    robber: Robber,
}

impl Board {
    pub fn new(edition: impl GameEdition) -> Self {
        let tiles = Self::create_tiles(&edition);
        let bounds = tiles.get_bounds();
        let corners = Self::create_trades(bounds, &edition);
        let robber = Robber::place(&tiles);
        Board {
            corners,
            edges: EdgeTable::new(EdgeBounds::new(bounds)),
            tiles,
            robber,
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
        self.corners.get_trade(position)
    }

    pub fn get_building<H: Height>(&self, position: CornerPosition<H>) -> Option<Building> {
        self.corners.get_building(position)
    }

    pub fn get_transport<T: Valid>(&self, position: EdgePosition<T>) -> Option<Transport> {
        self.edges.get_transport(position)
    }

    pub fn set_building<H: Height>(
        &mut self,
        position: CornerPosition<H>,
        building: Building,
    ) -> Result<(), ()> {
        self.corners.set_building(position, building)
    }

    pub fn set_transport<T: Valid>(
        &mut self,
        position: EdgePosition<T>,
        transport: Transport,
    ) -> Result<(), ()> {
        self.edges.set_transport(position, transport)
    }

    pub fn move_robber(&mut self, position: HexPosition) {
        self.robber.r#move(position);
    }

    // TODO: Create error handler instead of expecting the position.
    fn create_tiles(edition: &impl GameEdition) -> HexTable<TileData> {
        let mut bounds = HexPerimeter::new();
        let iter = edition.get_tiles();

        for (b, _) in iter.clone() {
            bounds.expand(b);
        }

        let mut tiles = HexTable::new(bounds.clone());

        for (p, t) in iter {
            tiles.set(p, t).expect("HexPosition is out of bounds!")
        }

        tiles
    }

    fn create_trades(bounds: &HexPerimeter, edition: &impl GameEdition) -> CornerTable<CornerData> {
        CornerTable::new(CornerBounds::new(bounds)).with_trades(edition)
    }
}
