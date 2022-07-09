use std::collections::HashMap;

fn solution(s: String) -> char {
    // Create a hashmap for counting character instances
    let mut character_count: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if character_count.contains_key(&c) {
            character_count.insert(c, character_count[&c] + 1);
        } else {
            character_count.insert(c, 1);
        }
    }

    // Find the first character without a second occurance
    for c in s.chars() {
        if character_count[&c] == 1 {
            return c;
        }
    }

    '_'
}

fn main() {
    let mut s = String::from("abacabad");
    println!("{:?}", solution(s));
    s = String::from("abacabaabacaba");
    println!("{:?}", solution(s));
}
