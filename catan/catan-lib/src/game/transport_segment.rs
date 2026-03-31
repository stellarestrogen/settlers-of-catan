use hexgrid::edge::position::EdgePosition;

use crate::game::player::OwnershipToken;

#[derive(Debug, Clone)]
pub struct TransportSegment {
    history: Vec<EdgePosition>,
    current: EdgePosition,
    owner: OwnershipToken,
    finished_advancing: bool,
}

impl TransportSegment {
    pub fn new(owner: OwnershipToken, position: EdgePosition) -> Self {
        Self {
            history: Vec::new(),
            current: position,
            owner,
            finished_advancing: false,
        }
    }

    pub fn next_positions(
        &self,
        neighbors: impl Iterator<Item = EdgePosition> + Clone,
    ) -> impl Iterator<Item = EdgePosition> + Clone {
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

    pub fn history_overlap(&self, other: &Self) -> u32 {
        let mut overlap = 0;

        for (first, second) in self.history.iter().zip(other.history.iter()) {
            if *first == *second {
                overlap += 1;
            }
        }

        overlap
    }

    pub fn owner(&self) -> OwnershipToken {
        self.owner
    }

    pub fn current_position(&self) -> EdgePosition {
        self.current
    }

    pub fn finished(&mut self) {
        self.finished_advancing = true;
    }

    pub fn is_finished(&self) -> bool {
        self.finished_advancing
    }

    fn is_position_behind_current(&self, position: EdgePosition) -> bool {
        if let Some(last) = self.history.last() {
            position.is_neighbor(*last)
        } else {
            false
        }
    }
}
