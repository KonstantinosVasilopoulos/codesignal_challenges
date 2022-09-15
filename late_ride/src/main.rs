fn solution(n: i32) -> i32 {
    // Calculate the hours and minutes
    let hours: i32 = n / 60;
    let minutes: i32 = n % 60;

    // Find the sum of all the digits of the hh:mm time format
    let mut digits_sum: i32 = 0;
    digits_sum += hours / 10;
    digits_sum += hours % 10;
    digits_sum += minutes / 10;
    digits_sum += minutes % 10;

    digits_sum
}

fn main() {
    // Test the solution
    assert_eq!(4, solution(240));
    assert_eq!(14, solution(808));
}
