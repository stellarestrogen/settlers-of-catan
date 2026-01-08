use crate::game::{
    hand::Hand,
    structures::{OwnedStructures, PlayedStructures},
};

pub struct Player {
    hand: Hand,
    owned_structures: OwnedStructures,
    played_structures: PlayedStructures,
}

impl Player {}
