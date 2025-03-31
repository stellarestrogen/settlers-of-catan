use super::{
    corner::CornerHolder, edge::EdgeHolder, game::GameEdition, resource::ResourceDeck,
    tile::holder::TileHolder,
};

pub struct Board {
    corners: CornerHolder,
    edges: EdgeHolder,
    tiles: TileHolder,
}

impl Board {
    pub fn new(edition: impl GameEdition) -> Self {
        let length_corner = edition.get_board_length() * 2 + 2;
        let width_corner = edition.get_board_width() + 1;

        let length_edge = edition.get_board_length() * 2 + 1;
        let width_edge = edition.get_board_width() + 1;

        Board {
            corners: CornerHolder::new(length_corner, width_corner),
            edges: EdgeHolder::new(length_edge, width_edge),
            tiles: Self::create_tiles(edition),
        }
    }

    fn create_tiles(edition: impl GameEdition) -> TileHolder {
        let mut tiles = TileHolder::new(edition.get_board_length(), edition.get_board_width());
        let mut resource_deck = ResourceDeck::new(&edition);
        let mut iter = edition.get_tile_draw_iter();

        while let Some(p) = iter.next() {
            tiles[p] = resource_deck.draw();
        }

        tiles
    }
}
