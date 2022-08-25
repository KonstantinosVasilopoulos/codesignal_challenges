fn solution(input_array: Vec<i32>, k: i32) -> Vec<i32> {
    let mut j = 1;
    let mut without_kth: Vec<i32> = vec![];
    for i in 0..input_array.len() {
        if j * k - 1 != i as i32 {
            without_kth.push(input_array[i]);
        } else {
            j += 1;
        }
    }

    without_kth
}

fn main() {
    let input_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", solution(input_array, 3));
}
