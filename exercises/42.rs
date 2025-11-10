use std::cmp::Ordering;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = height[left];
        let mut right_max = height[right];
        let mut water = 0;

        while left < right {
            // Yes, a simple 'if' would be better
            // But the original approach was handling all arms and now I prefer this to remember
            // that Ordering exists.
            match height[left].cmp(&height[right]) {
                Ordering::Less => {
                    left += 1;
                    left_max = left_max.max(height[left]);
                    water += left_max - height[left];
                }
                _ => {
                    right -= 1;
                    right_max = right_max.max(height[right]);
                    water += right_max - height[right];
                }
            }
        }

        water
    }
}
