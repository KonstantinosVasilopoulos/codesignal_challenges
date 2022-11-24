fn solution(input_string: &String) -> bool {
    if input_string.len() != 17 {
        return false;
    }

    // Iterate over the string and check each character
    let mut i: u8 = 0;
    let hex = String::from("0123456789ABCDEF");
    for c in input_string.chars() {
        i += 1;

        // Check for hypen
        let is_hypen = i % 3 == 0 && i <= 15 && c == '-';

        // Check for hexadecimal
        let is_hex = i % 3 != 0 && i <= 17 && hex.contains(&c.to_string().to_uppercase());

        if !is_hypen && !is_hex {
            return false;
        }
    }

    true
}

// Test the solution
fn main() {
    let mut input_string = String::from("00-1B-63-84-45-E6");
    assert!(solution(&input_string), "\"{}\" is a valid MAC48 address.", input_string);

    input_string = String::from("Z1-1B-63-84-45-E6");
    assert!(!solution(&input_string), "\"{}\" is not a valid MAC48 address.", input_string);

    input_string = String::from("not a MAC-48 address");
    assert!(!solution(&input_string), "\"{}\" is not a valid MAC48 address.", input_string);
}
