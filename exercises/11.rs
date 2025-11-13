impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }

        let (mut left, mut right) = (0, height.len().saturating_sub(1));
        let mut max_seen = 0;

        while left < right {
            let min_bar = height[left].min(height[right]);
            max_seen = max_seen.max(min_bar * (right - left) as i32);
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        max_seen
    }
}
