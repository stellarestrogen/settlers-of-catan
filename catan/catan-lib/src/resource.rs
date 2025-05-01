use rand::prelude::*;
use std::iter;

use super::objects::{ResourceType, TileData, TileType};

static RESOURCES: [ResourceType; 5] = [
    ResourceType::Wood,
    ResourceType::Brick,
    ResourceType::Wheat,
    ResourceType::Sheep,
    ResourceType::Ore,
];

#[derive(Clone)]
pub struct ResourceDistribution {
    distribution: [(ResourceType, u32); 5],
}

impl ResourceDistribution {
    pub fn new(distribution: [(ResourceType, u32); 5]) -> Self {
        ResourceDistribution { distribution }
    }

    pub fn for_resource(&self, resource: ResourceType) -> u32 {
        let default = (resource, 0);
        let (_, d) = self
            .distribution
            .iter()
            .find(|(rsrc, _)| rsrc == &resource)
            .unwrap_or(&default);
        *d
    }
}

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
