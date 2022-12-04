fn main() {
    let lines = include_str!("../input.txt");

    let overlap_a = parse_input(lines)
        .into_iter()
        .filter(|assignments| {
            (assignments[0] >= assignments[2] && assignments[1] <= assignments[3])
                || (assignments[0] <= assignments[2] && assignments[1] >= assignments[3])
        })
        .count();
    println!("4a: {}", overlap_a);

    let overlap_b = parse_input(lines)
        .into_iter()
        // char needs to be in both ranges
        // assume: a1 <= char <= a2 and
        //         b1 <= char <= b2
        // assume each range is well formed:
        // a1 <= a2 and b1 <= b2
        // then: a1 <= b2 and b1 <= a2
        .filter(|assignments| assignments[0] <= assignments[3] && assignments[2] <= assignments[1])
        .count();
    println!("4b: {}", overlap_b);
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
