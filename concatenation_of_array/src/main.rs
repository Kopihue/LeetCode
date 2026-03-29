/*
Given an integer array "nums" of length "n", you want to create an array "ans" of length "2n" where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).

In human language, I have to copy the given vector and iterate over the given vector, pushing each value into the new created vector
*/

fn main() {
    let given_vector: Vec<i32> = vec![1, 2, 1];
    let new_vector = get_new_vector(given_vector);
}

fn get_new_vector(nums: Vec<i32>) -> Vec<i32> {
    let mut new_vector: Vec<i32> = nums.clone();

    for (index, _) in nums.iter().enumerate() {
        new_vector.push(nums[index]);
    }

    return new_vector;
}
