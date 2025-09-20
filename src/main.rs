#![warn(clippy::all, clippy::pedantic)]



// Clippy - utility to analyze and dandify the code
// Command to use Clippy: cargo clippy



//###############
//#   IMPORTS   #
//###############



use std::cmp::Ordering;



//############
//#   MAIN   #
//############



fn main() {
    let array = [-1, 1, 2, 4, 5, 6, 15, 33, 144];
    let target = -5;

    let result = bin_search(&array, target);

    match result {
        Some((element, index)) => println!("Found {element} on position {index}"),
        None => println!("Element not found")
    }
}



//#################
//#   FUNCTIONS   #
//#################



//----Binary search----
fn bin_search(array: &[i32], target: i32) -> Option<(i32, usize)> {
    //--Setting low and high indexes--
    let mut low = 0;
    let mut high = array.len() - 1;

    //--Setting iteration initializer--
    let mut i = 0;

    //--Entering loop only if target value is greater than lowest or less than highest--
    if target > array[low] && target < array[high] {
        while low <= high {
            //--Incrementing initializer--
            i += 1;

            //--Finding value in the middle of array--
            let mid = usize::midpoint(low, high);
            let mid_value = array[mid];

            //--Printing iteration data--
            println!("Iteration {i}:\nPosition: {mid}\nValue: {mid_value}\n");

            //--Matching middle value with target value--
            match mid_value.cmp(&target) {
                Ordering::Equal => return Some((mid_value, mid)),
                Ordering::Greater => high = mid - 1,
                Ordering::Less => low = mid + 1
            }
        }
    }

    //--Return if not found--
    None
}



//#############
//#   TESTS   #
//#############



// Command to run tests: cargo test



#[cfg(test)]
mod tests {
    use super::*;

    const ARRAY: [i32; 9] = [-1, 1, 2, 4, 5, 6, 15, 33, 144];



    //----Tests when found----
    #[test]
    fn element_found_1() {
        let target = 6;

        assert_eq!((6, 5), bin_search(&ARRAY, target).unwrap());
    }

    #[test]
    fn element_found_2() {
        let target = 33;

        assert_eq!((33, 7), bin_search(&ARRAY, target).unwrap());
    }



    //----Tests when not found----
    #[test]
    fn element_not_found_1() {
        let target = -5;

        assert!(bin_search(&ARRAY, target).is_none());
    }

    #[test]
    fn element_not_found_2() {
        let target = 145;

        assert!(bin_search(&ARRAY, target).is_none());
    }

}