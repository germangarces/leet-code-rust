impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        ratings
            .windows(2)
            .map(|x| x[0].cmp(&x[1]))
            // (total candies so far, candies for current ascending slope, peak candies, descending slope length)
            .fold((1, 1, 1, 0), |(candies, front, top, dec), x| match x {
                std::cmp::Ordering::Less => (candies + front + 1, front + 1, front + 1, 0),
                std::cmp::Ordering::Equal => (candies + 1, 1, 1, 0),
                std::cmp::Ordering::Greater if top <= dec + 1 => {
                    (candies + dec + 2, 1, top + 1, dec + 1)
                }
                std::cmp::Ordering::Greater => (candies + dec + 1, 1, top, dec + 1),
            })
            .0
    }
}

