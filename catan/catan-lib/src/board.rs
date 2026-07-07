use std::fmt::Debug;

use hexgrid::{
    corner::{bounds::CornerBounds, position::CornerPosition, table::CornerTable},
    edge::{bounds::EdgeBounds, position::EdgePosition, table::EdgeTable},
    hex::{bounds::HexBounds, position::HexPosition, table::HexTable},
};

use crate::{
    game::{GameRng, edition::GameEdition},
    object::{
        CornerInfo, EdgeInfo, Robber, TileData, TileType,
        resource::ResourceType,
        structure::{
            building::{Building, BuildingStore},
            transport::{Transport, TransportStore},
        },
        trade::{TradePort, TradePortDeck, TradeStore, TradeType},
    },
};

#[derive(Debug)]
pub struct Board {
    tiles: HexTable<TileData>,
    corners: CornerTable<CornerInfo>,
    edges: EdgeTable<EdgeInfo>,
    robber: Robber,
    trade_ports: TradePortDeck,
}

impl Board {
    pub fn new(edition: impl GameEdition, rng: &mut GameRng) -> Self {
        let tiles = Self::create_tiles(&edition, rng);
        let bounds = tiles.get_bounds();
        let corners = Self::create_trades(bounds, &edition, rng);
        let edges = EdgeTable::new(EdgeBounds::new(bounds));
        let robber = Robber::place(&tiles);
        let trade_ports = edition.get_trades(rng).collect();
        Board {
            tiles,
            corners,
            edges,
            robber,
            trade_ports,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.tiles.get_bounds().get_width()
    }

    pub fn get_height(&self) -> u32 {
        self.tiles.get_bounds().get_height()
    }

    pub fn get_offset(&self) -> HexPosition {
        self.tiles.get_bounds().get_top_left()
    }

    pub fn get_tile(&self, position: HexPosition) -> TileData {
        if let Some(r) = self.tiles.get(position) {
            *r
        } else {
            TileData::new(TileType::Water)
        }
    }

    pub fn get_tile_data(&self) -> impl Iterator<Item = TileData> {
        self.tiles.positions().map(|p| self.get_tile(p))
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

    pub fn trades(&self) -> impl Iterator<Item = TradeType> {
        self.corners.get_trades()
    }

    pub fn trade_ports(&self) -> impl Iterator<Item = TradePort> {
        self.trade_ports.clone()
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

    pub fn neighboring_edges_for_corner(
        &self,
        position: CornerPosition,
    ) -> impl Iterator<Item = EdgePosition> + Clone {
        position
            .neighboring_edges()
            .into_iter()
            .filter(|p| self.edges.get_bounds().contains(*p))
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

    pub fn neighboring_hex_for_edge(
        &self,
        position: EdgePosition,
    ) -> impl Iterator<Item = HexPosition> + Clone {
        position
            .neighboring_hex()
            .into_iter()
            .filter(|p| self.tiles.get_bounds().contains(*p))
    }

    pub fn neighboring_corners_for_edge(
        &self,
        position: EdgePosition,
    ) -> impl Iterator<Item = CornerPosition> + Clone {
        position
            .neighboring_corners()
            .into_iter()
            .filter(|p| self.corners.get_bounds().contains(*p))
    }

    pub fn neighboring_edges(
        &self,
        position: EdgePosition,
    ) -> impl Iterator<Item = EdgePosition> + Clone + Debug {
        position
            .neighboring_edges()
            .into_iter()
            .filter(|p| self.edges.get_bounds().contains(*p))
    }

    fn create_tiles(edition: &impl GameEdition, rng: &mut GameRng) -> HexTable<TileData> {
        let mut bounds = HexBounds::new();
        let iter = edition.get_tiles(rng);

        for (b, _) in iter.clone() {
            bounds.expand(b);
        }

        let mut tiles = HexTable::new(bounds.clone());

        for (p, t) in iter {
            tiles.set(p, t).expect("HexPosition is out of bounds!")
        }
        tiles
    }

    fn create_trades(
        bounds: &HexBounds,
        edition: &impl GameEdition,
        rng: &mut GameRng,
    ) -> CornerTable<CornerInfo> {
        CornerTable::new(CornerBounds::new(bounds)).with_trades(edition, rng)
    }
}
