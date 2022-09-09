fn solution(input_array: Vec<i32>, k: i32) -> i32 {
    let mut sum: i32;
    let mut max_sum: i32 = i32::MIN;
    for i in 0..input_array.len() {
        // Decide if done
        if i > input_array.len() - (k as usize) {
            break;
        }

        // Find the sum for the i-element
        sum = 0;
        for j in 0..(k as usize) {
            sum += input_array[i + j];
        }

        // Check if the current sum is the maximum
        if sum > max_sum {
            max_sum = sum;
        }
    }

    max_sum
}

fn main() {
    // Test the solution
    assert_eq!(8, solution(vec![2, 3, 5, 1, 6], 2));
}
