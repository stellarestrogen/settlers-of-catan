use hexgrid::corner::position::{CornerPosition, Height, High, Low};


struct PlayedBuildings {
    settlements: Vec<(Option<CornerPosition<Low>>, Option<CornerPosition<High>>)>,
    cities: Vec<(Option<CornerPosition<Low>>, Option<CornerPosition<High>>)>
}

impl PlayedBuildings{
    pub fn add_settlement<H: Height>(&mut self, position: CornerPosition<H>) {
        
    }
}

struct PlayedTransport {
    
}

pub struct Player {
    
    
}

impl Player {}
