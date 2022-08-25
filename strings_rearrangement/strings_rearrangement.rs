// TODO: Find a working solution! 
use std::collections::{LinkedList, VecDeque};

fn solution(input_array: Vec<String>) -> bool {
    //  Linked list for testing all permutations
    let permutations: LinkedList<String> = LinkedList::new();

    // Queue for storing all the strings
    let queue: VecDeque<String> = VecDeque::from(input_array);

    /* Iterate over each string in the array and test all
    possible permutations starting with that string. */
    for i in 0..input_array.len() {
        permutations.append(queue.pop_front());
        for j in 0..input_array.len() - 1 {
            let next_string = queue.pop_front();
            if differ_by_one(permutations.front(), &next_string) {
                // Add the next string to the permutations list
                permutations.append(next_string);
            }
        }

        // Check if the permutation is valid
        if permutations.len() == input_array.len() {
            return true;
        }

        // Empty the permutations list back to the queue
        for j in 0..permutations.len() {
            queue.push_back(permutations.pop_back());
        }
    }

    false
}

// Return whether two string differ by exactly one character
fn differ_by_one(s1: &String, s2: &String) -> bool {
    // Iterate over the string slices and count the number of different
    // characters at the same index    
    let mut count: u16 = 0;
    for i in 0..s1.len() {
        if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
            count += 1;

            // Avoid unecessary loops
            if count >= 2 {
                return false;
            }
        }
    }

    count == 1
}

fn main() {
    let mut input_array: Vec<String> = vec![
        String::from("aba"),
        String::from("bbb"),
        String::from("bab"),
    ];
    println!("{:?}", solution(input_array));
    input_array = vec![
        String::from("ab"),
        String::from("bb"),
        String::from("aa"),
    ];
    println!("{:?}", solution(input_array));
}
