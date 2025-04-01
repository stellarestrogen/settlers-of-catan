use rand::prelude::*;
use std::iter;

use super::{
    game::GameEdition,
    objects::{ResourceDistribution, ResourceType, TileType},
    tile::Tile,
};

static RESOURCES: [ResourceType; 5] = [
    ResourceType::Wood,
    ResourceType::Brick,
    ResourceType::Wheat,
    ResourceType::Sheep,
    ResourceType::Ore,
];

/// Holds all of the resource and desert tiles.
pub struct ResourceDeck {
    resources: Vec<Tile>,
}

impl ResourceDeck {
    pub fn new(edition: &impl GameEdition, size: usize, distribution: ResourceDistribution, roll_numbers: Vec<u32>) -> Self {
        ResourceDeck {
            resources: Self::create_tiles(edition, size, distribution, roll_numbers),
        }
    }

    pub fn draw(&mut self) -> Tile {
        self.resources
            .pop()
            .expect("No more resources to draw from ResourceDeck!")
    }

    pub fn into_vec(self) -> Vec<Tile> {
        self.resources
    }

    fn create_tiles(edition: &impl GameEdition, size: usize, distribution: ResourceDistribution, roll_numbers: Vec<u32>) -> Vec<Tile> {
        let mut resources =
            Vec::<Option<ResourceType>>::with_capacity(size);
        for resource in RESOURCES {
            resources.extend(iter::repeat_n(
                Some(resource),
                distribution.for_resource(resource) as usize,
            ));
        }
        resources.truncate(size);

        while resources.len() < size {
            resources.push(None);
        }

        resources.shuffle(&mut rand::rng());

        let mut roll_numbers = roll_numbers.into_iter();

        resources
            .into_iter()
            .map(|resource| {
                resource
                    .map(|rsrc| {
                        Tile::new(TileType::Resource {
                            resource: rsrc,
                            roll_number: roll_numbers.next().unwrap(),
                        })
                    })
                    .unwrap_or(Tile::new(TileType::Desert))
            })
            .collect()
    }
}
