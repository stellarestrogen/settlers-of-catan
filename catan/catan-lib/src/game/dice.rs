use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Debug, Clone)]
pub struct Dice {
    dice: [u8; 2],
    rng: SmallRng,
}

impl Dice {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            dice: [0, 0],
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn roll(&mut self) -> &Self {
        let first = self.rng.random_range(1..7);
        let second = self.rng.random_range(1..7);

        self.dice = [first, second];

        self
    }

    pub fn sum(&self) -> u8 {
        self.dice.iter().sum()
    }
}
