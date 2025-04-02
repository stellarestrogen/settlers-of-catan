use std::{
    iter,
    ops::{Index, IndexMut},
};

use crate::game::objects::TileType;

use super::{bounds::TileBounds, position::TilePosition, Tile};

/// Holds the entire "game board" tiles, including water tiles.
pub struct TileHolder {
    tiles: Vec<Tile>,
    bounds: TileBounds
}

impl TileHolder {
    pub fn new(bounds: TileBounds) -> Self {
        let mut tiles = Vec::with_capacity(bounds.get_size());
        tiles.extend(iter::repeat_n(Tile::new(TileType::Water), bounds.get_size()));
        TileHolder {
            tiles: tiles,
            bounds
        }
    }

    fn calc_index(&self, position: TilePosition) -> Option<usize> {
        if !self.bounds.check_bounds(position) {
            return None
        }

        let rights: isize = position
            .horizontal_distance(self.bounds.get_top_left())
            .ceil()
            .try_into()
            .ok()?;

        let downs: isize = position
            .vertical_distance(self.bounds.get_top_left())
            .try_into()
            .ok()?;

        let length: isize = self.bounds.get_length().try_into().ok()?;

        downs.checked_mul(length)?
            .checked_add(rights)?
            .try_into()
            .ok()
    }

    pub fn get(&self, position: TilePosition) -> Option<&Tile> {
        Some(&self.tiles[self.calc_index(position)?])
    }

    pub fn get_mut(&mut self, position: TilePosition) -> Option<&mut Tile> {
        let idx = self.calc_index(position)?;
        Some(&mut self.tiles[idx])
    }

    pub fn get_bounds(&self) -> &TileBounds {
        &self.bounds
    }
}

impl Index<TilePosition> for TileHolder {
    type Output = Tile;

    fn index(&self, index: TilePosition) -> &Tile {
        self.get(index).expect("Indexed TileHolder out of bounds!")
    }
}

impl IndexMut<TilePosition> for TileHolder {
    fn index_mut(&mut self, index: TilePosition) -> &mut Tile {
        self.get_mut(index)
            .expect("Indexed TileHolder out of bounds!")
    }
}
