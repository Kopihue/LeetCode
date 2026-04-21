// You have a set of integers "s", which originally contains every number from 1 to "n".
// One of the numbers got duplicated to another number in the set, which results in
// Repetition of one number and lose of another one
// You are given an integer array nums representing the set after the error.
// Find the duplicated and missing number and return them as an array.
// It can go backwards
// It can be sorted or not, doesn't matter

fn main() {
    let numbers = vec![3, 2, 3, 4, 6, 5];
    let sorted_vec = find_error_nums(numbers);
    println!("{:#?}", sorted_vec);
}

fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    let mut repeated: [i32; 2] = [0, 1];

    for n in 1..=nums.len() {
        let mut counter = 0;

        for &num in &nums {
            if num == n as i32 {
                counter += 1; 
            }
        }

        if counter > repeated[1] {
            repeated = [n as i32, counter];
        }
    }

    for i in 1..=nums.len() {
        let i = i as i32;
        let mut appeared = 0;
         
        for &n in &nums {
            if i == n {
                appeared += 1;
            }
        }

        if appeared == 0 {
            return vec![repeated[0], i];
        }
    }

    vec![]
}
