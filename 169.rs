use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut seen_count = HashMap::with_capacity(n);

        for &num in &nums {
            let count = seen_count.entry(num).and_modify(|c| *c += 1).or_insert(1);
            if *count > n / 2 {
                return num;
            }
        }

        unreachable!()
    }
}
