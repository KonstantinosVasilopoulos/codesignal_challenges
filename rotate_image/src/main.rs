fn solution(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n: usize = a.len();

    // Transpose the image
    for _layer in 0..(n / 2) {
        for i in 0..n {
            for j in i..n {
                swap(&mut a, i, j, j, i);
            }
        }
    }

    // Flip the image horizontally
    let mut start: usize;
    let mut end: usize;
    for i in 0..n {
        start = 0;
        end = n - 1;
        while end.abs_diff(start) > 1 {
            swap(&mut a, i, start, i, end);
            start += 1;
            end -= 1;
        }
    }

    a
}

// Swap two image elements
fn swap(image: &mut Vec<Vec<i32>>, i1: usize, j1: usize, i2: usize, j2: usize) {
    let temp = image[i1][j1];
    image[i1][j1] = image[i2][j2];
    image[i2][j2] = temp;
}

// Helper function
#[allow(dead_code)]
fn print_image(image: &Vec<Vec<i32>>) {
    let mut line: String;
    for i in 0..image.len() {
        line = String::new();
        for j in 0..image.len() {
            line.push(' ');
            line.push_str(&image[i][j].to_string()[..]);
        }
        println!("{:?}", line.trim_start());
    }
    println!();
}

fn main() {
    // Test the solution
    let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
    assert_eq!(expected, solution(a));
}
