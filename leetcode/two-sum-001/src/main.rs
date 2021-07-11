/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they
 * add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 */

 // use HashMap -> time complexity O(n)
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn tow_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.len();
        let mut m = HashMap::with_capacity(l);
        
        for (index, num) in nums.iter().enumerate() {
            match m.get(&(target - num)) {
                None => {
                    m.insert(num, index);
                }
                Some(i) => return vec![*i as i32, index as i32],
            }
        }
        vec![]
    }
}

fn main() {}
