struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut neg_max = 0;
        let mut pos_max = 0;
        
        for num in nums.into_iter() {
            if num == 0 {
                neg_max = 0;
                pos_max = 0;
                max = i32::max(max, 0)
            }
            
            if num > 0 {
                pos_max = i32::max(pos_max*num, num);
                neg_max = neg_max * num;
            }
            
            if num < 0 {
                let pos_pre = pos_max;
                pos_max = neg_max * num;
                neg_max = i32::min(pos_pre * num, num);
            }
            
            if pos_max != 0 {
                max = i32::max(max, pos_max);
            }
        }
        max
    }
}

fn main() {
    println!("Hello, world!");
}
