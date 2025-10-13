use rand::Rng;
const LOW: u8 = 0;
const HIGH: u8 = 255;

mod random_number;

use random_number::Rand;

pub fn random() -> Rand {
    super::shared();

    let rand = rand::rng().random_range(LOW..=HIGH);
    Rand::new(rand)
}
