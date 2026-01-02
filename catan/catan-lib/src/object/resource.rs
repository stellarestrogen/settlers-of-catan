use rand::prelude::*;
use std::iter;

use super::{TileData, TileType};
use crate::distribution::Distribution;

pub const RESOURCE_NO: usize = 5;

#[derive(Clone, Copy, PartialEq)]
pub enum ResourceType {
    Wood,
    Brick,
    Wheat,
    Sheep,
    Ore,
}

pub static RESOURCES: [ResourceType; RESOURCE_NO] = [
    ResourceType::Wood,
    ResourceType::Brick,
    ResourceType::Wheat,
    ResourceType::Sheep,
    ResourceType::Ore,
];

pub type ResourceDistribution = Distribution<ResourceType, RESOURCE_NO>;

/// Holds all of the resource and desert tiles.
#[derive(Clone)]
pub struct ResourceDeck {
    resources: Vec<TileData>,
}

impl ResourceDeck {
    pub fn new(
        size: usize,
        distribution: ResourceDistribution,
        roll_numbers: &mut impl Iterator<Item = u8>,
    ) -> Self {
        ResourceDeck {
            resources: Self::create_tiles(size, distribution, roll_numbers),
        }
    }

    fn create_tiles(
        size: usize,
        distribution: ResourceDistribution,
        roll_numbers: &mut impl Iterator<Item = u8>,
    ) -> Vec<TileData> {
        let mut resources = Vec::<Option<ResourceType>>::with_capacity(size);
        for resource in RESOURCES {
            resources.extend(iter::repeat_n(
                Some(resource),
                distribution.for_obj(resource) as usize,
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
                        TileData::new(TileType::Resource {
                            resource: rsrc,
                            roll_number: roll_numbers.next().unwrap().into(),
                        })
                    })
                    .unwrap_or(TileData::new(TileType::Desert))
            })
            .collect()
    }
}

impl Iterator for ResourceDeck {
    type Item = TileData;

    fn next(&mut self) -> Option<TileData> {
        self.resources.pop()
    }
}
