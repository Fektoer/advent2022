fn main() {
    let input = include_str!("../input.txt");
    println!("6a: {}", analyze_window(input, 4));
    println!("6b: {}", analyze_window(input, 14));
}

fn analyze_window(input: &str, size: usize) -> i32 {
    let input_as_vec = input.chars().collect::<Vec<char>>();
    let mut windows = input_as_vec.windows(size);

    let mut index = 0;
    loop {
        index += 1;
        let value = windows.next();
        if value == None {
            break;
        } else {
            if is_unique(value.unwrap().to_vec()) {
                break;
            }
        }
    }
    return index + (size as i32 - 1);
}

fn is_unique(vec_of_char: Vec<char>) -> bool {
    let mut cloned_vec = vec_of_char.clone();
    cloned_vec.sort();
    cloned_vec.dedup();
    vec_of_char.len() == cloned_vec.len()
}
