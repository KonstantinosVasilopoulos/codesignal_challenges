use std::collections::HashMap;

fn solution(a: Vec<i32>) -> i32 {
    let mut items_count: HashMap<i32, i32> = HashMap::new();
    
    // Iterate over the array and find the first second occurance
    for i in 0..a.len() {
        if items_count.contains_key(&a[i]) {
            return a[i];
        } else {
            items_count.insert(a[i], 1);
        }
    }

    -1
}

fn main() {
    let mut a = vec![2, 1, 3, 5, 3, 2];
    println!("{:?}", solution(a));
    a = vec![2, 2];
    println!("{:?}", solution(a));
    a = vec![2, 4, 3, 5, 1];
    println!("{:?}", solution(a));
}
