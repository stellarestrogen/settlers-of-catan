use super::{
    draw_iters::CircularOrbit,
    objects::{ResourceDistribution, ResourceType},
    tile::position::TilePosition,
};

const ROLL_ORDER_REVERSE: [u32; 18] = [11, 3, 6, 5, 4, 9, 10, 8, 4, 11, 12, 9, 10, 8, 3, 6, 2, 5];

const ROLL_ORDER_EXP: [u32; 28] = [
    2, 5, 4, 6, 3, 9, 8, 11, 11, 10, 6, 3, 8, 4, 8, 10, 11, 12, 10, 5, 4, 9, 5, 9, 12, 3, 2, 6,
];

const ROLL_ORDER_EXP_REVERSE: [u32; 28] = [
    6, 2, 3, 12, 9, 5, 9, 4, 5, 10, 12, 11, 10, 8, 4, 8, 3, 6, 10, 11, 11, 8, 9, 3, 6, 4, 5, 2,
];

pub trait GameEdition {
    fn get_board_size(&self) -> usize;
    fn get_board_length(&self) -> u32;
    fn get_board_width(&self) -> u32;
    fn get_resource_distribution(&self) -> ResourceDistribution;
    fn get_roll_numbers(&self) -> Vec<u32>;
    fn get_tile_draw_iter(&self) -> impl Iterator<Item = TilePosition>;
}

pub struct BaseEdition {
    size: usize,
    length: u32,
    width: u32,
    rsrc_distr: ResourceDistribution,
    roll_numbers: Vec<u32>,
    iter: CircularOrbit,
}

impl BaseEdition {
    pub fn new() -> Self {
        BaseEdition {
            size: 19,
            width: 5,
            length: 5,
            rsrc_distr: ResourceDistribution::new([
                (ResourceType::Wood, 4),
                (ResourceType::Brick, 3),
                (ResourceType::Wheat, 4),
                (ResourceType::Sheep, 4),
                (ResourceType::Ore, 3),
            ]),
            roll_numbers: ROLL_ORDER_REVERSE.to_vec(),
            iter: CircularOrbit::new(TilePosition::RIGHT, 3, 5),
        }
    }
}

impl GameEdition for BaseEdition {
    fn get_board_size(&self) -> usize {
        self.size
    }

    fn get_board_length(&self) -> u32 {
        self.length
    }

    fn get_board_width(&self) -> u32 {
        self.width
    }

    fn get_resource_distribution(&self) -> ResourceDistribution {
        self.rsrc_distr.clone()
    }

    fn get_roll_numbers(&self) -> Vec<u32> {
        self.roll_numbers.clone()
    }

    fn get_tile_draw_iter(&self) -> impl Iterator<Item = TilePosition> {
        self.iter
    }
}

pub struct ExpansionEdition {
    size: usize,
    length: u32,
    width: u32,
    rsrc_distr: ResourceDistribution,
    roll_numbers: Vec<u32>,
    iter: CircularOrbit,
}

impl ExpansionEdition {
    pub fn new() -> Self {
        ExpansionEdition {
            size: 30,
            length: 6,
            width: 7,
            rsrc_distr: ResourceDistribution::new([
                (ResourceType::Wood, 6),
                (ResourceType::Brick, 5),
                (ResourceType::Wheat, 6),
                (ResourceType::Sheep, 6),
                (ResourceType::Ore, 5),
            ]),
            roll_numbers: ROLL_ORDER_EXP_REVERSE.to_vec(),
            iter: CircularOrbit::new(TilePosition::RIGHT, 3, 6),
        }
    }
}

impl GameEdition for ExpansionEdition {
    fn get_board_size(&self) -> usize {
        self.size
    }

    fn get_board_length(&self) -> u32 {
        self.length
    }

    fn get_board_width(&self) -> u32 {
        self.width
    }

    fn get_resource_distribution(&self) -> ResourceDistribution {
        self.rsrc_distr.clone()
    }

    fn get_roll_numbers(&self) -> Vec<u32> {
        self.roll_numbers.clone()
    }

    fn get_tile_draw_iter(&self) -> impl Iterator<Item = TilePosition> {
        self.iter
    }
}

pub struct CustomEdition {
    shortest: u32,
    longest: u32,
    rsrc_distr: ResourceDistribution,
    roll_numbers: Vec<u32>,
    iter: CircularOrbit,
}

impl CustomEdition {
    pub fn new(
        shortest: u32,
        longest: u32,
        rsrc_distr: ResourceDistribution,
        roll_numbers: Vec<u32>,
    ) -> Self {
        let position = TilePosition::DOWN_LEFT * ((longest - shortest) / 2 + 1)
            + TilePosition::DOWN_RIGHT * ((longest - shortest) / 2)
            + TilePosition::UP_RIGHT * (longest - shortest);
        CustomEdition {
            shortest,
            longest,
            rsrc_distr,
            roll_numbers,
            iter: CircularOrbit::new(position, shortest, longest),
        }
    }
}

impl GameEdition for CustomEdition {
    fn get_board_size(&self) -> usize {
        (self.longest + self.longest * (self.longest - 1) - self.shortest * (self.shortest - 1))
            as usize
    }

    fn get_board_length(&self) -> u32 {
        self.longest
    }

    fn get_board_width(&self) -> u32 {
        (self.longest - self.shortest) * 2 - 1
    }

    fn get_resource_distribution(&self) -> ResourceDistribution {
        self.rsrc_distr.clone()
    }

    fn get_roll_numbers(&self) -> Vec<u32> {
        self.roll_numbers.clone()
    }

    fn get_tile_draw_iter(&self) -> impl Iterator<Item = TilePosition> {
        self.iter
    }
}
