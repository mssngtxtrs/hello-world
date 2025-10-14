#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

fn main() {
    // Create hash map
    let mut a = HashMap::new();

    // Insert value in hash map
    a.insert("demo".to_string(), 42);
    a.insert("aaa".to_string(), 100);

    // Rewrite value in hash map
    a.insert("demo".to_string(), 200);

    // Insert value in hash map if not exists
    a.entry("aaa".to_string()).or_insert(450);
    a.entry("bbb".to_string()).or_insert(600);

    // Get value
    let a_res = a
        .get("demo")
        .copied()
        .expect("Error getting value from hash map!");

    // Print value
    println!("{a_res}");

    // Get values in cycle
    for (k, v) in &a {
        println!("{k}, {v}");
    }
}
