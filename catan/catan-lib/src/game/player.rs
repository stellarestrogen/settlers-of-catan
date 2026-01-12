use hexgrid::corner::position::{CornerPosition, Height};

use crate::{
    game::{
        hand::Hand,
        structures::{OwnedStructures, PlayedStructures},
    },
    object::{Structure, resource::{RESOURCES, ResourceType}},
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
        for rsrc in RESOURCES {
            if self.count_resource(rsrc) >= structure.resource_cost(rsrc) {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }

    pub fn try_build_structure<H: Height>(&mut self, structure: Structure, position: CornerPosition<H>) -> Result<(), ()> {
        self.can_build_structure(structure).then_some(()).ok_or(())?;
        self.owned_structures.remove_structure(structure)?;
        for rsrc in RESOURCES {
            self.sub_resource(rsrc, structure.resource_cost(rsrc));
        }
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
