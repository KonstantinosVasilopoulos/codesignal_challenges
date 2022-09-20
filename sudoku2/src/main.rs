fn solution(grid: Vec<Vec<char>>) -> bool {
    for i in 0..grid.len() {
        // Check the columns and the rows
        if !(check_column(&grid, i) && check_row(&grid, i)) {
            return false;
        }

        // Check the 3x3 subgrid
        if i % 3 == 0 {
            for j in (0..grid.len()).step_by(3) {
                if !check_subgrid(&grid, i, j) {
                    return false;
                }
            }
        }
    }

    true
}

// Check whether a single sudoku column is valid
fn check_column(grid: &Vec<Vec<char>>, i: usize) -> bool {
    let mut numbers_count: [u8; 9] = [0; 9];
    for number in &grid[i] {
        if *number != '.' {
            numbers_count[number.to_digit(10).unwrap() as usize - 1] += 1;
        }
    }

    // Ensure that all numbers exist either one or zero times
    for count in numbers_count {
        if count > 1 {
            return false;
        }
    }

    true
}

// Check whether a single sudoku row is valid
fn check_row(grid: &Vec<Vec<char>>, j: usize) -> bool {
    let mut numbers_count: [u8; 9] = [0; 9];
    for i in 0..grid.len() {
        if grid[i][j] != '.' {
            numbers_count[grid[i][j].to_digit(10).unwrap() as usize - 1] += 1;
        }
    }

    // Ensure that all numbers exist either one or zero times
    for count in numbers_count {
        if count > 1 {
            return false;
        }
    }

    true
}

// Check whether a 3x3 grid is valid
// The subgrid's top-left cell is used as its coordinates
fn check_subgrid(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // Panic if the given coords are off the grid
    if i + 2 > grid.len() || j + 2 > grid.len() {
        panic!("Off-grid coordinates provided.");
    }

    let mut numbers_count: [u8; 9] = [0; 9];
    for col in i..(i + 3) {
        for row in j..(j + 3) {
            if grid[col][row] != '.' {
                numbers_count[grid[col][row].to_digit(10).unwrap() as usize - 1] += 1;
            }
        }
    }

    // Ensure that all numbers exist either one or zero times
    for count in numbers_count {
        if count > 1 {
            return false;
        }
    }

    true
}

fn main() {
    let grid1 = vec![
        vec!['.', '.', '.', '1', '4', '.', '.', '2', '.'],
        vec!['.', '.', '6', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '1', '.', '.', '.', '.', '.', '.'],
        vec!['.', '6', '7', '.', '.', '.', '.', '.', '9'],
        vec!['.', '.', '.', '.', '.', '.', '8', '1', '.'],
        vec!['.', '3', '.', '.', '.', '.', '.', '.', '6'],
        vec!['.', '.', '.', '.', '.', '7', '.', '.', '.'],
        vec!['.', '.', '.', '5', '.', '.', '.', '7', '.'],
    ];
    let grid2 = vec![
        vec!['.', '.', '.', '.', '2', '.', '.', '9', '.'],
        vec!['.', '.', '.', '.', '6', '.', '.', '.', '.'],
        vec!['7', '1', '.', '.', '7', '5', '.', '.', '.'],
        vec!['.', '7', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '8', '3', '.', '.', '.'],
        vec!['.', '.', '8', '.', '.', '7', '.', '6', '.'],
        vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
        vec!['.', '1', '.', '2', '.', '.', '.', '.', '.'],
        vec!['.', '2', '.', '.', '3', '.', '.', '.', '.'],
    ];

    // Test the check_column function
    assert!(check_column(&grid1, 0));
    assert!(check_column(&grid1, 2));
    assert!(!check_column(&grid2, 2));

    // Test the check_row function
    assert!(check_row(&grid1, 0));
    assert!(check_row(&grid2, 2));

    // Test the check_subgrid function
    assert!(check_subgrid(&grid1, 0, 0));
    assert!(check_subgrid(&grid1, 1, 1));
    assert!(!check_subgrid(&grid2, 2, 0));
    
    // Test the solution
    assert!(solution(grid1));
    assert!(!solution(grid2));
}
