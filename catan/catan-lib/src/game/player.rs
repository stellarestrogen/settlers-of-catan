use crate::{
    game::{
        hand::Hand,
        structures::{OwnedStructures, PlayedStructures},
    },
    object::{Structure, resource::ResourceType},
};

pub struct Player {
    hand: Hand,
    owned_structures: OwnedStructures,
    played_structures: PlayedStructures,
}

impl Player {
    pub fn new(owned_structures: OwnedStructures) -> Self {
        Self {
            hand: Hand::new(),
            owned_structures,
            played_structures: PlayedStructures::new(),
        }
    }

    pub fn can_build_structure(&self, structure: Structure) -> bool {
        match structure {
            Structure::Settlement => {
                self.count_resource(ResourceType::Wood) >= 1
                    && self.count_resource(ResourceType::Brick) >= 1
                    && self.count_resource(ResourceType::Wheat) >= 1
                    && self.count_resource(ResourceType::Sheep) >= 1
            }
            Structure::City => {
                self.count_resource(ResourceType::Wheat) >= 2
                    && self.count_resource(ResourceType::Ore) >= 3
            }
            Structure::Road => {
                self.count_resource(ResourceType::Wood) >= 1
                    && self.count_resource(ResourceType::Brick) >= 1
            }
            Structure::Boat => {
                self.count_resource(ResourceType::Wood) >= 1
                    && self.count_resource(ResourceType::Sheep) >= 1
            }
        }
    }

    pub fn count_resource(&self, resource: ResourceType) -> u32 {
        self.hand.get_resource(resource).get_count()
    }
}
