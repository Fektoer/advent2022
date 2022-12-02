fn main() {
    let score = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold(0, |acc, (x, y)| match (x, y) {
            ("A", "X") => acc + 4,
            ("A", "Y") => acc + 8,
            ("A", "Z") => acc + 3,
            ("B", "X") => acc + 1,
            ("B", "Y") => acc + 5,
            ("B", "Z") => acc + 9,
            ("C", "X") => acc + 7,
            ("C", "Y") => acc + 2,
            ("C", "Z") => acc + 6,
            _ => unreachable!(),
        });
    println!("a1: {}", score)
}
