fn solution(input_string: String) -> char {
    let numbers = "0123456789";
    for c in input_string.chars() {
        if numbers.contains(c) {
            return c;
        }
    }

    '0'
}

fn main() {
    assert_eq!('1', solution(String::from("var1__Int")));
    assert_eq!('2', solution(String::from("q2q-q")));
    assert_eq!('0', solution(String::from("0ss")));
}
