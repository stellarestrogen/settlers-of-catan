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

    pub fn buildings(&self) -> impl Iterator<Item = Building> {
        self.corners.buildings()
    }

    pub fn get_transport<T: Valid>(&self, position: EdgePosition<T>) -> Option<Transport> {
        self.edges.get_transport(position)
    }

    pub fn transports(&self) -> impl Iterator<Item = Transport> {
        self.edges.transports()
    }

    pub fn set_building<H: Height>(
        &mut self,
        building: Building,
        position: CornerPosition<H>,
    ) -> Result<(), ()> {
        self.corners.set_building(position, building)
    }

    pub fn set_transport<T: Valid>(
        &mut self,
        transport: Transport,
        position: EdgePosition<T>,
    ) -> Result<(), ()> {
        self.edges.set_transport(position, transport)
    }

    pub fn move_robber(&mut self, position: HexPosition) {
        self.robber.r#move(position);
    }

    pub fn neighboring_hex_for_corner<H: Height>(
        &self,
        position: CornerPosition<H>,
    ) -> impl Iterator<Item = HexPosition> {
        let mut tiles: Vec<HexPosition> = Vec::with_capacity(3);
        for t in position.neighboring_hex().into_iter() {
            if self.tiles.get_bounds().contains(t) {
                tiles.push(t)
            }
        }
        tiles.into_iter()
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
