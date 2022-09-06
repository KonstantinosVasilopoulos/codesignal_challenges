use std::cmp::{max, min};

fn solution(cities: i32, roads: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_roads: Vec<Vec<i32>> = Vec::new();

    // Deep-copy the roads vector
    let mut all_roads: Vec<Vec<i32>> = Vec::new();
    for connection in roads {
        all_roads.push(connection.clone());
    }

    // Iterate over the cities and check each intercity connection
    for city in 0..cities {
        for other_city in 0..cities {
            if city == other_city {
                continue;
            }

            if !check_connection(city, other_city, &all_roads) {
                // Establish a new connection and amend the roads vector
                let new_connection = vec![min(city, other_city), max(city, other_city)];
                new_roads.push(new_connection.clone());
                all_roads.push(new_connection);
            }
        }
    }

    // Sort the array lexicographically before returning it
    new_roads.sort();
    new_roads
}

// Check whether two cities are connected
fn check_connection(i: i32, j: i32, roads: &Vec<Vec<i32>>) -> bool {
    for connection in roads {
        if connection.contains(&i) && connection.contains(&j) {
            return true;
        }
    }

    false
}

fn main() {
    // Test the check_connection function
    let roads = vec![vec![0, 1], vec![1, 2], vec![2, 0],];
    assert!(check_connection(0, 2, &roads));
    assert!(check_connection(1, 0, &roads));
    assert!(!check_connection(0, 3, &roads));

    // Test the solution
    let outcome = vec![vec![0, 3], vec![1, 3], vec![2, 3]];
    assert_eq!(
        outcome,
        solution(4, roads)
    );
}
