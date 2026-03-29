use hexgrid::edge::position::EdgePosition;

use crate::game::player::OwnershipToken;

pub struct TransportSegment {
    history: Vec<EdgePosition>,
    current: EdgePosition,
    owner: OwnershipToken,
}

impl TransportSegment {
    pub fn new(owner: OwnershipToken, position: EdgePosition) -> Self {
        Self {
            history: Vec::new(),
            current: position,
            owner,
        }
    }

    pub fn update(&mut self, position: EdgePosition) {
        self.history.push(self.current);
        self.current = position;
    }

    pub fn is_in_history(&self, position: EdgePosition) -> bool {
        self.history.iter().find(|p| **p == position).is_some()
    }

    pub fn owner(&self) -> OwnershipToken {
        self.owner
    }
}
