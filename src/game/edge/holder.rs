use std::ops::{Index, IndexMut};

use super::{bounds::EdgeBounds, position::EdgePosition, Edge};

pub struct EdgeHolder {
    edges: Vec<Edge>,
    bounds: EdgeBounds
}

impl EdgeHolder {
    pub fn new(bounds: EdgeBounds) -> Self {
        let edges = Vec::with_capacity(bounds.get_size());
        EdgeHolder {
            edges,
            bounds
        }
    }

    fn calc_index(&self, position: EdgePosition) -> Option<usize> {
        if !self.bounds.check_bounds(position) {
            return None;
        }

        let rights: isize = position.horizontal_distance(EdgePosition::EMPTY).try_into().ok()?;
        let downs: isize = position.vertical_distance(EdgePosition::EMPTY).try_into().ok()?;
        let length_h: isize = self.get_length().try_into().ok()?;
        let length_v: isize = self.get_vertical_length().try_into().ok()?;

        ((downs.checked_div(2)? + downs % 2) * length_h + downs.checked_div(2)? * length_v + if downs % 2 == 0 { rights.checked_div(2)? } else { rights - (rights.checked_div(3)? * 3) })
        .try_into()
        .ok()
       
    }

    pub fn calc_adjacent_edges(&self, position: EdgePosition) -> Vec<EdgePosition> {
        position.calc_adjacent_edges()
        .into_iter()
        .filter(|p| self.bounds.check_bounds(*p) )
        .collect()
    }

    pub fn get(&self, position: EdgePosition) -> Option<&Edge> {
        Some(&self.edges[self.calc_index(position)?])
    }

    pub fn get_mut(&mut self, position: EdgePosition) -> Option<&mut Edge> {
        let idx = self.calc_index(position)?;
        Some(&mut self.edges[idx])
    }

    fn get_length(&self) -> i32 {
        self.bounds.get_bottom_right().horizontal_distance(self.bounds.get_top_left()).abs()/2
    }

    fn get_width(&self) -> i32 {
        self.bounds.get_bottom_right().vertical_distance(self.bounds.get_top_left()).abs()
    }

    /// Get the number of edges in a "vertical row".
    fn get_vertical_length(&self) -> i32 {
        // due to the way a hexagonal board works, `length` is guaranteed to be an odd number.
        (self.get_length() + 1)/2
    }

    /// Get the number of rows of vertical edges.
    /// Currently unused.
    fn get_vertical_width(&self) -> i32 {
        self.get_width() - 1
    }
}

impl Index<EdgePosition> for EdgeHolder {
    type Output = Edge;

    fn index(&self, index: EdgePosition) -> &Self::Output {
        self.get(index).expect("EdgePosition is out of bounds!")
    }
}

impl IndexMut<EdgePosition> for EdgeHolder {
    fn index_mut(&mut self, index: EdgePosition) -> &mut Self::Output {
        self.get_mut(index).expect("EdgePosition is out of bounds!")
    }
}