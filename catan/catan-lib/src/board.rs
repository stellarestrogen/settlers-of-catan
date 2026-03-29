use hexgrid::{
    corner::{bounds::CornerBounds, position::CornerPosition, table::CornerTable},
    edge::{bounds::EdgeBounds, position::EdgePosition, table::EdgeTable},
    hex::{bounds::HexPerimeter, position::HexPosition, table::HexTable},
};

use crate::{
    game::edition::GameEdition,
    object::{
        CornerData, EdgeData, Robber, TileData, TileType,
        resource::ResourceType,
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

    pub fn get_resource_type(&self, position: HexPosition) -> Option<ResourceType> {
        self.get_tile(position).get_tile_type().get_resource_type()
    }

    pub fn get_tile_roll_number(&self, position: HexPosition) -> Option<u32> {
        self.get_tile(position).get_roll_number()
    }

    pub fn get_trade(&self, position: CornerPosition) -> Option<TradeType> {
        self.corners.get_trade(position)
    }

    pub fn get_building(&self, position: CornerPosition) -> Option<Building> {
        self.corners.get_building(position)
    }

    pub fn set_building(&mut self, building: Building, position: CornerPosition) -> Result<(), ()> {
        self.corners.set_building(position, building)
    }

    pub fn buildings(&self) -> impl Iterator<Item = Building> {
        self.corners.buildings()
    }

    pub fn get_transport(&self, position: EdgePosition) -> Option<Transport> {
        self.edges.get_transport(position)
    }

    pub fn set_transport(
        &mut self,
        transport: Transport,
        position: EdgePosition,
    ) -> Result<(), ()> {
        self.edges.set_transport(position, transport)
    }

    pub fn transports(&self) -> impl Iterator<Item = Transport> {
        self.edges.transports()
    }

    pub fn move_robber(&mut self, position: HexPosition) {
        self.robber.r#move(position);
    }

    pub fn has_robber(&self, position: HexPosition) -> bool {
        self.robber.position() == position
    }

    pub fn neighboring_hex_for_corner(
        &self,
        position: CornerPosition,
    ) -> impl Iterator<Item = HexPosition> + Clone {
        position
            .neighboring_hex()
            .into_iter()
            .filter(|p| self.tiles.get_bounds().contains(*p))
    }

    pub fn neighboring_corners(
        &self,
        position: CornerPosition,
    ) -> impl Iterator<Item = CornerPosition> + Clone {
        position
            .neighboring_corners()
            .into_iter()
            .filter(|p| self.corners.get_bounds().contains(*p))
    }

    pub fn neighboring_edges(
        &self,
        position: EdgePosition,
    ) -> impl Iterator<Item = EdgePosition> + Clone {
        position
            .neighboring_edges()
            .into_iter()
            .filter(|p| self.edges.get_bounds().contains(*p))
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
