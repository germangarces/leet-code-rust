impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut curr_window_sum = 0;
        let mut res: Option<usize> = None;

        for (right, &num) in nums.iter().enumerate() {
            curr_window_sum += num;

            while curr_window_sum >= target {
                let window_len = right - left + 1;
                res = Some(res.map_or(window_len, |v| v.min(window_len)));

                curr_window_sum -= nums[left];
                left += 1;
            }
        }

        res.map_or(0, |v| v as i32)
    }
}
