fn main() {
    let lines = include_str!("../input.txt");

    let full_overlap_count = parse_input(lines)
        .into_iter()
        // range1 needs to be fully encompassed by range2, or vice versa
        // thus: a1 <= b1 <= a2 && a1 <= b2 <= a2
        //       but also the other way around:
        //       b1 <= a1 <= b2 && b1 <= a2 <= b2
        // resulting in: (a1 >= b1 && a2 <= b2) || (a1 <= b1 && a2 >= b2
        .filter(|assignments| {
            (assignments[0] >= assignments[2] && assignments[1] <= assignments[3])
                || (assignments[0] <= assignments[2] && assignments[1] >= assignments[3])
        })
        .count();
    println!("4a: {}", full_overlap_count);

    let partial_overlap_count = parse_input(lines)
        .into_iter()
        // char needs to be in both ranges
        // thus: a1 <= char <= a2 and
        //       b1 <= char <= b2
        // assume each range is well formed:
        //       a1 <= a2 and b1 <= b2
        // resulting in: a1 <= b2 and b1 <= a2
        .filter(|assignments| assignments[0] <= assignments[3] && assignments[2] <= assignments[1])
        .count();
    println!("4b: {}", partial_overlap_count);
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    // Create a vector containing vectors containing the high/low of both ranges
    return input
        .lines()
        .map(|l| {
            l.split([',', '-'])
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
}
