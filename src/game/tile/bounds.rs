use super::position::{TilePosition, HorizontalDistance};

#[derive(Debug)]
pub struct TileBounds {
    top_left: TilePosition,
    bottom_right: TilePosition
}

impl TileBounds {
    pub fn new() -> Self {
        TileBounds {
            top_left: TilePosition::ORIGIN,
            bottom_right: TilePosition::ORIGIN
        }
    }

    pub fn get_top_left(&self) -> TilePosition {
        self.top_left
    }

    pub fn get_bottom_right(&self) -> TilePosition {
        self.bottom_right
    }

    pub fn check_bounds(&self, position: TilePosition) -> bool {
        (position.is_right(self.top_left) || position.is_same_column(self.top_left)) && 
        (position.is_below(self.top_left) || position.is_same_row(self.top_left)) && 
        (position.is_left(self.bottom_right) || position.is_same_column(self.bottom_right)) && 
        (position.is_above(self.bottom_right) || position.is_same_row(self.bottom_right))
    }

    pub fn expand_bounds(&mut self, position: TilePosition) {
        if position.is_left(self.top_left) {
            self.top_left += TilePosition::LEFT * position.horizontal_distance(self.top_left).ceil().abs();
        } else if position.is_right(self.bottom_right) {
            self.bottom_right += TilePosition::RIGHT * position.horizontal_distance(self.bottom_right).ceil().abs();
        }
        
        if position.is_above(self.top_left) {
            let vertical_distance = position.vertical_distance(self.top_left).abs();
            let pos_offset = position + TilePosition::LEFT * position.horizontal_distance(self.top_left).ceil().abs();
            let shift: f64 = pos_offset.horizontal_distance(self.top_left).into();
            let adjustment = if shift > 0. { TilePosition::UP_RIGHT } else if shift < 0. { TilePosition::UP_LEFT } else { TilePosition::ORIGIN };
            self.top_left += TilePosition::UP_LEFT * (vertical_distance/2) + TilePosition::UP_RIGHT * (vertical_distance/2) + adjustment;

        } else if position.is_below(self.bottom_right) {
            let vertical_distance = position.vertical_distance(self.bottom_right).abs();
            let pos_offset = position + TilePosition::RIGHT * position.horizontal_distance(self.bottom_right).ceil().abs();
            let shift: f64 = pos_offset.horizontal_distance(self.bottom_right).into();
            let adjustment = if shift > 0. { TilePosition::DOWN_RIGHT } else if shift < 0. { TilePosition::DOWN_LEFT } else { TilePosition::ORIGIN };
            self.bottom_right += TilePosition::DOWN_LEFT * (vertical_distance/2) + TilePosition::DOWN_RIGHT * (vertical_distance/2) + adjustment;
        }
    }
}

#[test]
fn test() {
    let mut bounds = TileBounds::new();
    let pos1 = TilePosition::DOWN_LEFT * 4 + TilePosition::LEFT * 3; // -5, 4
    let pos2 = TilePosition::UP_LEFT; // 0, -1
    let pos3 = TilePosition::DOWN_RIGHT * 4 + TilePosition::RIGHT * 2; // 4, 4
    bounds.expand_bounds(pos1);
    assert_eq!(bounds.get_top_left(), TilePosition::LEFT * 5);
    assert_eq!(bounds.get_bottom_right(), TilePosition::DOWN_LEFT * 2 + TilePosition::DOWN_RIGHT * 2);

    bounds.expand_bounds(pos2);
    assert_eq!(bounds.get_top_left(), TilePosition::LEFT * 5 + TilePosition::UP_LEFT);
    assert_eq!(bounds.get_bottom_right(), TilePosition::DOWN_LEFT * 2 + TilePosition::DOWN_RIGHT * 2);

    bounds.expand_bounds(pos3);
    assert_eq!(bounds.get_top_left(), TilePosition::LEFT * 5 + TilePosition::UP_LEFT);
    assert_eq!(bounds.get_bottom_right(), TilePosition::DOWN_RIGHT * 4 + TilePosition::RIGHT * 2);
}