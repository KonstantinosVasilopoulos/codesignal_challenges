fn solution(input_string: String) -> String {
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut shifted_string = String::from("");
    for c in input_string.chars() {
        // Search for every character in the alphabet except for the last one
        for i in 0..alphabet.len() - 1 {
            if c == alphabet.chars().nth(i).unwrap() {
                // Push the next character into the result string
                shifted_string.push(alphabet.chars().nth(i + 1).unwrap());
            }
        }

        // If this is the last character of the alphabet,
        // then the first one has to be pushed into the result string
        if c == alphabet.chars().rev().nth(0).unwrap() {
            shifted_string.push(alphabet.chars().nth(0).unwrap());
        }
    }

    shifted_string
}

fn main () {
    let input_string = String::from("crazy");
    println!("{:?}", solution(input_string));
}
