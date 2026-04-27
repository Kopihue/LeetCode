// You have a set of integers "s", which originally contains every number from 1 to "n".
// One of the numbers got duplicated to another number in the set, which results in
// Repetition of one number and lose of another one
// You are given an integer array nums representing the set after the error.
// Find the duplicated and missing number and return them as a vector
// It can go backwards
// It can be sorted or not, doesn't matter

use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 1, 3, 4, 6, 5];
    let sorted_vec = find_error_nums(numbers);
    println!("{:#?}", sorted_vec);
}

fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    let mut repeated = HashMap::new();

    for &num in &nums {
        if repeated.contains_key(&num) {
            if let Some(value) = repeated.get_mut(&num) {
                *value += 1;
            }
        } else {
            repeated.insert(num, 1);
        }
    }

    // save missing value
    let mut duplicated = None;
    let mut missing = None;
    for num in 1..=nums.len() {
        let num = num as i32;

        if let Some(value) = repeated.get(&num) {
            if *value > 1 { duplicated = Some(num) }
        } else {
            missing = Some(num);
        }
    }

    match (duplicated, missing) {
        (Some(dup), Some(miss)) => return vec![dup, miss],
        _ => return vec![],
    }
}
