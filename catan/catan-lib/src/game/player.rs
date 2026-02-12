use std::sync::atomic::{AtomicU64, Ordering};

use crate::{
    game::{error::BuildError, hand::Hand},
    object::{
        card::ResourceCard,
        resource::{ResourceType, Resources},
        structure::{OwnedStructures, StructureType},
    },
};

static NEXT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnershipToken(u64);

impl OwnershipToken {
    pub fn new() -> Self {
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug)]
pub struct Player {
    hand: Hand,
    owned_structures: OwnedStructures,
    token: OwnershipToken,
}

impl Player {
    pub fn new(owned_structures: OwnedStructures) -> Self {
        Self {
            hand: Hand::new(),
            owned_structures,
            token: OwnershipToken::new(),
        }
    }

    pub fn token(&self) -> OwnershipToken {
        self.token
    }

    pub fn try_play_structure(&self, structure: StructureType) -> Result<(), BuildError> {
        if self.owned_structures.get_structure(structure) == 0 {
            return Err(BuildError::NoStructures {
                token: self.token,
                structure,
            });
        }

        let mut insufficient_resources = Vec::<ResourceCard>::new();

        for resource in Resources::new() {
            if self.count_resource(resource) < structure.resource_cost(resource) {
                insufficient_resources.push(self.hand.get_resource(resource));
            }
        }

        if !insufficient_resources.is_empty() {
            return Err(BuildError::InsufficientResources {
                structure,
                insufficient_resources,
            });
        }

        Ok(())
    }

    pub fn play_structure(&mut self, structure: StructureType) -> Result<(), BuildError> {
        self.try_play_structure(structure)?;

        if structure == StructureType::City {
            self.owned_structures
                .add_structure(StructureType::Settlement);
        }
        self.owned_structures.remove_structure(structure);

        Ok(())
    }

    pub fn count_resource(&self, resource: ResourceType) -> u32 {
        self.hand.get_resource(resource).get_count()
    }

    pub fn add_resource(&mut self, resource: ResourceType, count: u32) {
        self.hand.add_resource_card(resource, count);
    }

    pub fn sub_resource(&mut self, resource: ResourceType, count: u32) {
        self.hand.sub_resource_card(resource, count);
    }
}
