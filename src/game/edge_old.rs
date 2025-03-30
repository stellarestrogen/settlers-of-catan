#[derive(Clone)]
#[derive(Copy)]
pub enum Edge {
    Road,
    None,
}

pub struct EdgeHolder {
    edges: Vec<Edge>,
    total_edges: u32,
    row_length_horizontal: u32,
    row_count_horizontal: u32,
    row_length_vertical: u32,
    row_count_vertical: u32,

}

impl EdgeHolder {
    pub fn new() -> Self {
        EdgeHolder {
            edges: Vec::<Edge>::new(),
            total_edges: 0,
            row_length_horizontal: 0,
            row_count_horizontal: 0,
            row_length_vertical: 0,
            row_count_vertical: 0,
        }
    }

    pub fn clear(&mut self) {
        self.edges.clear();
        self.total_edges = 0;
        self.row_length_horizontal = 0;
        self.row_count_horizontal = 0;
        self.row_length_vertical = 0;
        self.row_count_vertical = 0;

    }

    /// sets the length of a row for each vector of edges based on the length of a tile row.
    pub fn set_row_length(&mut self, length: u32) {
        self.row_length_horizontal = length * 2 + 1;
        self.row_length_vertical = length + 1;
    }

    /// sets the number of rows for each vector of edges based on the number of tile rows.
    pub fn set_row_count(&mut self, count: u32) {
        self.row_count_horizontal = count + 1;
        self.row_count_vertical = count;
    }

    pub fn setup(&mut self, length: u32, count: u32) {
        self.set_row_length(length);
        self.set_row_count(count);
        // the `-2` is due to the fact the top and bottom rows of edges (which are always horizontal) have 1 less edge in them.
        self.total_edges = self.row_length_horizontal * self.row_count_horizontal + self.row_length_vertical * self.row_count_vertical - 2;
    }

    /// Generates the array. Edges will be utilized such that horizontal and vertical edges are stored "together"
    /// For example, if you had a 3x5 board, there would be 11 horizontal edges per row, and 6 vertical edges per row, with a total of 6 rows. 
    /// The only exception to this is the first and last row of edges; they contain 1 less edge each due to not having a row above/below them.
    pub fn generate(&mut self, length: u32, count: u32) {
        self.setup(length, count);
        self.edges.reserve(self.total_edges as usize);

        for _ in 0..self.edges.capacity() {
            self.edges.push(Edge::None);
        }
    
    }
    
    pub fn build_road (&mut self, idx: u32) {
        self.edges[idx as usize] = Edge::Road;
    }

    fn calc_row(&self, idx: u32) -> u32 {
        // the first row of horizontal edges having 1 less edge than the rest causes another off-by-one. 
        (idx + 1) / (self.row_length_horizontal + self.row_length_vertical)
    }

    fn calc_row_idx(&self, idx: u32) -> u32 {
        if self.calc_row(idx) == 0 && idx < self.row_length_horizontal - 1 { return idx };
        let row_length = self.row_length_horizontal + self.row_length_vertical;
        let edge = (idx + 1) % row_length;
        if edge >= self.row_length_horizontal {
            edge - self.row_length_horizontal
        } else {
            edge
        }
    }

    fn is_edge_vertical(&self, idx: u32) -> bool {
        let row_length = self.row_length_horizontal + self.row_length_vertical;
        // the first row of horizontal edges having 1 less edge than the rest causes an off-by-one. 
        (idx + 1) % row_length >= self.row_length_horizontal
    }

    /// this will only generate valid results if the edge is vertical!
    fn find_adjacent_edges(&self, idx: u32) -> [Option<u32>; 4] {
        let row = self.calc_row(idx);
        let row_idx = self.calc_row_idx(idx);
        let mut adjacent_edges: [Option<u32>; 4] = if row % 2 == 0 {
            [
                Some(idx.saturating_sub(self.row_length_horizontal) + row_idx),
                Some(idx.saturating_sub(self.row_length_horizontal - 1) + row_idx),
                Some(idx + self.row_length_vertical + row_idx),
                Some(idx + (self.row_length_vertical + 1) + row_idx)
            ]
        } else {
            [
                Some(idx.saturating_sub(self.row_length_horizontal + 1) + row_idx),
                Some(idx.saturating_sub(self.row_length_horizontal) + row_idx),
                Some(idx + (self.row_length_vertical - 1) + row_idx),
                Some(idx + self.row_length_vertical + row_idx)
            ]
        };
        
        for edge in 0..adjacent_edges.len() {
            // validity check. all of these edges that are adjacent to the vertical edge must be horizontal. if they are not, set to None to indicate it's invalid.
            // this is because not all vertical edges have 4 adjacent edges. some only have 2 or 3, and any edge produced by the method above will be vertical if it is not actually adjacent.
            if self.is_edge_vertical(adjacent_edges[edge].unwrap()) || adjacent_edges[edge].unwrap() >= self.total_edges { 
                adjacent_edges[edge] = None; 
            } 
        }

        adjacent_edges
    }

    /// determine if 2 edges are adjacent
    pub fn is_edge_adjacent(&self, first: u32, second: u32) -> bool {
        if self.is_edge_vertical(first) && self.is_edge_vertical(second) {
            return false; // 2 vertical edges cannot be adjacent.
        } else if !self.is_edge_vertical(first) && !self.is_edge_vertical(second) {
            return first.abs_diff(second) <= 1; // if both edges are horizontal, we simply have to check if they are next to each other.
        } else {
            // most complex case, where one edge is vertical and one edge is horizontal
            let (vertical_edge, horizontal_edge) = if self.is_edge_vertical(first) { (first, second) } else { (second, first) };
            
            // find all of the edges that border the vertical edge
            for edge in self.find_adjacent_edges(vertical_edge) {
                // check if any of the edges are equal to the horizontal edge. if so, return true!
                match edge {
                    Some(value) => if value == horizontal_edge { return true; } else { continue; },
                    None => continue,
                }
            }

            false
        }
    }

}