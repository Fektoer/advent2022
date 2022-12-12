use std::collections::HashMap;
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Point(usize, usize);

fn main() {
    let mut values = HashMap::new();
    let mut visited = HashMap::new();
    let mut distance = HashMap::new();
    let mut start = Point(0, 0);
    let mut end = Point(0, 0);
    include_str!("../input.txt")
        .split("\n")
        .enumerate()
        .for_each(|line| {
            line.1.chars().enumerate().for_each(|char| {
                match char.1 {
                    'S' => start = Point(line.0, char.0),
                    'E' => end = Point(line.0, char.0),
                    _ => {}
                }
                values.insert(Point(line.0, char.0), char.1);
            })
        });

    for value in values {
        distance.insert(value, i32::MAX);
        visited.insert(value, false);
    }

    distance.insert(start, 0);
    println!("done");
}
