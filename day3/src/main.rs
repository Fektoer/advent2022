fn calc_priority(value: char) -> i32 {
    if value.is_lowercase() {
        return value as i32 - 96;
    } else {
        return value as i32 - 38;
    }
}

fn main() {
    // 3A
    //=====================================================
    let sum_priorities = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| {
            let mut duplicate_char = '\0';
            for char in a.chars() {
                if b.contains(char) {
                    duplicate_char = char;
                }
            }
            return duplicate_char;
        })
        .map(|char| calc_priority(char))
        .sum::<i32>();

    println!("2a: {}", sum_priorities);

    // 3B
    //=====================================================
    let mut three_group = Vec::new();
    let mut groups = Vec::new();
    let rucksacks = include_str!("../input.txt").lines();
    for rucksack in rucksacks {
        three_group.push(rucksack);
        if three_group.len() == 3 {
            groups.push(three_group);
            three_group = Vec::new();
        }
    }

    let sum_priorities_group = groups
        .iter()
        .map(|group| {
            let mut duplicate_char = '\0';
            for char in group[0].chars() {
                if group[1].contains(char) && group[2].contains(char) {
                    duplicate_char = char;
                }
            }
            return duplicate_char;
        })
        .map(|char| calc_priority(char))
        .sum::<i32>();

    println!("2a: {}", sum_priorities_group);
}
