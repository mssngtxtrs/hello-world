#![warn(clippy::all, clippy::pedantic)]

use std::marker::Copy;
use std::ops::Add;

// Trait
trait Drive {
    fn can_drive(&self) -> bool;
}

// Structure with two types
#[derive(Debug)]
struct Employee<T, K> {
    age: T,
    salary: K,
    tax: K,
}

// Structures with trait implementations
struct Car {
    gas: u32,
}
impl Drive for Car {
    fn can_drive(&self) -> bool {
        self.gas > 0
    }
}

struct ElectroCar {
    charge: u32,
}
impl Drive for ElectroCar {
    fn can_drive(&self) -> bool {
        self.charge > 0
    }
}

fn main() {
    let int_1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let int_2: Vec<i8> = vec![1, 2, 3, 4, 5];

    let result_1 = sum(&int_1);
    let result_2 = sum(&int_2);

    println!("{result_1}\n{result_2}");

    // Now this structure can have two different types
    let employee = Employee {
        age: 30,
        salary: 30000.,
        tax: 5000.,
    };

    let electric_car = ElectroCar { charge: 12 };
    let car = Car { gas: 0 };

    println!("{employee:?}");
    car_info(&car, &electric_car);
}

// Functions with trait implementation
fn car_info<T: Drive, U: Drive>(car: &T, other_car: &U) {
    println!("Can drive? {}", car.can_drive());
    println!("Can other car drive? {}", other_car.can_drive());
}

fn sum<T: Copy + Add<Output = T>>(numbers: &[T]) -> T {
    numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
}
