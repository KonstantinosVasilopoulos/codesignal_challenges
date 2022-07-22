fn solution(a: Vec<i32>) -> i32 {
    // Iterate over the array and calculate the distance of each element
    let mut min_distance = i32::MAX;
    let mut min_distance_element = a[0];
    for element in &a {
        let distance = calculate_distance(&a, element);

        // Find the minimum distance
        if min_distance > distance {
            min_distance = distance;
            min_distance_element = *element;
        }
    }

    min_distance_element
}

// Calculate the distance of an element of the array
fn calculate_distance(a: &Vec<i32>, element: &i32) -> i32 {
    let mut distance: i32 = 0;
    for i in 0..a.len() {
        distance += (a[i] - element).abs();
    }

    distance
}

fn main() {
    let mut a: Vec<i32> = vec![2, 4, 7];
    println!("{:?}", solution(a));
    a = vec![2, 3];
    println!("{:?}", solution(a));
}
