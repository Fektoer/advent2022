fn main() {
    let sum_priorities = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| {
            let mut duplicate_char = '\0';
            for char in a.chars() {
                if b.contains(char) {
                    duplicate_char = char;
                }
            }
            return duplicate_char;
        })
        .map(|char| {
            if char.is_lowercase() {
                return char as i32 - 96;
            } else {
                return char as i32 - 38;
            }
        })
        .sum::<i32>();

    println!("2a: {}", sum_priorities);
}
