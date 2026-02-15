use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Dice([u8; 2]);

impl Dice {
    pub fn roll(rng: &mut (impl SeedableRng + RngCore)) -> Self {
        let first = rng.random_range(1..7);
        let second = rng.random_range(1..7);

        Dice([first, second])
    }

    pub fn sum(&self) -> u8 {
        self.0.iter().sum()
    }
}
