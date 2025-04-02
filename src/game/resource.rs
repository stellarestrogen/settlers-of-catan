use rand::prelude::*;
use std::iter;

use super::{
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
    pub fn new(size: usize, distribution: ResourceDistribution, roll_numbers: &mut impl Iterator<Item = u8>) -> Self {
        ResourceDeck {
            resources: Self::create_tiles(size, distribution, roll_numbers),
        }
    }

    // pub fn draw(&mut self) -> Tile {
    //     self.resources
    //         .pop()
    //         .expect("No more resources to draw from ResourceDeck!")
    // }

    // pub fn into_vec(self) -> Vec<Tile> {
    //     self.resources
    // }

    fn create_tiles(size: usize, distribution: ResourceDistribution, roll_numbers: &mut impl Iterator<Item = u8>) -> Vec<Tile> {
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


        resources
            .into_iter()
            .map(|resource| {
                resource
                    .map(|rsrc| {
                        Tile::new(TileType::Resource {
                            resource: rsrc,
                            roll_number: roll_numbers.next().unwrap().into(),
                        })
                    })
                    .unwrap_or(Tile::new(TileType::Desert))
            })
            .collect()
            
    }
}

impl Iterator for ResourceDeck {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        self.resources.pop()
    }
}