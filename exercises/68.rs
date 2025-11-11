impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = Vec::new();          // Stores the final justified lines
        let mut cur = Vec::new();          // Stores words for the current line
        let mut num_of_letters: i32 = 0;   // Counts letters in the current line (excluding spaces)

        for word in &words {
            // Check if adding the new word would exceed max width (including minimum one space between words)
            if word.len() as i32 + cur.len() as i32 + num_of_letters > max_width {
                // Distribute extra spaces across the current line
                for i in 0..(max_width - num_of_letters) {
                    /* 
                     * idx calculation explanation:
                     * - cur.len() - 1 gives the number of gaps between words.
                     * - We use modulo to cycle through gaps repeatedly if more spaces than gaps exist.
                     * - If only one word in the line, just append all extra spaces to that word.
                     */
                    let idx = i as usize % (if cur.len() > 1 { cur.len() - 1 } else { cur.len() });
                    cur[idx] = format!("{} ", cur[idx]);  // Add one space to the chosen word
                }
                res.push(cur.join(""));  // Join words with the extra spaces added
                cur.clear();              // Reset for next line
                num_of_letters = 0;       // Reset letter count
            }
            cur.push(word.clone());       // Add current word to the line
            num_of_letters += word.len() as i32; // Update letter count
        }

        // Handle the last line: left-justified with spaces padded to max_width
        let last_line = cur.join(" ");
        res.push(format!("{:<width$}", last_line, width=max_width as usize));

        res
    }
}
