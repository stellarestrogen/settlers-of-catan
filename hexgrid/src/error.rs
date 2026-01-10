
pub enum PositionErr {
    Hex,
    Corner,
    Edge
}

impl PositionErr {

}

pub enum GridErr {
    OutOfBounds { r#type: PositionErr },
    NoData { r#type: PositionErr },
}

impl GridErr {
    
}