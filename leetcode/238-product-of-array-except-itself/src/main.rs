// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

 

// Example 1:

// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
// Example 2:

// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]


pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let list_length = nums.len();
        if list_length < 2 {
            return vec![];
        }

        let mut answer = vec![1; list_length];
        let mut temp_num = 1;
        
        for i in (0..list_length-1).rev() {
            temp_num *= nums[i+1];
            answer[i] = temp_num;
        }
        
        temp_num = 1;
        
        for i in 1..list_length {
            temp_num *= nums[i-1];
            answer[i] *= temp_num;
        }
        
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
