use std::cmp::Reverse;

impl Solution {
    // I just took a look at wikipedia definition of h-index
    // And the function expects a orderdered slice so I did it.
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut array = citations.to_vec();
        array.sort_by_key(|w| Reverse(*w));

        let mut lo = 0;
        let mut hi = array.len();

        while lo < hi {
            let mid = (lo + hi) / 2;
            if array[mid] >= (mid + 1) as i32 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo as i32
    }
}
