fn solution(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Apply the kernel to all 3x3 boxes in the image
    let mut blurred_image: Vec<Vec<i32>> = vec![vec![]];
    let (mut i, mut j) = (1, 1); // First box is always at (1, 1)
    let (width, height) = (image[0].len() - 1, image.len() - 1); // max j & i
    let mut blurred_image_row: usize  = 0;
    loop {
        println!("Applying kernel to ({:?}, {:?})...", i, j); // -0
        let image_box = get_box(&image, i, j);
        let box_blur = apply_kernel(image_box);

        blurred_image[blurred_image_row].push(box_blur);

        if j + 2 <= width {
            // Next column
            j += 1;
        } else if i + 2 <= height {
            // Next row
            i += 1;
            j = 1;
            blurred_image.push(vec![]);
            blurred_image_row += 1;
        } else {
            break;
        }
    }

    blurred_image
}

// Calculate & return the box blur value of a given 3x3 box
fn apply_kernel(image_box: Vec<Vec<i32>>) -> i32 {
    let mut s: i32 = 0;
    for row in image_box.iter() {
        for i in row.iter() {
            s += i;
        }
    }

    s / 9
}

// Get the 3x3 box of a given point
fn get_box(image: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<Vec<i32>> {
    let mut box_vec: Vec<Vec<i32>> = vec![];
    for row in (i - 1)..=(i + 1) {
        let mut row_vec: Vec<i32> = vec![];
        for n in &image[row][j-1..=j+1] {
            row_vec.push(*n);
        }

        box_vec.push(row_vec);
    }

    box_vec
}

fn test_apply_kernel() {
    let image: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 7, 1], vec![1, 1, 1]];
    let blur_value = apply_kernel(image);
    assert_eq!(blur_value, 1, "kernel returns wrong value: given {:?} != expected {:?}", blur_value, 1);
}

fn test_get_box() {
    let image = vec![
        vec![7, 4, 0, 1], 
        vec![5, 6, 2, 2], 
        vec![6, 10, 7, 8], 
        vec![1, 4, 2, 0]
    ];
    assert_eq!(get_box(&image, 1, 1), vec![vec![7, 4, 0], vec![5, 6, 2], vec![6, 10, 7]]);
    assert_eq!(get_box(&image, 1, 2), vec![vec![4, 0, 1], vec![6, 2, 2], vec![10, 7, 8]]);
}

// Run all test functions
fn run_tests() {
    test_apply_kernel();
    test_get_box();
}
