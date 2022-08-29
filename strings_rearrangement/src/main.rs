fn solution(mut input_array: Vec<String>) -> bool {
    let k = input_array.len();
    let mut output: Vec<bool> = vec![];
    permute(&mut input_array, k, &mut output);
    output.contains(&true)
}

// Heap's algorithm implementation
fn permute(input_array: &mut Vec<String>, k: usize, output: &mut Vec<bool>) {
    if k == 1 {
        output.push(check_permutation(input_array.to_vec()));
        return
    } else {
        permute(input_array, k - 1, output);

        for i in 0..k-1 {    assert_eq!(true, check_permutation(vec![String::from("aa"), String::from("ab"), String::from("bb")]));

            if k % 2 == 0 {
                input_array.swap(i, k-1);
            } else {
                input_array.swap(0, k-1);
            }

            permute(input_array, k - 1, output);
        }
    }
}

// Check whether a permutation is valid
fn check_permutation(permutation: Vec<String>) -> bool {
    let mut counter: u8;
    for i in 0..permutation.len()-1 {
        counter = 0;
        for (j, c) in permutation[i].chars().enumerate() {
            if c != permutation[i+1].chars().nth(j).unwrap() {
                counter += 1;
            }
        }

        if counter != 1 {
            return false;
        }
    }

    true
}

fn main() {
    let mut input_array = vec![String::from("aba"), String::from("bbb"), String::from("bab")];
    assert_eq!(false, solution(input_array));

    input_array = vec![String::from("ab"), String::from("bb"), String::from("aa")];
    assert_eq!(true, solution(input_array));

    input_array = vec![String::from("abc"), String::from("abx"), String::from("axx"), String::from("abx"), String::from("abc")];
    assert_eq!(true, solution(input_array));
}
