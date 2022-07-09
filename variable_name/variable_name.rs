fn solution(name: String) -> bool {
    let acceptable_characters: String = String::from("_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");

    // Check if the variable starts with a number
    for c in acceptable_characters[acceptable_characters.len()-10..=acceptable_characters.len()-1].chars() {
        if name.starts_with(c) {
            return false;
        }
    }

    // Search for any unacceptable characters
    for c in name.chars() {
        if !acceptable_characters.contains(c) {
            return false;
        }
    }

    true
}

fn main() {
    let acceptable_characters: String = String::from("_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
    let mut name = String::from("var_1__Int");
    println!("{:?}", solution(name));
    name = String::from("qq-q");
    println!("{:?}", solution(name));
    name = String::from("2w2");
    println!("{:?}", solution(name));
}
