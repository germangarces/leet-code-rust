impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let shortest = strs.iter().min_by_key(|s| s.len()).unwrap();

        for i in 0..shortest.len() {
            let prefix = &shortest[..=i];
            if strs.iter().any(|s| !s.starts_with(prefix)) {
                return shortest[..i].to_string();
            }
        }

        shortest.clone()
    }
}
