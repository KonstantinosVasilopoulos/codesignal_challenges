fn solution(n: i32) -> bool {
    for digit in n.to_string().chars() {
        if digit as i32 % 2 != 0 {
            return false;
        }
    }

    true
}

fn main() {
    let mut n = 248622;
    println!("{:?}", solution(n));
    n = 642386;
    println!("{:?}", solution(n));
}
