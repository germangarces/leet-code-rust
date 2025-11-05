struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().map_or(0, |w| w.len() as i32)
    }
}

fn main() {
    assert_eq!(
        Solution::length_of_last_word(String::from("Hello World")),
        5
    );
    assert_eq!(
        Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
        4
    );
    assert_eq!(
        Solution::length_of_last_word(String::from("luffy is still joyboy")),
        6
    );
}
