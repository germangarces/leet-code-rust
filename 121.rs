use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // acc.0 keeps track of minimum price
        // acc.1 keeps track of maximum profit
        prices
            .iter()
            .fold((i32::MAX, 0), |acc, &p| {
                (acc.0.min(p), acc.1.max(p - acc.0))
            })
            .1
    }
}
