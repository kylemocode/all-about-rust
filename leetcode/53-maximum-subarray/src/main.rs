// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

 

// Example 1:

// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
// Example 2:

// Input: nums = [1]
// Output: 1
// Example 3:

// Input: nums = [5,4,-1,7,8]
// Output: 23

// Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

struct Solution{}


// O(n) solution
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::min_value();
        let mut curr = 0;
        for j in 0..nums.len() {
            curr += nums[j];
            max = i32::max(max, curr);
            if curr <= 0 {
                curr = 0;
            }
        }
        max
    }
}

// Divide and Conqure Solution

fn main() {
    println!("Hello, world!");
}
