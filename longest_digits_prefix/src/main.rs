fn solution(input_string: String) -> String {
    let numbers = String::from("0123456789");
    let mut prefix = String::new();
    for c in input_string.chars().into_iter() {
        if numbers.contains(c) {
            prefix.push(c);
        } else {
            break;
        }
    }

    prefix
}

fn main() {
    // Test the solution
    assert_eq!(String::from("123"), solution(String::from("123aa1")));
}
