impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // Furthest reachable index
        let mut available_jumps = 0;

        for (i, &num) in nums.iter().enumerate() {
            // Can I reach this index? If not, fail. Otherwise, update my maxx reach
            if i > available_jumps as usize {
                return false;
            }
            available_jumps = available_jumps.max(i as i32 + num);
        }

        true
    }
}

