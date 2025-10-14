#![warn(clippy::all, clippy::pedantic)]

use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;

fn main() {
    let mut current_path = get_current_dir().expect("Cannot get current path!");

    current_path.push("resources/");

    match create_dir_in(&current_path) {
        Ok(p) => println!("Path \"{p}\" created successfully!"),
        Err(e) => panic!("Error: {e:?}"),
    }
}

fn get_current_dir() -> Result<PathBuf, std::io::Error> {
    let path = current_dir()?;
    Ok(path)
}

fn create_dir_in(target: &PathBuf) -> Result<std::string::String, std::io::Error> {
    match create_dir_all(target) {
        Ok(()) => Ok(format!("{}", target.to_string_lossy())),
        Err(e) => Err(e),
    }
}
