use super::{
    corner::{
        bounds::CornerBounds, 
        holder::CornerHolder, 
        position::CornerPosition
    }, edge::{bounds::EdgeBounds, holder::EdgeHolder, position::EdgePosition}, game::GameEdition, position::Position, tile::{
        bounds::TileBounds, 
        holder::TileHolder, 
        position::TilePosition
    }
};

pub struct Board {
    corners: CornerHolder,
    edges: EdgeHolder,
    tiles: TileHolder,
}

impl Board {
    pub fn new(edition: impl GameEdition) -> Self {
        let tiles = Self::create_tiles(edition);
        let bounds = tiles.get_bounds();
        Board {
            corners: CornerHolder::new(Self::create_corner_bounds(bounds)),
            edges: EdgeHolder::new(Self::create_edge_bounds(bounds)),
            tiles,
        }
    }

    fn create_tiles(edition: impl GameEdition) -> TileHolder {
        let mut bounds = TileBounds::new();
        let iter = edition.get_tiles();

        for (b, _) in iter.clone() {
            bounds.expand_bounds(b);
        }

        let mut tiles = TileHolder::new(bounds);

        for (p, t) in iter {
            tiles[p] = t;
        }

        tiles
    }

    fn create_corner_bounds(bounds: &TileBounds) -> CornerBounds {
        let shift_top_left: f64 = bounds.get_top_left().horizontal_distance(TilePosition::ORIGIN).into();
        let top_left = CornerPosition::RIGHT * ((shift_top_left * 2.) as i32) + CornerPosition::DOWN * bounds.get_top_left().vertical_distance(TilePosition::ORIGIN);

        let shift_bottom_right: f64 = bounds.get_bottom_right().horizontal_distance(TilePosition::ORIGIN).into();
        let bottom_right = CornerPosition::RIGHT * ((shift_bottom_right * 2.) as i32) + CornerPosition::DOWN * bounds.get_bottom_right().vertical_distance(TilePosition::ORIGIN);

        CornerBounds::new(top_left, bottom_right)
    }

    fn create_edge_bounds(bounds: &TileBounds) -> EdgeBounds {
        let shift_top_left: f64 = bounds.get_top_left().horizontal_distance(TilePosition::ORIGIN).into();
        let top_left = EdgePosition::RIGHT * ((shift_top_left* 4.) as i32) + EdgePosition::DOWN * (bounds.get_top_left().vertical_distance(TilePosition::ORIGIN));

        let shift_bottom_right: f64 = bounds.get_bottom_right().horizontal_distance(TilePosition::ORIGIN).into();
        let bottom_right = EdgePosition::RIGHT * ((shift_bottom_right * 4.) as i32) + EdgePosition::DOWN * (bounds.get_bottom_right().vertical_distance(TilePosition::ORIGIN));

        EdgeBounds::new(top_left.expect("Something has gone horribly wrong..."), bottom_right.expect("Something has gone horribly wrong..."))
    }
}