impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut i = 2; // read index
        for j in 2..nums.len() {
            if nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }

        nums.truncate(i);
        i as i32
    }
}
