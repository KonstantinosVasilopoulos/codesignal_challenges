fn solution(s: String) -> i32 {
    let mut symbols_found: Vec<char> = Vec::new();
    for c in s.chars() {
        if !symbols_found.contains(&c) {
            symbols_found.push(c);
        }
    }

    symbols_found.len() as i32
}

fn main() {
    assert_eq!(3, solution(String::from("cabca")));
    assert_eq!(10, solution(String::from("asdfgb5432")));
}
