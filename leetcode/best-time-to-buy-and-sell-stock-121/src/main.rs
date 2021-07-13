// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

// Example 1:
// Input: prices = [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
// Example 2:

// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transactions are done and the max profit = 0.
 
// Constraints:
// 1 <= prices.length <= 105
// 0 <= prices[i] <= 104

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min = prices[0];

        for (_i, item) in prices.iter().enumerate() {
            if *item < min {
                min = *item;
            }

            if *item - min > profit {
                profit = *item - min;
            }

        }

        return profit;
    }
}

// Better solution: 最大子陣列問題

impl Solution {
    pub fn max_profit_Better(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut curr = 0;
        for i in 1..prices.len() {
            curr = curr + prices[i] - prices[i - 1];
            if curr <= 0 {
                curr = 0;
            } else {
                max = i32::max(max, curr);
            }
        }
        max
    }
}

fn main() {}
