impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &num) in numbers.iter().enumerate() {
            let searched = target - num;
            if let Ok(j) = numbers.binary_search(&searched) {
                if i != j {
                    return vec![(i + 1) as i32, (j + 1) as i32];
                }
            }
        }
        unreachable!("Input guarantees one solution");
    }
}
