use rand::{prelude::*, rngs::ChaCha8Rng};
use serde::{Deserialize, Serialize};
use std::iter;
use tsify::Tsify;

use super::{TileData, TileType};
use crate::{distribution::Distribution, game::GameRng};

pub const RESOURCE_NO: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Tsify, Serialize, Deserialize)]
pub enum ResourceType {
    Wood,
    Brick,
    Wheat,
    Sheep,
    Ore,
}

pub const RESOURCES: [ResourceType; RESOURCE_NO] = [
    ResourceType::Wood,
    ResourceType::Brick,
    ResourceType::Wheat,
    ResourceType::Sheep,
    ResourceType::Ore,
];

#[derive(Debug, Clone, Copy)]
pub struct Resources {
    resources: [ResourceType; RESOURCE_NO],
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: RESOURCES,
        }
    }
}

impl IntoIterator for Resources {
    type Item = ResourceType;
    type IntoIter = std::array::IntoIter<Self::Item, RESOURCE_NO>;

    fn into_iter(self) -> Self::IntoIter {
        self.resources.into_iter()
    }
}

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
        rng: &mut GameRng,
    ) -> Self {
        ResourceDeck {
            resources: Self::create_tiles(size, distribution, roll_numbers, rng),
        }
    }

    fn create_tiles(
        size: usize,
        distribution: ResourceDistribution,
        roll_numbers: &mut impl Iterator<Item = u8>,
        rng: &mut GameRng,
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

        resources.shuffle(rng);

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
