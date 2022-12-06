fn main() {
    let input = include_str!("../input.txt");
    println!("6a: {}", analyze_window(input, 4));
    println!("6b: {}", analyze_window(input, 14));
}

fn analyze_window(input: &str, size: i32) -> i32 {
    let input_as_vec = input.chars().collect::<Vec<char>>();
    let mut windows = input_as_vec.windows(size as usize).enumerate();

    loop {
        let (index, value) = windows.next().unwrap();
        if value.len() == 0 {
            return index as i32;
        } else {
            if contains_only_unique(value.to_vec()) {
                return index as i32 + (size);
            }
        }
    }
}

fn contains_only_unique(vec_of_char: Vec<char>) -> bool {
    let mut cloned_vec = vec_of_char.clone();
    cloned_vec.sort();
    cloned_vec.dedup();
    vec_of_char.len() == cloned_vec.len()
}
