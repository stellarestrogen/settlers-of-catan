use std::iter;
use rand::prelude::*;

use super::{game::GameEdition, objects::{ResourceType, TileType}, tile::Tile};

static RESOURCES: [ResourceType; 5] = [ResourceType::Wood, ResourceType::Brick, ResourceType::Wheat, ResourceType::Sheep, ResourceType::Ore];

/// Holds all of the resource and desert tiles.
pub struct ResourceDeck {
    resources: Vec<Tile>
}

impl ResourceDeck {
    pub fn new(edition: impl GameEdition) -> Self {
        ResourceDeck {
            resources: Self::create_tiles(edition)
        }
    }

    pub fn draw(&mut self) -> Option<Tile> {
        self.resources.pop()
    }

    pub fn into_vec(self) -> Vec<Tile> {
        self.resources
    }

    fn create_tiles(edition: impl GameEdition) -> Vec<Tile> {
        let mut resources = Vec::<Option<ResourceType>>::with_capacity(edition.get_board_size() as usize);
        let distr = edition.get_resource_distribution();
        for resource in RESOURCES {
            resources.extend(iter::repeat_n(Some(resource), distr.for_resource(resource) as usize));
        }
        resources.truncate(edition.get_board_size() as usize);

        while resources.len() < edition.get_board_size() as usize {
            resources.push(None);
        }

        resources.shuffle(&mut rand::rng());

        let mut roll_numbers = edition.get_roll_numbers().into_iter();

        resources.into_iter()
            .map(|resource| resource.map(|rsrc| Tile::new(TileType::Resource { resource: rsrc, roll_number: roll_numbers.next().unwrap() } ))
            .unwrap_or(Tile::new(TileType::Desert)))
        .collect()
    }
}