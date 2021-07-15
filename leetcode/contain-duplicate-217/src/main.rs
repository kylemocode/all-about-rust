// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

 

// Example 1:

// Input: nums = [1,2,3,1]
// Output: true
// Example 2:

// Input: nums = [1,2,3,4]
// Output: false
// Example 3:

// Input: nums = [1,1,1,3,3,4,3,2,4,2]
// Output: true

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut number_map = HashMap::new();
        for i in 0..nums.len() {
            let count = number_map.entry(nums[i]).or_insert(0);
            *count += 1;
            if *count > 1 {
                return true;
            }
        }
        false
    }
}

// better solution

impl Solution {
    pub fn contains_duplicate_better(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return false;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut prev = nums[0];
        for i in 1..nums.len() {
            if nums[i] == prev {
                return true;
            }
            prev = nums[i]
        }
        false
    }
}

fn main() {}
