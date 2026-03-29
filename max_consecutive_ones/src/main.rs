/*
Given a binary array nums, return the maximum number of consecutive 1's in the array.

Soooo... I have to iterate over the array, and count the times the consecutives o"1" had appeared, I have to get an integer representing the largest time "1" was conecutive.
*/

fn main() {
    let binary_vector = vec![1, 0, 1, 1, 0, 1];
    let consecutives = get_consecutive_ones(binary_vector);
    println!("{consecutives}");
}

fn get_consecutive_ones(nums: Vec<i32>) -> i32 {
    // 1, 0, 1, 1, 0, 1
    let mut consecutives = 0;
    let mut tries = 0;
    for value in nums {
        if value == 1 {
            tries += 1;
            if tries > consecutives {
                consecutives = tries;
            }
        } else {
            tries = 0;
        }
    }

    consecutives
}
