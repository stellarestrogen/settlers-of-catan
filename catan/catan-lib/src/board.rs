use hexgrid::{
    corner::{
        bounds::CornerBounds,
        position::{CornerPosition, Height, High, Low},
        table::CornerTable,
    },
    edge::{
        bounds::EdgeBounds,
        position::{EdgePosition, Even, Odd, Positive, Valid},
        table::EdgeTable,
    },
    hex::{bounds::HexPerimeter, position::HexPosition, table::HexTable},
};

use crate::{
    game::edition::GameEdition,
    object::{Building, CornerData, EdgeData, TileData, TileType, Transport, trade::*},
};

pub struct Board {
    corners: CornerTable<CornerData>,
    edges: EdgeTable<EdgeData>,
    tiles: HexTable<TileData>,
}

impl Board {
    pub fn new(edition: impl GameEdition) -> Self {
        let tiles = Self::create_tiles(&edition);
        let bounds = tiles.get_bounds();
        let corners = CornerTable::new(CornerBounds::new(bounds));
        let mut board = Board {
            corners,
            edges: EdgeTable::new(EdgeBounds::new(bounds)),
            tiles,
        };

        board.create_trades(&edition);

        board
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

    pub fn set_building<H: Height>(
        &mut self,
        position: CornerPosition<H>,
        building: Building,
    ) -> Result<(), ()> {
        if let Some(data) = self.corners.get_mut(position) {
            data.set_building(building);
            Ok(())
        } else {
            let mut data = CornerData::new();
            data.set_building(building);
            self.corners.set(position, data)
        }
    }

    pub fn set_transport<T: Valid>(
        &mut self,
        position: EdgePosition<T>,
        transport: Transport,
    ) -> Result<(), ()> {
        self.edges.set(position, EdgeData::new(transport))
    }

    fn set_trade<H: Height>(
        &mut self,
        position: CornerPosition<H>,
        trade: TradeType,
    ) -> Result<(), ()> {
        if let Some(data) = self.corners.get_mut(position) {
            data.set_trade(trade);
            Ok(())
        } else {
            let mut data = CornerData::new();
            data.set_trade(trade);
            self.corners.set(position, data)
        }
    }

    fn set_trades(&mut self, trade_port: TradePort) -> Result<(), ()> {
        let (p1, p2) = trade_port.get_positions();
        let trade = trade_port.get_type();

        self.set_trade(p1, trade)?;
        self.set_trade(p2, trade)?;
        Ok(())
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

    // TODO: Create error handler instead of expecting the position.
    fn create_trades(&mut self, edition: &impl GameEdition) {
        let trades = edition.get_trades();
        for trade in trades.into_iter() {
            self.set_trades(trade)
                .expect("CornerPosition is out of bounds!");
        }
    }
}

struct PlayedStructures {
    settlements: Vec<(Option<CornerPosition<Low>>, Option<CornerPosition<High>>)>,
    cities: Vec<(Option<CornerPosition<Low>>, Option<CornerPosition<High>>)>,
    roads: Vec<(
        Option<EdgePosition<Even>>,
        Option<EdgePosition<Odd>>,
        Option<EdgePosition<Positive>>,
    )>,
    boats: Vec<(
        Option<EdgePosition<Even>>,
        Option<EdgePosition<Odd>>,
        Option<EdgePosition<Positive>>,
    )>,
}

impl PlayedStructures {
    pub fn new() -> Self {
        PlayedStructures {
            settlements: Vec::new(),
            cities: Vec::new(),
            roads: Vec::new(),
            boats: Vec::new(),
        }
    }

    pub fn build_settlement<H: Height>(&mut self, position: CornerPosition<H>) {
        self.settlements
            .push((position.as_low(), position.as_high()))
    }

    /// Tries to find a settlement at the position. If one does not exist, returns Err(()). Otherwise, replace it with a city.
    pub fn build_city<H: Height>(&mut self, position: CornerPosition<H>) -> Result<(), ()> {
        let settlement = self
            .settlements
            .iter()
            .position(|(p1, p2)| &position.as_low() == p1 && &position.as_high() == p2)
            .ok_or(())?;
        self.settlements.swap_remove(settlement);
        self.cities.push((position.as_low(), position.as_high()));
        Ok(())
    }

    pub fn build_road<T: Valid>(&mut self, position: EdgePosition<T>) {
        self.roads.push((
            position.as_even(),
            position.as_odd(),
            position.as_positive(),
        ))
    }

    pub fn build_boat<T: Valid>(&mut self, position: EdgePosition<T>) {
        self.boats.push((
            position.as_even(),
            position.as_odd(),
            position.as_positive(),
        ))
    }
}
