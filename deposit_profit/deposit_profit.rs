fn solution(deposit: i32, rate: i32, threshold: i32) -> i32 {
    let mut years = 0;
    let mut money = deposit as f32; // deposit mutable copy
    let rate_percentage: f32 = rate as f32 / 100.0;
    while money < threshold as f32 {
        years += 1;
        money += money * rate_percentage;
    }

    years
}

fn main() {
    println!("{:?}", solution(100, 20, 170)); // 3
}
