#![warn(clippy::all, clippy::pedantic)]

mod pseudo_generator;

use pseudo_generator::random;

fn main() {
    let random = random().value;
    println!("Hello, World!\n{random}");
}
