#![warn(clippy::all, clippy::pedantic)]

use std::env::current_dir;
use std::fs::create_dir_all;
use std::io::ErrorKind;

fn main() {
    let mut current_path = match current_dir() {
        Ok(p) => p,
        Err(e) => panic!("Error: {e:?}"),
    };

    current_path.push("resources/");

    match create_dir_all(&current_path) {
        Ok(()) => println!("Path {} created successfully!", current_path.display()),
        Err(e) => match e.kind() {
            ErrorKind::InvalidFilename => panic!("Invalid filename!"),
            _ => panic!("unknown!"),
        },
    }
}
