fn solution(input_array: Vec<i32>, elem_to_replace: i32, substitution_elem: i32) -> Vec<i32> {
    let mut output_array: Vec<i32> = vec![];
    for i in 0..input_array.len() {
        if input_array[i] == elem_to_replace {
            output_array.push(substitution_elem);
        } else {
            output_array.push(input_array[i]);
        }
    }

    output_array
}

fn main() {
    let input_array = vec![1, 2, 1];
    println!("{:?}", solution(input_array, 1, 3)); // [3, 2, 3]
}
