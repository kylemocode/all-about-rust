struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
         let mut size = nums.len();
        if size == 0 {
            return -1;
        }
        let mut base = 0_usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            if nums[mid] == target {
                return mid as i32;
            }
            // we split the ring to [base..half] & [half+1..base-1]
            // if target not in [base..half] ring, move base to select another ring
            if !(((nums[base] < nums[mid]) && (target >= nums[base] && target <= nums[mid]))
                || ((nums[base] > nums[mid]) && (target >= nums[base] || target <= nums[mid])))
            {
                base = mid;
            }
            size -= half;
        }
        if nums[base] == target {
            base as i32
        } else {
            -1
        }
    }
}

fn main() {
    println!("Hello, world!");
}
