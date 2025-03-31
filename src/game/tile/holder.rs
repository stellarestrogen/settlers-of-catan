use std::{
    iter,
    ops::{Index, IndexMut},
};

use crate::game::objects::TileType;

use super::{Tile, position::TilePosition};

/// Holds the entire "game board" tiles, including water tiles.
pub struct TileHolder {
    tiles: Vec<Tile>,
    length: u32,
    width: u32,
}

impl TileHolder {
    pub fn new(length: u32, width: u32) -> Self {
        let mut tiles = Vec::with_capacity((length * width) as usize);
        tiles.extend(iter::repeat_n(
            Tile::new(TileType::Water),
            (length * width) as usize,
        ));

        TileHolder {
            tiles,
            length,
            width,
        }
    }

    fn calc_index(&self, position: TilePosition) -> Option<usize> {
        let rights: isize = position
            .horizontal_distance(TilePosition::ORIGIN)
            .ceil()
            .try_into()
            .ok()?;

        let downs: isize = position
            .vertical_distance(TilePosition::ORIGIN)
            .try_into()
            .ok()?;

        let length: isize = self.length.try_into().ok()?;
        let width: isize = self.width.try_into().ok()?;

        if rights < length && downs < width && rights >= 0 && downs >= 0 {
            downs
                .checked_mul(length)?
                .checked_add(rights)?
                .try_into()
                .ok()
        } else {
            None
        }
    }

    pub fn get(&self, position: TilePosition) -> Option<&Tile> {
        Some(&self.tiles[self.calc_index(position)?])
    }

    pub fn get_mut(&mut self, position: TilePosition) -> Option<&mut Tile> {
        let idx = self.calc_index(position)?;
        Some(&mut self.tiles[idx])
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
