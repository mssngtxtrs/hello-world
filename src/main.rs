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

    create_dir_all(&current_path).unwrap_or_else(|e| panic!("Error: {e:?}"));
}
