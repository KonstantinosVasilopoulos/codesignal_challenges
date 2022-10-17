fn solution(n: i32) -> i32 {
    let mut num = n;
    let mut degree = 0;
    while num > 9 {
        num = calculate_digit_sum(&num);
        degree += 1;
    }

    degree
}

// Calculate the sum of all digits of an integer
fn calculate_digit_sum(n: &i32) -> i32 {
    let mut num = *n;
    let mut sum = 0;
    let base = 10;
    while num != 0 {
        sum += num % base;
        num /= base;
    }

    sum
}

// Test the solution
fn main() {
    assert_eq!(solution(5), 0);
    assert_eq!(solution(100), 1);
    assert_eq!(solution(91), 2);
}
