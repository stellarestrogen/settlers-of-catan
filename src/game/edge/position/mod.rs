
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