use std::collections::HashMap;

fn solution(input_string: String) -> bool {
    // Create a hashmap for storing all characters in the alphabet alongside
    // the times they appear in the input string
    let mut char_counts: HashMap<char, u8> = HashMap::new();
    for c in input_string.chars() {
        if !char_counts.contains_key(&c) {
            char_counts.insert(c, 1);
        } else {
            *char_counts.get_mut(&c).unwrap() += 1;
        }
    }
    
    // Iterate over the characters in the alphabet and make sure each character
    // appears no more times than the previous one
    let alphabet = String::from("bcdefghijklmnopqrstuvwxyz");
    let mut previous = 'a';
    for c in alphabet.chars() {
        if char_counts.get(&c).unwrap_or(&0) > char_counts.get(&previous).unwrap_or(&0) {
            return false;
        }

        previous = c;
    }

    true
}

// Test solution
fn main() {
    assert!(solution(String::from("bbbaacdafe")));
    assert!(!solution(String::from("aabbb")));
    assert!(!solution(String::from("bbc")));
}
