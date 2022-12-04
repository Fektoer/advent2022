fn main() {
    let overlap_a = include_str!("../temp.txt")
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

    let overlap_a = include_str!("../temp.txt")
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .fold(0, |curr, (a, b)| {
            let range_a = a.split_once("-").unwrap();
            for number in range_a.0.parse::<i32>().unwrap()..range_a.1.parse::<i32>().unwrap() {
                println!("{}", number);
            }
            1
        });
    println!("4a: {}", overlap_a);
}
