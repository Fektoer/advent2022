fn main() {
    let overlap_a = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .fold(0, |curr, (a, b)| {
            let range_a = a.split_once("-").unwrap();
            let range_b = b.split_once("-").unwrap();
            if (range_a.0.parse::<i32>().unwrap() >= range_b.0.parse::<i32>().unwrap()
                && range_a.1.parse::<i32>().unwrap() <= range_b.1.parse::<i32>().unwrap())
                || (range_a.0.parse::<i32>().unwrap() <= range_b.0.parse::<i32>().unwrap()
                    && range_a.1.parse::<i32>().unwrap() >= range_b.1.parse::<i32>().unwrap())
            {
                curr + 1
            } else {
                curr + 0
            }
        });
    println!("4a: {}", overlap_a);

    let overlap_a = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .fold(0, |curr, (a, b)| {
            let range_a = a.split_once("-").unwrap();
            let range_b = b.split_once("-").unwrap();

            // char needs to be in both ranges
            // assume: a1 <= char <= a2 and
            //         b1 <= char <= b2
            // assume each range is well formed:
            // a1 <= a2 and b1 <= b2
            // then: a1 <= b2 and b1 <= a2
            if range_a.0.parse::<i32>().unwrap() <= range_b.1.parse::<i32>().unwrap()
                && range_b.0.parse::<i32>().unwrap() <= range_a.1.parse::<i32>().unwrap()
            {
                curr + 1
            } else {
                curr + 0
            }
        });
    println!("4a: {}", overlap_a);
}
