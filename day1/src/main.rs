pub fn main() {
    let mut elves = include_str!("../input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    elves.sort_by(|a, b| b.cmp(a));

    // Day1a
    println!("1a: {}", elves[0]);

    // Day1b
    println!("1b: {}", elves.into_iter().take(3).sum::<i32>())
}
