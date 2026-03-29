use hexgrid::edge::position::EdgePosition;

use crate::game::player::OwnershipToken;

#[derive(Debug, Clone)]
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

    pub fn next_positions(
        &self,
        neighbors: impl Iterator<Item = EdgePosition> + Clone,
    ) -> impl Iterator<Item = EdgePosition> + Clone{
        neighbors.filter(|p| {
            self.history.iter().find(|h| **h == *p).is_none() && self.is_position_behind_current(*p)
        })
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

    pub fn current_position(&self) -> EdgePosition {
        self.current
    }

    fn is_position_behind_current(&self, position: EdgePosition) -> bool {
        position
            .neighboring_edges()
            .into_iter()
            .find(|p| Some(*p) == self.history.last().copied())
            .is_some()
    }
}
