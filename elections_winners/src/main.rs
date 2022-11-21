fn solution(votes: Vec<i32>, k: i32) -> i32 {
    let mut possible_winners = 0;
    let mut can_be_winner: bool;
    for (i, v) in votes.iter().enumerate() {
        can_be_winner = true;
        for (j, z) in votes.iter().enumerate() {
            if i == j {
                continue;
            }

            if *v + k <= *z {
                can_be_winner = false;
                break;
            }
        }

        if can_be_winner {
            possible_winners += 1;
        }
    }

    possible_winners
}

// Test the solution
fn main() {
    assert_eq!(2, solution(vec![2, 3, 5, 2], 3));
}
