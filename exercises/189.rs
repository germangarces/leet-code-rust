impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if !nums.is_empty() {
            let len = nums.len();
            // Use rem to handle cases where k >= len
            nums.rotate_right(k.rem_euclid(len as i32) as usize)
        }
    }
}
