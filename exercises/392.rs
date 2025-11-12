impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        if s.len() > t.len() {
            return false;
        }

        let mut s_iter = s.bytes();
        let mut current = s_iter.next();

        for b in t.bytes() {
            if Some(b) == current {
                current = s_iter.next();
                if current.is_none() {
                    return true;
                }
            }
        }

        false
    }
}
