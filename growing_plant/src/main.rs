fn solution(up_speed: i32, down_speed: i32, desired_height: i32) -> i32 {
    if up_speed <= down_speed {
        return -1;
    }

    // Calculate the plant's height for each day, until the desired height is reached
    let mut day: i32 = 0;
    let mut height: i32 = 0;
    loop {
        day += 1;
        height += up_speed;

        // Check whether the plant is high enough
        if height >= desired_height {
           return day; 
        }

        height -= down_speed;
    }
}

fn main() {
    // Test the solution
    assert_eq!(10, solution(100, 10, 910));
}

