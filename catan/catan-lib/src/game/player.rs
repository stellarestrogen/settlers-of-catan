use crate::{
    game::{
        hand::Hand,
        structures::{OwnedStructures, Structure},
    },
    object::resource::ResourceType,
};

pub struct Player {
    hand: Hand,
    owned_structures: OwnedStructures,
}

impl Player {
    pub fn new(owned_structures: OwnedStructures) -> Self {
        Self {
            hand: Hand::new(),
            owned_structures,
        }
    }

    pub fn play_structure(&mut self, structure: Structure) {
        if structure == Structure::City {
            self.owned_structures.add_structure(Structure::Settlement);
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
