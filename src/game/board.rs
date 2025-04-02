use super::{
    corner::{
        bounds::CornerBounds, 
        holder::CornerHolder, 
        position::CornerPosition
    }, 
    edge::EdgeHolder, 
    game::GameEdition, 
    tile::{
        bounds::TileBounds, 
        holder::TileHolder, 
        position::TilePosition
    },
};

pub struct Board {
    tiles: TileHolder,
    corners: CornerHolder,
    edges: EdgeHolder,
}

impl Board {
    pub fn new(edition: impl GameEdition) -> Self {
        let tiles = Self::create_tiles(edition);
        Board {
            tiles,
            corners: CornerHolder::new(Self::create_corner_bounds(tiles.get_bounds())),
            edges: 
        }
    }

    fn create_tiles(edition: impl GameEdition) -> TileHolder {
        let mut bounds = TileBounds::new();
        let mut iter = edition.get_tiles();

        while let Some((p, _)) = iter.next() {
            bounds.expand_bounds(p);
        }

        let mut tiles = TileHolder::new(bounds);
        let mut iter = edition.get_tiles();

        while let Some((p, t)) = iter.next() {
            tiles[p] = t;
        }

        tiles
    }

    fn create_corner_bounds(bounds: &TileBounds) -> CornerBounds {
        let top_left = CornerPosition::LEFT * (bounds.get_top_left().horizontal_distance(TilePosition::ORIGIN).ceil().abs() + 1) + CornerPosition::UP * bounds.get_top_left().vertical_distance(TilePosition::ORIGIN).abs();
        let bottom_right = CornerPosition::RIGHT * (bounds.get_bottom_right().horizontal_distance(TilePosition::ORIGIN).ceil().abs() + 2) + CornerPosition::DOWN * bounds.get_bottom_right().vertical_distance(TilePosition::ORIGIN).abs();
        CornerBounds::new(top_left, bottom_right)
    }

    fn create_edge_bounds(bounds: &TileBounds) -> EdgeBounds {

    }
}
