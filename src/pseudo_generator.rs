use rand::Rng;
const LOW: u8 = 0;
const HIGH: u8 = 255;

pub struct Rand {
    pub value: u8,
}
impl Rand {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

pub fn random() -> Rand {
    let rand = rand::rng().random_range(LOW..=HIGH);
    Rand::new(rand)
}
