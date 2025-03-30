#[derive(Clone, Copy)]
pub enum Transport {
    Road,
    Boat
}

pub struct Edge {
    transport: Option<Transport>
}

impl Edge {
    pub fn new() -> Self {
        Edge {
            transport: None
        }
    }

    pub fn set_transport(&mut self, transport: Transport) {
        self.transport = Some(transport)
    }

    pub fn unset_transport(&mut self) {
        self.transport = None
    }
}

#[derive(Clone, Copy)]
pub struct EdgePosition {
    rights: i32,
    downs: i32
}

impl EdgePosition {
    pub fn new(rights: i32, downs: i32) -> Self {
        EdgePosition {
            rights,
            downs
        }
    }

    pub fn is_valid(&self) -> bool {
        (self.rights % 2 == 0 && self.downs % 2 == 0) ||(self.rights % 2 == 1 && self.downs % 2 == 1 && (self.rights + self.downs) % 4 == 0)
    }

    pub fn calc_distance(&self, other: Self) -> i32 {
        ((self.rights - other.get_rights()).abs() + (self.downs - other.get_downs()).abs())
        .checked_div(2)
        .unwrap()
    }

    pub fn get_rights(&self) -> i32 {
        self.rights
    }

    pub fn get_downs(&self) -> i32 {
        self.downs
    }

    pub fn calc_adjacent_edges(&self) -> Vec<EdgePosition> {
        let edges = [
            EdgePosition::new(self.rights - 1, self.downs + 1),
            EdgePosition::new(self.rights + 1, self.downs + 1),
            EdgePosition::new(self.rights + 2, self.downs),
            EdgePosition::new(self.rights - 2, self.downs),
            EdgePosition::new(self.rights - 1, self.downs - 1),
            EdgePosition::new(self.rights + 1, self.downs - 1)
        ];

        edges.into_iter()
        .filter(|r| r.is_valid() )
        .collect()

    }
}

pub struct EdgeHolder {
    edges: Vec<Edge>,
    length: u32,
    width: u32,
}

impl EdgeHolder {
    pub fn new(length: u32, width: u32) -> Self {
        let edges = Vec::with_capacity(((length + (length + 1)/2) * width - 2) as usize);
        EdgeHolder {
            edges,
            length,
            width
        }
    }

    fn calc_index(&self, position: EdgePosition) -> Option<usize> {
        let rights: isize = position.get_rights().try_into().ok()?;
        let downs: isize = position.get_downs().try_into().ok()?;
        let length_h: isize = self.length.try_into().ok()?;
        let length_v: isize = self.get_vertical_length().try_into().ok()?;

        if position.is_valid() && self.is_edge_position_valid(position) {
            ((downs.checked_div(2)? + downs % 2) * length_h + downs.checked_div(2)? * length_v + if downs % 2 == 0 { rights.checked_div(2)? } else { rights - (rights.checked_div(3)? * 3) })
            .try_into()
            .ok()
        } else {
            None
        }
    }

    pub fn calc_adjacent_edges(&self, position: EdgePosition) -> Vec<EdgePosition> {
        position.calc_adjacent_edges()
        .into_iter()
        .filter(|p| self.is_edge_position_valid(*p) )
        .collect()
    }

    pub fn get(&self, position: EdgePosition) -> Option<&Edge> {
        Some(&self.edges[self.calc_index(position)?])
    }

    pub fn get_mut(&mut self, position: EdgePosition) -> Option<&mut Edge> {
        let idx = self.calc_index(position)?;
        Some(&mut self.edges[idx])
    }

    fn is_edge_position_valid(&self, position: EdgePosition) -> bool {
        position.get_rights().checked_div(2).unwrap_or_default() <= self.length.try_into().unwrap_or_default() && position.get_downs().checked_div(2).unwrap_or_default() <= self.width.try_into().unwrap_or_default()
    }

    /// Get the number of edges in a "vertical row".
    fn get_vertical_length(&self) -> u32 {
        // due to the way a hexagonal board works, `length` is guaranteed to be an odd number.
        (self.length + 1)/2
    }

    /// Get the number of rows of vertical edges.
    /// Currently unused.
    fn get_vertical_width(&self) -> u32 {
        self.width - 1
    }

    /// Get the total number of edges.
    pub fn get_size(&self) -> usize {
        ((self.length * self.width - 2) + self.get_vertical_length() * self.get_vertical_width()) as usize
    }
}