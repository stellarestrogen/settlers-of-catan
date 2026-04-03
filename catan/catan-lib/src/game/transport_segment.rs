use std::fmt::Debug;

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

    pub fn from_history(owner: OwnershipToken, history: Vec<EdgePosition>) -> Self {
        let current = *history.last().unwrap();
        Self {
            history,
            current,
            owner,
            finished_advancing: true,
        }
    }

    pub fn next_positions(
        &self,
        neighbors: impl Iterator<Item = EdgePosition> + Clone + Debug,
    ) -> impl Iterator<Item = EdgePosition> + Clone + Debug {
        neighbors.filter(|p| {
            !self.is_in_history(*p) && !self.is_position_behind_current(*p)
        })
    }

    pub fn update(&mut self, position: EdgePosition) {
        self.history.push(self.current);
        self.current = position;
    }

    pub fn history(&self) -> Vec<EdgePosition> {
        self.history.clone()
    }

    pub fn is_in_history(&self, position: EdgePosition) -> bool {
        self.history.iter().find(|p| **p == position).is_some()
    }

    pub fn history_overlap(&self, other: &Self) -> impl Iterator<Item = EdgePosition> + Clone {
        let mut overlap: Vec<EdgePosition> = Vec::new();

        for first in self.history.iter() {
            for second in other.history.iter() {
                if *first == *second {
                    overlap.push(*first);
                }
            }
        }

        overlap.into_iter()
    }

    pub fn owner(&self) -> OwnershipToken {
        self.owner
    }

    pub fn current_position(&self) -> EdgePosition {
        self.current
    }

    pub fn finished(&mut self) {
        self.finished_advancing = true;
        self.history.push(self.current);
    }

    pub fn is_finished(&self) -> bool {
        self.finished_advancing
    }

    pub fn length(&self) -> u32 {
        self.history.len() as u32
    }

    pub fn is_continuous(&self) -> Option<bool> {
        let mut history = self.history.iter();
        let mut previous_position = *history.next()?;
        for position in history {
            if previous_position.is_neighbor(*position) {
                previous_position = *position;
                continue;
            } else {
                return Some(false);
            }
        }
        Some(true)
    }

    fn is_position_behind_current(&self, position: EdgePosition) -> bool {
        if let Some(last) = self.history.last() {
            position.is_neighbor(*last)
        } else {
            false
        }
    }
}
