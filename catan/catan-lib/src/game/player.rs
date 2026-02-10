use std::sync::atomic::{AtomicU64, Ordering};

use crate::{
    game::{
        hand::Hand,
        structures::{OwnedStructures, StructureType},
    },
    object::resource::ResourceType,
};

static NEXT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnershipToken(u64);

impl OwnershipToken {
    pub fn new() -> Self {
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

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

    pub fn play_structure(&mut self, structure: StructureType) {
        if structure == StructureType::City {
            self.owned_structures
                .add_structure(StructureType::Settlement);
        }
        self.owned_structures.remove_structure(structure);
    }

    pub fn count_resource(&self, resource: ResourceType) -> u32 {
        self.hand.get_resource(resource).get_count()
    }

    fn add_resource(&mut self, resource: ResourceType, count: u32) {
        self.hand.add_resource_card(resource, count);
    }

    fn sub_resource(&mut self, resource: ResourceType, count: u32) {
        self.hand.sub_resource_card(resource, count);
    }
}
