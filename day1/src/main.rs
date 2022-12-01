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
    println!("{}", elves[0]);

    // Day1b
    println!("{}", elves[0] + elves[1] + elves[2])
}
