fn main() {
    let (sum_a, sum_b) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(acc_a, acc_b), (he, me)| match (he, me) {
            ("A", "X") => (acc_a + 3 + 1, acc_b + 0 + 3),
            ("A", "Y") => (acc_a + 6 + 2, acc_b + 3 + 1),
            ("A", "Z") => (acc_a + 0 + 3, acc_b + 6 + 2),
            ("B", "X") => (acc_a + 0 + 1, acc_b + 0 + 1),
            ("B", "Y") => (acc_a + 3 + 2, acc_b + 3 + 2),
            ("B", "Z") => (acc_a + 6 + 3, acc_b + 6 + 3),
            ("C", "X") => (acc_a + 6 + 1, acc_b + 0 + 2),
            ("C", "Y") => (acc_a + 0 + 2, acc_b + 3 + 3),
            ("C", "Z") => (acc_a + 3 + 3, acc_b + 6 + 1),
            _ => unreachable!(),
        });
    println!("2a: {}", sum_a);
    println!("2b: {}", sum_b)
}
