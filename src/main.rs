#![warn(clippy::all, clippy::pedantic)]

use std::ops::Add;

//fn sum<T>(numbers: &[T]) -> T {
//    numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
//}

#[derive(Debug)]
struct Employee<T, K> {
    age: T,
    salary: K,
    tax: K,
}

fn main() {
    //let int_1: Vec<i32> = vec![1, 2, 3, 4];
    //let int_2: Vec<i8> = vec![63, 127, 0];
    //
    //let result_1 = sum(&int_1);
    //let result_2 = sum(&int_2);

    let employee = Employee {
        age: 30,
        salary: 30000.,
        tax: 5000.,
    };

    println!("{employee:?}");
}
