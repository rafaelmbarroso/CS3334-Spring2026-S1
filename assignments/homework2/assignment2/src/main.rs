fn most_frequent_word(text: &str) -> (String, usize) {
    
    // Take a string and first split it up into words of which we can count the frequeny.
    let mut max_word = String::new(); // Empty string to store the most frequent word
    let mut max_count = 0; // Max word counter
    for word in text.split_whitespace() {
        let count = text.matches(word).count();
        if count > max_count {
            max_word = word.to_string();
            max_count = count;
        }
    }
    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}