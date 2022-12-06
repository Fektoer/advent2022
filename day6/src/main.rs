use std::collections::HashSet;
fn main() {
    let input = include_str!("../input.txt");
    println!("6a: {}", analyze_window(input, 4));
    println!("6b: {}", analyze_window(input, 14));
}

fn analyze_window(input: &str, size: i32) -> i32 {
    let input_as_vec = input.chars().collect::<Vec<char>>();
    let mut windows = input_as_vec.windows(size as usize).enumerate();

    if let Some((index, _)) = windows.find(|(_, value)| contains_only_unique(value.to_vec())) {
        return index as i32 + size;
    } else {
        return 0;
    };
}

fn contains_only_unique(vec_of_char: Vec<char>) -> bool {
    let cloned_set: HashSet<char> = HashSet::from_iter(vec_of_char.iter().cloned());
    vec_of_char.len() == cloned_set.len()
}
