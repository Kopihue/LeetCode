/*
Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
Return the array in the form [x1,y1,x2,y2,...,xn,yn].

"n" represents the first half of the Vector, so "2n" means that the array "nums" has a length of "2n".

"n" elements means the you have to take "n" elements in order to create the first half.

So, I have to return a vector containing the first element of the first half of the vector, then the first element of the second half of the vector and so on.
*/

fn main() {
    let given_vector: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
    let n: i32 = 3;
    let new_vector = get_new_vector(given_vector, n);
}

fn get_new_vector(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;

    let slice_one = &nums[0..n];
    let slice_two = &nums[n..];
    let mut new_vector = vec![];

    for index in 0..n {
        new_vector.push(slice_one[index]);
        new_vector.push(slice_two[index]);
    }

    new_vector
}
