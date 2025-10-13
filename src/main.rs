#![warn(clippy::all, clippy::pedantic)]

mod pseudo_generator {
    pub fn random() -> u8 {
        255
    }
}

use pseudo_generator::random;

fn main() {
    let random = random();
    println!("Hello, World!\n{random}");
}
