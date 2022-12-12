use regex::Regex;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut cycles: Vec<(i32, i32)> = Vec::new();
    include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .for_each(|command| {
            let cycle = command.0 as i32 + 1;
            let re_noop = Regex::new(r"noop").unwrap();
            match re_noop.captures(command.1) {
                Some(_) => {
                    cycles.push((cycle, 0));
                }
                None => {}
            }
            let re_add = Regex::new(r"^addx").unwrap();
            match re_add.captures(command.1) {
                Some(_) => {
                    let value = command.1.split_once(" ").unwrap().1.parse::<i32>().unwrap();
                    cycles.push((cycle, 0));
                    cycles.push((cycle, value));
                }
                None => {}
            }
        });

    println!(
        "10a {:?} in {:?}",
        [20, 60, 100, 140, 180, 220]
            .into_iter()
            .fold(0, |mut sum, curr| {
                sum += calc_register(cycles.clone(), curr) * curr;
                sum
            }),
        start.elapsed()
    );

    for row in 0..6 {
        println!("");
        for index in (row * 40) + 1..(row + 1) * 40 {
            let position = calc_register(cycles.clone(), index);
            let sprite = vec![position, position + 1, position + 2];
            print!(
                "{}",
                if sprite.contains(&(index % 40)) {
                    "#"
                } else {
                    "."
                }
            );
        }
    }
    println!("");
    println!("10b in {:?}", start.elapsed());
}

fn calc_register(cycles: Vec<(i32, i32)>, cycle: i32) -> i32 {
    return &cycles[0..(cycle as usize) - 1]
        .into_iter()
        .cloned()
        .map(|c| c.1)
        .sum::<i32>()
        + 1;
}
