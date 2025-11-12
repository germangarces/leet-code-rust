impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut p = s.to_lowercase();
        p.retain(|c| c.is_alphanumeric());
        p.chars().eq(p.chars().rev())
    }
}
