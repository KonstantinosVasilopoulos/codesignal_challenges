use std::cmp::{max, min};

fn solution(value1: i32, weight1: i32, value2: i32, weight2: i32, max_weight: i32) -> i32 {
    // Find the item that maximizes the value
    let max_value = max(value1, value2);
    let max_value_weight = if max_value == value1 { weight1 } else { weight2 };

    let mut value: i32 = 0;
    let mut current_weight: i32 = 0;
    if max_value_weight <= max_weight {
        value += max_value;
        current_weight += max_value_weight;
    }

    // Attempt to add the other weight
    let other_weight = if max_value == value1 { weight2 } else { weight1 };
    if other_weight <= max_weight - current_weight {
        value += min(value1, value2);
    }

    value
}

fn main() {
    // Test the solution
    assert_eq!(10, solution(10, 5, 6, 4, 8));
    assert_eq!(16, solution(10, 5, 6, 4, 9));
    assert_eq!(7, solution(5, 3, 7, 4, 6));
    assert_eq!(15, solution(15, 2, 20, 3, 2));
}
