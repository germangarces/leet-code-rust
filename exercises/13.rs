impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // For few values is more efficient that a HashMap
        // And we avoid heap allocation
        fn val(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        s.chars()
            .map(val) // Transform from roman to ints
            .fold((0, i32::MAX), |(mut acc, mut prev), curr| {
                if prev < curr {
                    acc -= 2 * prev;
                }
                acc += curr;
                prev = curr;
                (acc, prev)
            })
            .0
    }
}

