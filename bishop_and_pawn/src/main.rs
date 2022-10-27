#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    row: i8,
    column: i8,
}

trait CellBehavior {
    // Constructor
    fn new(row: i8, column: i8) -> Self;

    // Constructor for getting cell from A1-H7 coordinates
    fn new_from_coordinates(coords: String) -> Self;
}

impl Cell {
    const CELLS_X_COORDS: [char; 8] = [
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
    ];

    const CELLS_Y_COORDS: [char; 10] = [
        '0',
        '1',
        '2',
        '3',
        '4',
        '5',
        '6',
        '7',
        '8',
        '9',
    ];
}

impl CellBehavior for Cell {
    fn new(row: i8, column: i8) -> Self {
        Cell { row: row, column: column }
    }

    fn new_from_coordinates(coords: String) -> Self {
        // Extract the coordinates from the string
        let mut row = 0i8;
        let mut column = 0i8;
        for (i, c) in coords.chars().enumerate() {
            // Parse the column first
            if i == 0 {
                for j in 0..8 {
                    if c == Cell::CELLS_X_COORDS[j] {
                        break;
                    }
                    column += 1;
                }
            } else { // The row, counting from zero
                for j in 0..10 {
                    if c == Cell::CELLS_Y_COORDS[j] {
                        row -= 1;
                        break;
                    }
                    row += 1;
                }
            }
        }

        Cell { row: row, column: column }
    }
}

fn solution(bishop: String, pawn: String) -> bool {
    // Validate input
    if bishop == pawn {
        panic!("Bishop and pawn cannot occupy the same cell at the same time.");
    }

    // Convert coordinates to purely number-based cells
    let bishop_cell = Cell::new_from_coordinates(bishop);
    let pawn_cell = Cell::new_from_coordinates(pawn);

    // Check the top-left diagonal
    if check_diagonal(&bishop_cell, &pawn_cell, -1, -1) {
        return true;
    }

    // Check the bottom-right diagonal
    if check_diagonal(&bishop_cell, &pawn_cell, 1, 1) {
        return true;
    }

    // Check the top-right diagonal
    if check_diagonal(&bishop_cell, &pawn_cell, -1, 1) {
        return true;
    }

    // Check the bottom-left diagonal
    if check_diagonal(&bishop_cell, &pawn_cell, 1, -1) {
        return true;
    }

    false
}

/*
Check a single diagonal of the bishop for the pawn.
There are four diagonals:
    1. top-left
    2. bottom-right
    3. top-right
    4. bottom-left
*/
fn check_diagonal(bishop: &Cell, pawn: &Cell, row_direction: i8, column_direction: i8) -> bool {
    let mut cell = Cell::new(
        bishop.row.clone(),
        bishop.column.clone()
    );
    loop {
        // Check for the pawn
        if cell == *pawn {
            return true;
        }

        // Break if the cell is about to exit the board
        let next_row = cell.row + row_direction;
        let next_column = cell.column + column_direction;
        if next_row < 0 || next_row > 7 || next_column < 0 || next_column > 7 {
            break;
        }

        // Next cell
        cell.row  = next_row;
        cell.column  = next_column;
    }

    false
}

// Test solution
fn main() {
    assert!(solution(String::from("a1"), String::from("c3")), "Bishop A1 can eat pawn at C3.");
    assert!(!solution(String::from("h1"), String::from("h3")), "Bishop H1 cannot eat pawn at H3.");
}
