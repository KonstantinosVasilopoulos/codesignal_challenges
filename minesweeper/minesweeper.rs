fn solution(matrix: Vec<Vec<bool>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..matrix.len() {
        result.push(vec![]);
        for j in 0..matrix[i].len() {
            result[i].push(calculace_cell_value(&matrix, i, j));
        }
    }

    result
}

// Return a cell's neighbouring mines count
fn calculace_cell_value(matrix: &Vec<Vec<bool>>, i: usize, j: usize) -> i32 {
    let mut mines_count = 0;
    let (max_j, max_i) = (matrix[0].len() - 1, matrix.len() - 1);
    let (i_i32, j_i32) = (i as i32, j as i32);

    // Top-left neighbour
    if i_i32 - 1 >= 0 && j_i32 - 1 >= 0 {
        mines_count += matrix[i-1][j-1] as i32;
    }

    // Top
    if i_i32 - 1 >= 0 {
        mines_count += matrix[i-1][j] as i32;
    }

    // Top-right
    if i_i32 - 1 >= 0 && j + 1 <= max_j {
        mines_count += matrix[i-1][j+1] as i32;
    }

    // Right
    if j + 1 <= max_j {
        mines_count += matrix[i][j+1] as i32;
    }

    // Bottom-right
    if i + 1 <= max_i && j + 1 <= max_j {
        mines_count += matrix[i+1][j+1] as i32;
    }

    // Bottom
    if i + 1 <= max_i {
        mines_count += matrix[i+1][j] as i32;
    }

    // Bottom-left
    if i + 1 <= max_i && j_i32 - 1 >= 0 {
        mines_count += matrix[i+1][j-1] as i32;
    }

    // Left
    if j_i32 - 1 >= 0 {
        mines_count += matrix[i][j-1] as i32;
    }

    mines_count
}

fn main() {
    let matrix: Vec<Vec<bool>> = vec![
        vec![true, false, false],
        vec![false, true, false],
        vec![false, false, false]
    ];
    println!("{:?}", solution(matrix));
}
