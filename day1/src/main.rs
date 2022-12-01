pub fn main() {
    let mut elves = vec![];
    let mut sum = 0;
    include_str!("../input.txt").lines().for_each(|calorie| {
        if !calorie.is_empty() {
            sum += calorie.parse::<i32>().unwrap();
        } else {
            elves.push(sum);
            sum = 0;
        }
    });
    elves.sort_by(|a, b| b.cmp(a));

    // Day1a
    println!("1a: {}", elves[0]);

    // Day1b
    elves.truncate(3);
    println!("1b: {}", elves.into_iter().sum::<i32>())
}
