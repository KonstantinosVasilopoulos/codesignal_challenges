/*
We can assign a number at each column instead of a character. So, instead of
columns A, B, C, D, E, etc we have columns 1, 2, 3, 4, 5, etc. A cell is colored white only
if either the row's or the column's number is even. If both are even or none are, then
the cell is definitely black.
*/
fn solution(cell1: String, cell2: String) -> bool {
    // Get the number coordinates of each cell
    let (x1, y1) = calculate_cell_coords(cell1);
    let (x2, y2) = calculate_cell_coords(cell2);

    // Check that both cells are of the same color
    cell_is_white(x1, y1) == cell_is_white(x2, y2)
}

// Read the cell's coordinates and return the cell's column & row as integers
fn calculate_cell_coords(cell: String) -> (u8, u8) {
    // Find the column's number
    let alphabet = String::from("ABCDEFGH");
    let column_str = cell.chars().nth(0).unwrap();
    let mut column: u8 = 0;
    for i in 0..alphabet.len() {
        if column_str == alphabet.chars().nth(i).unwrap() {
            column = i as u8 + 1;
        }
    }

    // Find the row's number
    let row: u8 =  cell.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
 
    (column, row)
}

// Return true if a cell's color is white
fn cell_is_white(x: u8, y: u8) -> bool {
    (x % 2 == 0 && y % 2 != 0) || (x % 2 != 0 && y % 2 == 0)
}

// For testing
fn main() {
    println!("{:?}", solution("A1".to_string(), "C3".to_string()));
    println!("{:?}", solution("A1".to_string(), "H3".to_string()));
}
