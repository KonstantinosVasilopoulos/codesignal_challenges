fn solution(symbol: char) -> bool {
    String::from("0123456789").chars().collect::<Vec<char>>().contains(&symbol)    
}

// Test the solution
fn main() {
    assert!(solution('0'), "'0' is a digit.");
    assert!(!solution('-'), "'-' is not a digit.");
    assert!(!solution('@'), "'@' is not a digit.");
}
