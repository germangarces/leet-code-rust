impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 { return 0; }

        let mut jumps = 0;
        let mut current_end = 0;
        let mut farthest = 0;

        for (i, &num) in nums.iter().take(n - 1).enumerate() {
            farthest = farthest.max(i + num as usize);
            if i == current_end {
                jumps += 1;
                current_end = farthest;
            }
        }

        jumps
    }
}
